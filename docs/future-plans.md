# Future Plans & Client Implementation Notes

> Extracted from SPEC.md — content that is NOT yet covered in existing docs/.
> These are planned features, client-side implementation notes, and SDK design decisions
> that haven't been implemented yet but should guide future work.

---

## 1. Client-Side Architecture (Dart / TypeScript)

### Dart WS Client — RPC Pattern

```dart
class ChatClient {
  final WebSocket _ws;
  final EventBus events = EventBus();
  final _pending = <int, Completer<Uint8List>>{};  // seq → completer
  int _seq = 0;

  /// Fire-and-forget (seq=0): Typing, ReadReceipt, Ping
  void push(WsFrame frame) {
    frame.seq = 0;
    _ws.sink.add(frame.encode());
  }

  /// RPC: ожидает Ack или Error с совпадающим seq
  Future<Uint8List> rpc(WsFrame frame, {Duration timeout = const Duration(seconds: 30)}) {
    frame.seq = ++_seq;
    final completer = Completer<Uint8List>();
    _pending[frame.seq] = completer;
    _ws.sink.add(frame.encode());
    return completer.future.timeout(timeout, onTimeout: () {
      _pending.remove(frame.seq);
      throw TimeoutException('RPC timeout');
    });
  }

  void _onFrame(Uint8List bytes) {
    final frame = WsFrame.decode(bytes);
    if (frame.kind == FrameKind.ack || frame.kind == FrameKind.error) {
      _pending.remove(frame.seq)?.complete(frame.payload);
    } else {
      events.dispatch(frame);
    }
  }
}
```

### EventBus — Synchronous Typed Callbacks (Not Stream)

Stream has overhead per event. For high-frequency events — synchronous callbacks:

```dart
class EventBus {
  final _handlers = <FrameKind, List<Function>>{};

  void on<T>(FrameKind kind, void Function(T) handler) {
    _handlers.putIfAbsent(kind, () => []).add(handler);
  }

  void dispatch(WsFrame frame) {
    _handlers[frame.kind]?.forEach((h) => h(frame.decoded()));
  }
}
```

Stream can be exposed on top for `StreamBuilder` compatibility.

### Command Queue Throttling

При burst отправки (например reconnect + flush outbox) — throttle чтобы не заблокировать Dart event loop:

```dart
Future<void> _flush() async {
  _flushing = true;
  final sw = Stopwatch()..start();
  while (_queue.isNotEmpty) {
    _sendImmediate(_queue.removeFirst());
    if (sw.elapsedMilliseconds >= 8) {
      await Future.delayed(Duration.zero); // yield to event loop
      sw.reset();
    }
  }
  _flushing = false;
}
```

### TypeScript Client

Независимая TS библиотека — полноценный клиент для браузера и Node.js:
- WS транспорт напрямую к серверу (не через Rust)
- Flutter Web использует TS клиент через `dart:js_interop` + `extension type`
- Для браузера — `SharedWorker` для общего WS соединения между вкладками

```
npm (открытый):
└── @chat-sdk/client  ← WS клиент + codec + EventBus
```

---

## 2. Connection States

Клиентский код (Dart/TS) должен реализовать следующие состояния:

| State                             | Description                             | UI                             |
| --------------------------------- | --------------------------------------- | ------------------------------ |
| `Connecting`                      | Первое подключение                      | Full placeholder               |
| `Connected`                       | Handshake успешен                       | Chat                           |
| `Reconnecting(attempt, delay_ms)` | Переподключение после разрыва           | Banner, chat readable (cache)  |
| `Disconnected`                    | Потеря сети, timeout                    | Banner                         |
| `AuthError`                       | 2× Unauthorized, token_expired не обновился | Auth screen                |

```dart
chat.events.on<ConnectionEvent>((e) => switch (e) {
  Disconnected()                   => showBanner('Check internet'),
  Reconnecting(:final attempt)     => showBanner('Reconnecting... $attempt'),
  Connected()                      => hideBanner(),
  AuthError()                      => router.replace('/login'),
  _                                => null,
});
```

---

## 3. Authentication & Sessions

### Device ID

- Генерируется один раз (UUID v4), хранится в отдельном файле (не в chat_u{uid}.db)
- Переживает смену пользователя, разлогин, очистку БД
- Не секрет — просто стабильный идентификатор устройства

### Two Auth Modes for SDK

**Mode 1 — Token Exchange (dev/prototyping):**
Клиент передаёт API токен → сервер чата верифицирует через webhook на бэкенд разработчика.

**Mode 2 — Pre-issued Chat Token (production, recommended):**
Бэкенд разработчика запрашивает chat_token у сервера чата → передаёт фронтенду.
- Токен основного API никогда не покидает инфраструктуру разработчика
- JWT верификация на чат-сервере локальная
- Стандартная схема (Sendbird, Stream, Twilio)

Различие по `iss` claim или префиксу в JWT.

### tokenProvider Pattern

SDK принимает **функцию получения токена**, а не токен:

```dart
ChatSDK.initialize(
  auth: PreIssuedTokenAuth(
    tokenProvider: () => myBackend.getChatToken(),
  ),
);
```

### Reconnect Flow with Token Refresh

```
Disconnect
      │
      ▼
DisconnectCode.should_reconnect() == true?
      │ yes
      ▼
Token still fresh? ──yes──► reconnect with cached token
      │
      no
      │
      ▼
Dart: await tokenProvider()
      │
  ┌───┴────────────────┐
  token received        error
  ▼                    ▼
reconnect         emit AuthError → Dart decides
```

Exponential backoff: 1, 2, 4, 8, 16, 30, 30, 30...
При 2 подряд Unauthorized — `Event::AuthError`.

### Token Storage

- Access token — in memory (not persisted)
- Refresh token — in secure storage (keychain/keystore)
- Crate `keyring` abstracts all platforms

---

## 4. SQLite Client Cache (chat_client_rs — separate repo)

### Client-side SQLite Schema

```sql
CREATE TABLE messages (
    id           INTEGER PRIMARY KEY,
    chat_id      INTEGER NOT NULL,
    sender_id    INTEGER NOT NULL,
    ts           INTEGER NOT NULL,
    kind         INTEGER NOT NULL,
    status       INTEGER NOT NULL DEFAULT 0,
    content      TEXT NOT NULL,
    rich_content BLOB,
    extra        TEXT,          -- JSON, NULL if empty
    reply_to_id  INTEGER,
    updated_at   INTEGER NOT NULL
);

CREATE INDEX idx_messages_chat_id ON messages(chat_id, id);
CREATE INDEX idx_messages_updated ON messages(chat_id, updated_at);

CREATE TABLE outbox (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    idempotency_key BLOB NOT NULL,  -- 16 bytes UUID
    chat_id         INTEGER NOT NULL,
    kind            INTEGER NOT NULL,
    payload         BLOB NOT NULL,
    created_at      INTEGER NOT NULL,
    status          INTEGER NOT NULL DEFAULT 0, -- 0=pending, 1=sending, 2=failed_permanent
    attempts        INTEGER NOT NULL DEFAULT 0,
    last_attempt    INTEGER,
    error           TEXT
);

CREATE TABLE read_receipts (
    chat_id    INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
    ts         INTEGER NOT NULL,
    PRIMARY KEY (chat_id, user_id)
);

CREATE TABLE reactions (
    message_id INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    emoji      TEXT NOT NULL,
    ts         INTEGER NOT NULL,
    PRIMARY KEY (message_id, user_id, emoji)
);

CREATE TABLE chat_sync (
    chat_id        INTEGER PRIMARY KEY,
    last_update_ts INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE chats (
    id                     INTEGER PRIMARY KEY,
    kind                   INTEGER NOT NULL,
    title                  TEXT,
    avatar_url             TEXT,
    last_message_id        INTEGER,
    last_message_content   TEXT,
    last_message_sender_id INTEGER,
    last_message_at        INTEGER,
    unread_count           INTEGER NOT NULL DEFAULT 0,
    updated_at             INTEGER NOT NULL
);
```

### WAL Mode + Read/Write Separation

```sql
PRAGMA journal_mode = WAL;
```

- **Write connection**: one, via `tokio-rusqlite` (dedicated thread). All INSERT/UPDATE/DELETE.
- **Read pool**: `r2d2` + `r2d2_sqlite`, 2-4 connections. LoadWindow, Search, LoadChunk don't block writes or each other.

```rust
struct Database {
    writer: tokio_rusqlite::Connection,
    reader: r2d2::Pool<SqliteConnectionManager>,
}
```

### DB per User

```
/data/app/databases/
├── chat_u123456.db
├── chat_u789012.db
```

При смене uid:
1. Flush outbox текущего пользователя
2. Закрыть текущее соединение с БД
3. Закрыть WebSocket
4. Открыть БД нового пользователя
5. Event в Dart → сброс всего UI state

Old DBs (>30 days without login) — delete on startup.

### Migrations

```rust
const MIGRATIONS: &[Migration] = &[
    Migration::new(1, include_str!("migrations/001_initial.sql")),
    Migration::new(2, include_str!("migrations/002_reactions.sql")),
];
```

Version in `PRAGMA user_version`.

---

## 5. Outbox — Delivery Guarantee

### Message IDs — Only Server Generates

Клиент **никогда** не генерирует серверный ID сообщения. При offline отправке:
- Сообщение помещается в outbox (ещё не полноценное сообщение)
- В UI показывается как pending с локальным `outbox_id`
- После успешной доставки outbox entry удаляется
- Новое сообщение с серверным ID приходит через `Event::MessageNew`
- Dart заменяет pending элемент на реальное сообщение

### Lifecycle

```
User presses "send"
         │
         ▼
INSERT INTO outbox (status=pending)
Clear input field immediately
Show pending message (from outbox)
         │
         ▼
Outbox worker: status=sending, attempts++
Send WS frame SendMessage
         │
    ┌────┴─────────────────────────┐
    │ Ack received                 │ error
    ▼                              ▼
DELETE FROM outbox          transient? → backoff retry
                            permanent? → status=failed_permanent
                                         show error to user
         │
         ▼
Server broadcasts Event::MessageNew (with server id) via WS
INSERT INTO messages (server id, status=delivered)
Replace pending item with real message
```

### Sending Order

При переподключении outbox worker отправляет от старых к новым (`ORDER BY created_at ASC`). Если элемент упал с transient ошибкой — outbox worker **останавливается** и ждёт следующего тика (preserving order).

### Cancel from Outbox

User can delete message from outbox while status=pending. If already sent (has server id) — standard `DeleteMessage`.

### Retry Strategy

Exponential backoff: 1, 2, 4, 8, 16, 32, 60, 60, 60...

Permanent errors (no retry): Forbidden, ChatNotFound, MessageTooLarge.
Transient errors (retry): timeout, network error, 5xx.

### Recovery After Restart

At startup: `UPDATE outbox SET status=0 WHERE status=1` (rollback sending → pending).

---

## 6. Offline Command Strategies

| Command         | Method | persist | Offline behavior                              |
| --------------- | ------ | ------- | --------------------------------------------- |
| `SendMessage`   | push   | ✅       | → outbox (SQLite or in-memory)                |
| `EditMessage`   | push   | ✅       | → outbox                                      |
| `DeleteMessage` | push   | ✅       | → outbox                                      |
| `ReadReceipt`   | push   | ✅       | → outbox                                      |
| `Typing`        | push   | ❌       | drop (ephemeral)                              |
| `Subscribe`     | push   | ❌       | drop, execute on reconnect                    |
| `LoadMessages`  | rpc    | —       | LocalFallback (SQLite via chat_client_rs) or Fail |
| `Search`        | rpc    | —       | Fail (server search only, offline unavailable)|
| `GetPresence`   | rpc    | —       | Fail (immediate error)                        |

Without `chat_client_rs` — outbox stored in-memory, doesn't survive restart.

---

## 7. Message Windowing & Pagination (Client-Side)

### Window + Anchor Concept

В памяти всегда живёт окно ~100-150 сообщений с якорным сообщением посередине. Прыжок на 10k сообщений — просто смена окна.

```
[  ...  ] [50 before anchor] [ANCHOR] [50 after anchor] [  ...  ]
               ↑ load older                  ↑ load newer
```

### Chunking via Bit Shift (Dart)

ID сообщений строго автоинкрементны в разрезе чата. Сообщения не удаляются физически.

```dart
const int chunkShift = 6;               // 2^6 = 64 messages per chunk
const int chunkSize  = 1 << chunkShift; // 64
const int chunkMask  = chunkSize - 1;   // 0x3F

int chunkId(int messageId)    => messageId >> chunkShift;
int posInChunk(int messageId) => messageId & chunkMask;
int chunkStart(int chunkId)   => chunkId << chunkShift;
```

Sparse chunk map — `Map<int, MessagesChunk>`, not `List` (chunks loaded non-sequentially).

### Chunk Trust & Lazy Invalidation

Каждый чанк имеет `max_updated_at` (максимальный `updated_at` среди его сообщений).
При скролле к чанку клиент проверяет trust set и при необходимости запрашивает delta:
`LoadMessages(mode=Chunk, chunk_id, since_ts=max_updated_at)`.

Trust set (`Set<int>`) хранит chunk_id, подтверждённые сервером в текущей сессии.
Сбрасывается при disconnect и при смене чата. Подробнее: [client.md — Trust Model](client.md#trust-model--hashset-of-trusted-chunks).

`max_updated_at` для delta-запросов вычисляется из таблицы `messages` напрямую:
`SELECT MAX(updated_at) FROM messages WHERE chat_id = ? AND id >= chunk_start AND id < chunk_start + 64`.
Отдельная таблица не нужна — range scan по PK `(chat_id, id)` покрывает максимум 64 строки.

### Eviction

Chunks more than 3 away from current anchor — evicted from memory.

---

## 8. Custom Viewport & Scroll (Dart/Flutter)

### Why Not Standard ListView

- Считает общую `scrollExtent` — невозможно при неизвестном количестве сообщений
- `ScrollPosition` на абсолютных пикселях — при вставке сообщений выше якоря всё прыгает
- Нет нативной поддержки anchor semantics

### Custom Viewport Architecture

Anchor pixel coordinate is fixed, everything else is relative:

```dart
class ChatViewport extends RenderBox {
  int _anchorId;
  double _anchorOffset; // pixels from viewport top to anchor top

  final Map<int, double> _heightCache = {};
  double? _cachedConstraintWidth;

  @override
  void performLayout() {
    if (constraints.maxWidth != _cachedConstraintWidth) {
      _heightCache.clear(); // invalidate on width change
      _cachedConstraintWidth = constraints.maxWidth;
    }
    _layoutVisible();
  }
}
```

### Height Cache with Versioning

```dart
class MessageHeightCache {
  final Map<int, _CacheEntry> _cache = {};

  double? get(int messageId, int contentVersion) {
    final entry = _cache[messageId];
    if (entry == null || entry.contentVersion != contentVersion) return null;
    return entry.height;
  }

  void record(int messageId, int contentVersion, double height);
  void invalidateAll() => _cache.clear(); // on maxWidth change
}
```

### Relayout/Repaint Conditions

| Event                                  | Layout | Paint | Cache              |
| -------------------------------------- | ------ | ----- | ------------------ |
| New message in current chunk           | ✅      | ✅     | add                |
| Visible updated, height changed        | ✅      | ✅     | invalidate         |
| Visible updated, same height           | ❌      | ✅     | keep               |
| Invisible chunk updated                | ❌      | ❌     | invalidate         |
| Chunk not loaded                       | ❌      | ❌     | ❌                  |
| Viewport width changed                 | ✅      | ✅     | invalidate all     |

### Shimmer Placeholders for Heavy Layout

Layout queue lives outside RenderObject (layout and paint are synchronous):

```dart
class LayoutQueue {
  static final LayoutQueue instance = LayoutQueue._();
  final _pending = PriorityQueue<_LayoutRequest>(); // priority: closer to center = higher

  Future<void> _process() async {
    final sw = Stopwatch()..start();
    while (_pending.isNotEmpty) {
      if (sw.elapsedMilliseconds >= 8) {
        await Future.delayed(Duration.zero); // event loop renders shimmers
        sw.reset();
      }
      final request = _pending.removeFirst();
      request.onReady();
    }
  }
}
```

RenderObject without ready layout returns estimated size and draws shimmer. After yield — check if element is still on screen.

### Two Scroll Modes

1. **Normal** — smooth movement between nearby messages, viewport controls `_anchorOffset`
2. **Fast scroll** (scrollbar hold) — quick jump through entire history, loads chunks via rpc

```
Scrollbar position = anchorId / totalCount  (O(1))
```

Custom scrollbar shows relative position with loaded/unloaded chunk visualization.

---

## 9. SDK Interface Design

### Auth Provider

```dart
abstract class ChatAuthProvider {
  Future<String> getChatToken();
  Future<String> refreshChatToken(String expiredToken);
  String? get deviceId => null; // SDK generates if not provided
}
```

### SDK vs Developer Responsibilities

```
Developer:                      SDK (Dart/TS):
├── Token acquisition           ├── WebSocket connection
├── Refresh logic               ├── Reconnect / backoff
├── Auth UI                     ├── Seq numbering & RPC correlation
├── User profile                ├── Subscribe / SyncBatch handling
├── Push notifications          ├── Message pagination
├── Custom design               ├── Binary frame decode
└── Business logic              └── device_id (if not provided)

                                SDK (chat_client_rs, optional):
                                ├── SQLite local cache
                                ├── Outbox with persist through restart
                                └── Cursor-based load from SQLite
```

### ChatConfig

```dart
class ChatConfig {
  final String wsUrl;
  final ChatAuthProvider auth;
  final Duration connectTimeout;
  final Duration messageTimeout;
  /// Optional: path to SQLite for local cache via chat_client_rs
  /// If null — works without persistent cache (in-memory only)
  final String? localCachePath;
}
```

---

## 10. Theming & UI Customization (Flutter)

### Two Separate Mechanisms

```
ChatTheme (InheritedWidget)
├── ChatThemeData          ← styles, colors, padding
│   ├── MessagesThemeData  (bubbleColor, textStyle, padding...)
│   ├── InputThemeData     (backgroundColor, sendButtonColor...)
│   ├── ChatListThemeData  (tileBackground, avatarSize...)
│   └── ConnectionStateThemeData
│
└── ChatComponents         ← replace entire widgets
    ├── MessageBuilder?
    ├── MessageBubbleBuilder?
    ├── MessageContentBuilder? (text, image, file, deleted — separate)
    ├── InputBuilder?
    ├── ChatTileBuilder?
    ├── ConnectionStateBuilder? (connecting, reconnecting, authError)
    └── ...
```

```dart
// Simple — just colors
ChatTheme(
  data: ChatThemeData(
    messages: MessagesThemeData(outgoingBubbleColor: Colors.green),
  ),
  components: const ChatComponents(),
  child: ChatView(client: chat),
)

// Full component replacement
ChatTheme(
  data: ChatThemeData(),
  components: ChatComponents(
    messageBubbleBuilder: (context, msg, child) => MyBubble(child: child),
  ),
  child: ChatView(client: chat),
)
```

---

## 11. Message Model (Dart)

```dart
class Message {
  final int id;
  final int chatId;
  final int senderId;
  final String senderName;
  final String? senderAvatarUrl;
  final DateTime sentAt;
  final MessageStatus status;
  final MessageContent content;
  final MessageExtra extra;

  // Computed by viewport during layout
  final bool isOutgoing;
  final bool isFirstInGroup;
  final bool isLastInGroup;

  // Threads
  final int? replyToId;
  final MessagePreview? replyPreview;

  // Forwarding
  final ForwardInfo? forwardInfo;
}

enum MessageStatus { sending, delivered, read, deleted, failedPermanent }

sealed class MessageContent {}
class TextContent extends MessageContent { ... }
class ImageContent extends MessageContent { ... }
class FileContent extends MessageContent { ... }
class DeletedContent extends MessageContent { ... }
```

### MessageExtra

```dart
extension type const MessageExtra(Map<String, Object?> _json)
    implements Map<String, Object?> {
  const MessageExtra.empty() : _json = const {};
  factory MessageExtra.fromJson(String json) =>
    MessageExtra(jsonDecode(json) as Map<String, Object?>);
  String toJson() => jsonEncode(_json);
  bool get isEmpty => _json.isEmpty;
}
```

- В `Message` — always non-null: `MessageExtra.empty()` if no data
- SDK не интерпретирует extra — передаёт as-is through entire stack
- Server passes to other users as-is too
- Use cases: attachments, geolocation, bot buttons, inline mentions

---

## 12. Server Extensibility

### Level 1 — Webhooks (async)

```yaml
webhooks:
  url: https://api.example.com/chat/webhook
  secret: "hmac_secret"
  events: [message.created, message.updated, chat.created, ...]
```

Payload with HMAC-SHA256 signature, deduplication via event UUID.

### Level 2 — Interceptors (sync)

Modification/blocking of message before saving. Timeout 500ms, then Allow by default.

```rust
enum InterceptAction {
    Allow,
    Modify { content, extra },
    Reject { reason },
}
```

### Level 3 — Bot API

Bots — users with `is_bot: true` and token. REST API (`/bot/v1/sendMessage`, `/bot/v1/answerCallback`). Event delivery via webhook or long polling.

### Message Processing Pipeline

```
Incoming message
    ▼ Interceptor (sync, optional)
    ▼ Save to PostgreSQL
    ▼ Deliver to WebSocket clients
    ▼ Webhook (async)
    ▼ Bot API events
```

---

## 13. Tiers & Scaling

### Free — Single Node (self-hosted)

- Limit: ~50-100 concurrent users
- All in memory: `DashMap` for sessions, tokio `broadcast` for pub/sub
- Webhooks, Bot API, Interceptors — disabled

### Pro — Multi Node (Redis)

- Unlimited concurrent users
- Redis Pub/Sub for cross-node delivery
- Redis for presence (online status) with TTL + heartbeat
- Redis for distributed rate limiting

### Abstraction via Trait

```rust
#[async_trait]
trait ClusterBackend: Send + Sync {
    async fn broadcast(&self, chat_id: i64, msg: Arc<Message>) -> Result<()>;
    async fn set_online(&self, user_id: i64, device_id: Uuid) -> Result<()>;
    async fn is_online(&self, user_id: i64) -> Result<bool>;
    async fn increment_connections(&self) -> Result<u32>;
}

struct SingleNodeBackend { ... }  // Free
struct RedisBackend { ... }       // Pro
```

### Licensing

JWT key with offline verification (signature). No callback home needed.

---

## 14. Media Upload/Download

### Architecture

Media uploaded via **separate HTTP endpoint** (not WS — binary data is a poor fit for WS frames). Storage in S3-compatible backend.

### ServerCapabilities

```rust
bitflags::bitflags! {
    pub struct ServerCapabilities: u32 {
        const MEDIA_UPLOAD    = 1 << 0;
        const THUMBNAILS      = 1 << 1;
        const RICH_TEXT       = 1 << 2;
        const REACTIONS       = 1 << 3;
        const THREADS         = 1 << 4;
        const WEBHOOKS        = 1 << 5;
        const BOT_API         = 1 << 6;
    }
}
```

If `storage.enabled = false` — client hides attach button.

### Upload Flow

```
Client                              Server
  │                                    │
  │ POST /api/v1/upload               │
  │ Authorization: Bearer {token}     │
  │ Content-Type: multipart/form-data │
  │ { file, chat_id }            ───► │
  │                                    │  validate permissions
  │                                    │  validate size/type
  │                                    │  upload to S3
  │                                    │  generate thumbnail (if image)
  │ ◄── { file_id, url,              │
  │       thumbnail_url,              │
  │       size, mime_type }           │
  │                                    │
  │ WS: SendMessage                   │
  │ { chat_id, content: "",           │
  │   extra: { attachments: [         │
  │     { file_id, url, ... }         │
  │   ]}}                        ───► │
  │                                    │  save message
  │ ◄── Event::MessageNew             │
```

Upload and message send are separate steps. This allows:
- Progress display before sending
- Multiple file attachments
- Reusing uploaded files (forward)

### Chunked Upload for Large Files

For files > 5MB — chunked upload with resume:

```
POST /api/v1/upload/init     → { upload_id, chunk_size }
PUT  /api/v1/upload/{id}/{n} → upload chunk N
POST /api/v1/upload/{id}/complete → { file_id, url, ... }
```

On disconnect: `GET /api/v1/upload/{id}/status` → last successful chunk → resume.

---

## 15. Codegen (chat_protocol → Dart/TS)

### Approach: Rust Structs as Source of Truth

```rust
/// Source of truth in chat_protocol
#[derive(Protocol)]
#[protocol(kind = 0x10, direction = "client_to_server", needs_ack = true)]
pub struct SendMessage {
    pub chat_id: i64,
    pub idempotency_key: Uuid,
    #[protocol(max_len = 4096)]
    pub content: String,
    pub rich_content: Option<Vec<u8>>,
    #[protocol(max_size = 4096)]
    pub extra: Option<serde_json::Value>,
    pub reply_to_id: Option<i64>,
}
```

```bash
cargo xtask codegen              # generates Dart + TS
cargo xtask codegen --lang dart  # Dart only
cargo xtask codegen --lang ts    # TypeScript only
```

### Type Mapping

| Rust                | Dart                   | TypeScript                | Wire (bytes)         |
| ------------------- | ---------------------- | ------------------------- | -------------------- |
| `i64`               | `int`                  | `bigint`                  | 8 bytes LE           |
| `i32` / `u32`       | `int`                  | `number`                  | 4 bytes LE           |
| `u8`                | `int`                  | `number`                  | 1 byte               |
| `u16`               | `int`                  | `number`                  | 2 bytes LE           |
| `bool`              | `bool`                 | `boolean`                 | 1 byte               |
| `String`            | `String`               | `string`                  | u32 len + UTF-8      |
| `Vec<u8>`           | `Uint8List`            | `Uint8Array`              | u32 len + bytes      |
| `Option<T>`         | `T?`                   | `T \| undefined`          | u8 flag + T          |
| `Uuid`              | `String`               | `string`                  | 16 bytes             |
| `serde_json::Value` | `Map<String, Object?>` | `Record<string, unknown>` | u32 len + JSON UTF-8 |

> **Note**: Derive macro `Protocol` and xtask codegen — second phase. Initially Rust types and Dart classes are written manually. Codegen is added when the type count makes manual sync burdensome.

---

## 16. Chat List Pagination (Client-Side)

### Cursor-based by `last_message_at`

```dart
// First page
final chats = await rpc(LoadChats(limit: 30));

// Next page — cursor = last_message_at of last chat
final more = await rpc(LoadChats(limit: 30, before: chats.last.lastMessageAt));
```

### Real-time Updates

Chats with changes arrive as Events via WebSocket:
- `Event::MessageNew` — chat moves to top
- `Event::ChatCreated` — new chat appears
- `Event::ChatUpdated` — title/avatar change
- `Event::UnreadCountChanged` — counter update

Client **does not re-fetch** entire list — just updates specific chat in UI by event.

At app launch — show from local SQLite cache instantly (if using `chat_client_rs`), sync with server in parallel.

---

## 17. Features Outside SDK Scope

| Feature         | Reason                         |
| --------------- | ------------------------------ |
| Voice / Video   | Separate domain (WebRTC)       |
| Stickers / GIF  | Content-specific, via extra    |
| E2EE            | Changes entire architecture    |
| Link previews   | Via extra + developer's service|

---

## 18. SDK-Specific Features (Planned)

- **MockChatClient**: for testing UI without server
- **Localization**: standard Flutter mechanism, overridable
- **Analytics**: abstract `ChatAnalytics` (no-op by default)

---

## 19. Distribution

### Dart/Flutter Packages

```
pub.dev (open):
├── chat_platform_interface   ← abstract types, 0 native deps
└── chat_flutter              ← widgets

GitHub Releases / S3 (chat_client_rs binaries — SEPARATE REPO):
└── v1.0.2/
    ├── libchat-android-arm64.so
    ├── libchat-android-armv7.so
    ├── libchat-android-x64.so
    ├── libchat-ios.a
    ├── libchat-macos.dylib
    ├── chat-windows.dll
    └── libchat-linux.so
```

### Native Assets Hook

`hook/build.dart` in `chat_core` package downloads pre-built `chat_client_rs` binary during `flutter build` (not during `pub get`). SHA256 checksum verification. Cache in `.dart_tool/chat_blobs/`.

Binaries needed only if using local SQLite cache. Without `chat_client_rs` — app works without native deps.

### Private pub Server Alternative

Protocol: `GET /api/packages/{name}` (JSON), `GET .../versions/{version}.tar.gz` (archive).

Note: pub.dev doesn't allow dependencies from packages on other hosts.

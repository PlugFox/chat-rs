# Chat SDK — Спецификация и архитектурное руководство

> Документ является одновременно спецификацией, идеей и отправной точкой для реализации.
> Основан на детальном проектировании архитектуры чат-платформы.
>
> **Архитектурное решение (актуально):** Dart и TypeScript клиенты подключаются к серверу напрямую через WebSocket. FFI прослойка для транспорта удалена. Rust отвечает только за сервер (`chat_server`) и wire-протокол (`chat_protocol`). Локальный кэш/SQLite — в отдельном репозитории `chat_client_rs`.
>
> **⚠️ OUTDATED:** Этот документ использует `i64` для всех ID (user_id, chat_id, message_id и т.д.).
> Актуальный wire-протокол использует `u32` — см. [docs/protocol.md](docs/protocol.md) и [docs/codec.md](docs/codec.md).
> При расхождении SPEC.md и docs/ — docs/ является источником истины.

---

## 1. Обзор проекта

**Chat SDK** — кроссплатформенная платформа для чата. Сервер на Rust (axum + PostgreSQL). Клиенты (Dart/Flutter, TypeScript) подключаются к серверу **напрямую через WebSocket**, используя бинарный протокол из `chat_protocol`. Локальный кэш — отдельная Rust-библиотека (`chat_client_rs`) в отдельном репозитории. Проектируется как SDK/платформа для сторонних приложений.

### Цели

- Высокопроизводительный сервер: WebSocket + PostgreSQL, axum
- Единый wire-протокол для всех клиентских платформ
- Эффективная работа с большими чатами (100k+ сообщений)
- Offline-first: outbox в SQLite пережиёт перезапуск приложения (реализовано в `chat_client_rs`)
- Расширяемость: кастомные метаданные, темы, компоненты, Bot API, webhooks

### Позиционирование

SDK для сторонних разработчиков (аналог Sendbird, Stream Chat), с возможностью self-hosted и SaaS.

---

## 2. Технологический стек

### Rust (сервер + протокол, этот репозиторий)

| Компонент         | Crate                          | Назначение                                         |
| ----------------- | ------------------------------ | -------------------------------------------------- |
| Async runtime     | `tokio` (features = `full`)    | Единственный runtime                               |
| WebSocket         | `tokio-tungstenite` + `rustls` | Без OpenSSL, работает одинаково на всех платформах |
| Сериализация      | `serde` + `serde_json`         | Для JSON полей (extra, конфиг)                     |
| Ошибки (protocol) | `thiserror`                    | Typed ошибки в `chat_protocol`                     |
| Ошибки (app)      | `anyhow`                       | Application-level ошибки в `chat_server`           |
| Bitflags          | `bitflags`                     | Стили rich text, права доступа                     |
| Web framework     | `axum` (features = `ws`)       | HTTP + WebSocket                                   |
| PostgreSQL        | `sqlx` (features = `postgres`) | Async, compile-time checked queries                |
| JWT               | `jsonwebtoken`                 | Верификация токенов                                |
| Rate limiting     | `governor`                     | GCRA, keyed, lock-free                             |

### Rust (клиентский кэш, отдельный репозиторий `chat_client_rs`)

| Компонент    | Crate                             | Назначение                                        |
| ------------ | --------------------------------- | ------------------------------------------------- |
| SQLite       | `rusqlite` (features = `bundled`) | Статически слинкованный                           |
| SQLite async | `tokio-rusqlite`                  | Write-соединение (выделенный поток)               |
| SQLite pool  | `r2d2` + `r2d2_sqlite`            | Read-пул для concurrent запросов                  |
| Миграции     | `rusqlite_migration`              | Версионирование схемы через `PRAGMA user_version` |

### Dart / Flutter

| Компонент       | Подход                                                               |
| --------------- | -------------------------------------------------------------------- |
| WS транспорт    | `web_socket_channel` или нативный WS — прямое подключение к серверу  |
| Протокол        | Бинарный decode `chat_protocol` фреймов через `ByteData` (zero-copy) |
| Локальный кэш   | `chat_client_rs` via `dart:ffi` (опционально)                        |
| Темизация       | `InheritedWidget` (`ChatTheme`)                                      |
| Кастомизация UI | `ChatComponents` — типизированные builder callbacks                  |

### TypeScript

| Компонент     | Подход                                                                              |
| ------------- | ----------------------------------------------------------------------------------- |
| WS транспорт  | `WebSocket` API (browser) / `ws` (Node.js) — прямое подключение к серверу           |
| Протокол      | Бинарный encode/decode `chat_protocol` фреймов (генерируется через `xtask codegen`) |
| SharedWorker  | Общее WS соединение между вкладками браузера                                        |
| Локальный кэш | IndexedDB (web) или без персистентности                                             |

### Почему `rustls` вместо OpenSSL

- Нет бинарных зависимостей, проще для app store review
- Публичный security audit, можно сослаться
- На всех нативных платформах работает идентично
- При публикации в App Store / Google Play нужно стандартно декларировать использование шифрования (Category (b) EAR exemption — стандартный TLS)

---

## 3. Архитектура

### Общая схема

```
Flutter (Dart)                  TypeScript / React
    │                                  │
    │ WebSocket (напрямую)             │ WebSocket (напрямую)
    │                                  │
    └─────────────────┬────────────────┘
                      │
                 chat_server
             (axum + PostgreSQL)
```

### Опциональный локальный кэш

Dart/Flutter может подключать `chat_client_rs` (отдельный репозиторий) через FFI для локального SQLite-кэша и outbox. TypeScript на web хранит кэш в IndexedDB или работает без персистентности.

```
Flutter (Dart)
    ├── WebSocket → chat_server    (транспорт: события, команды)
    └── FFI → chat_client_rs       (локальный кэш, outbox, SQLite)
              [отдельный репозиторий]
```

### Поток данных — Dart клиент

```
WS Event (server → client)
    │
    ▼
Dart EventBus
    │
    ├── ChatWindowState  (обновляет chunks)
    ├── ChatListState    (список чатов, unread)
    └── UserState        (онлайн статусы)
         │
         ▼
        UI (RenderObjects)

Dart UI
    │
    │  WS Command (push/rpc)
    ▼
chat_server (обрабатывает, сохраняет в PostgreSQL, рассылает Event'ы)
```

Dart никогда не мутирует состояние напрямую — только через WS команды. Сервер отвечает Event'ами.

### Порядок реализации crate'ов (этот репозиторий)

1. **`chat_protocol`** — первый crate. Все типы фреймов, codec, error codes. Контракт между клиентом и сервером, определяет 80% дальнейшей работы.
2. **`chat_server`** — сервер

### Workspace структура (этот репозиторий)

```
chat-rs/
├── crates/
│   ├── chat_protocol/     ← ПЕРВЫЙ crate: типы фреймов, codec, error codes
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── frames.rs  ← структуры фреймов (FrameKind, Hello, Welcome...)
│   │       ├── codec.rs   ← сериализация/десериализация
│   │       └── error.rs   ← ErrorCode enum, slugs
│   └── chat_server/       ← Сервер (axum + PostgreSQL), см. раздел 30
│       ├── src/           ← axum handlers, WS, services, DB
│       ├── migrations/    ← PostgreSQL миграции (sqlx)
│       └── config.example.toml
└── xtask/                 ← автоматизация сборки

chat_client_rs/            ← ОТДЕЛЬНЫЙ РЕПОЗИТОРИЙ
├── src/
│   ├── lib.rs             ← FFI entrypoint
│   ├── ffi.rs             ← FFI функции (cache_open/query/destroy)
│   ├── db.rs              ← SQLite (tokio-rusqlite writer + r2d2 read pool)
│   └── outbox.rs          ← Очередь отправки
└── migrations/            ← SQLite миграции (rusqlite_migration)

flutter/                   ← Dart/Flutter SDK (отдельный репозиторий или monorepo)
├── chat_platform_interface/  ← абстрактные типы (pub.dev)
├── chat_core/                ← WS транспорт + codec + опционально FFI → chat_client_rs
└── chat_flutter/             ← виджеты, тема (pub.dev)
```

---

## 4. Подключение клиентов (Dart / TypeScript → сервер)

### Принципы

- Клиенты (Dart, TypeScript) подключаются к серверу **напрямую через WebSocket** — нет Rust FFI прослойки для транспорта
- Бинарный протокол — общий для всех платформ, определён в `chat_protocol`
- Dart декодирует фреймы через `ByteData` (zero-copy read), TypeScript через `DataView`
- Батчинг: сервер может группировать до 20 событий в один WS фрейм (SyncBatch)

### Connection Lifecycle

1. Открыть WS соединение к `wss://<host>/ws`
2. Отправить **Hello** фрейм (protocol_version, sdk_version, platform, JWT token, device_id)
3. Получить **Welcome** (session_id, user_id, ServerLimits, ServerCapabilities)
4. Подписаться на чаты через `Subscribe` фреймы
5. Keepalive: Ping/Pong по интервалу из ServerLimits

### Seq numbering (клиентская сторона)

Клиент ведёт монотонный `seq: u32` счётчик. Каждому исходящему фрейму присваивается следующий seq. RPC-запросы (LoadChats, Search, GetChatInfo) ожидают `Ack(seq)` или `Error(seq)`.

```dart
class WsClient {
  int _seq = 0;
  final _pending = <int, Completer<Uint8List>>{};  // seq → completer

  int nextSeq() => ++_seq;

  Future<Uint8List> rpc(WsFrame frame) {
    final seq = nextSeq();
    frame.seq = seq;
    final completer = Completer<Uint8List>();
    _pending[seq] = completer;
    _ws.sink.add(frame.encode());
    return completer.future.timeout(const Duration(seconds: 30));
  }

  void _onFrame(WsFrame frame) {
    if (frame.kind == FrameKind.ack || frame.kind == FrameKind.error) {
      _pending.remove(frame.seq)?.complete(frame.payload);
    } else {
      _eventBus.dispatch(frame);
    }
  }
}
```

### Reconnection (клиентская сторона)

```
Разрыв соединения
    │
    ▼
DisconnectCode.should_reconnect() == true?
    ├── да → exponential backoff (1, 2, 4, 8, 16, 30, 30...)
    │         → Hello с fresh token
    │         → Subscribe на все активные чаты с last_update_ts
    └── нет → показать UI (TokenExpired, Forbidden, etc.)
```

---

## 5. Бинарный протокол (WS фреймы, клиентская сторона)

### WS Frame Header

Всё little-endian (нативный для ARM/x86).

```
┌──────────┬──────────┬───────────┬──────────────────┐
│ ver: u8  │ kind: u8 │  seq: u32 │ payload: bytes   │
└──────────┴──────────┴───────────┴──────────────────┘
```

6 байт заголовок, затем payload зависящий от `kind`.

### Батч сообщений (SyncBatch / LoadMessages response)

```
MessageBatch:
┌──────────────┬──────────────────────────────────────┐
│ count: u32   │ messages[count]                      │
└──────────────┴──────────────────────────────────────┘

Message (фиксированный заголовок 37 байт + variable):
┌─────────┬──────────┬───────────┬─────────┬────────┬─────────────┬──────────────────┐
│ id: i64 │ chat: i64│ sender:i64│ ts: i64 │kind: u8│ content_len │ content (UTF-8)  │
│  8 bytes│  8 bytes │  8 bytes  │  8 bytes│  1 byte│   u32 4bytes│  N bytes         │
└─────────┴──────────┴───────────┴─────────┴────────┴─────────────┴──────────────────┘
```

Затем: `rich_len: u32` + rich blob, `extra_len: u32` + extra JSON. Если len = 0 — данных нет, аллокаций нет.

### Декодер в Dart

```dart
class MessageBatchReader {
  final ByteData _data;
  int _offset = 0;

  MessageBatchReader(Uint8List bytes)
      : _data = ByteData.sublistView(bytes); // zero-copy

  List<Message> readAll() {
    final count = _readU32();
    return List<Message>.generate(count, (_) => _readMessage());
  }
  // _readI64, _readU32, _readU8, _readString — через ByteData
}
```

### Серверный батчинг событий

Сервер группирует до 20 событий или 16ms — что наступит раньше — в один `SyncBatch` фрейм. Клиент декодирует пачку сразу, уменьшая количество async wakeup'ов.

---

## 6. RPC и Event система (Dart сторона)

### ChatClient — WS-based

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

  /// RPC: ждёт Ack или Error с совпадающим seq
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

### EventBus — синхронные typed callbacks (не Stream)

Stream имеет overhead на каждый event. Для высокочастотных событий — синхронные коллбэки:

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

Stream можно выставить наружу поверх если нужна совместимость с `StreamBuilder`.

### Очередь команд (throttling event loop)

При burst отправки (например reconnect + flush outbox) — throttle чтобы не заблокировать Dart event loop:

```dart
Future<void> _flush() async {
  _flushing = true;
  final sw = Stopwatch()..start();
  while (_queue.isNotEmpty) {
    _sendImmediate(_queue.removeFirst());
    if (sw.elapsedMilliseconds >= 8) {
      await Future.delayed(Duration.zero); // уступаем event loop
      sw.reset();
    }
  }
  _flushing = false;
}
```

---

## 7. SQLite схема (chat_client_rs — отдельный репозиторий)

> Схема реализована в `chat_client_rs`. В этом разделе — спецификация для понимания модели данных.

### Сообщения

```sql
CREATE TABLE messages (
    id           INTEGER PRIMARY KEY,  -- rowid, глобально монотонный в разрезе чата
    chat_id      INTEGER NOT NULL,
    sender_id    INTEGER NOT NULL,
    ts           INTEGER NOT NULL,
    kind         INTEGER NOT NULL,
    status       INTEGER NOT NULL DEFAULT 0,
    content      TEXT NOT NULL,         -- plain text
    rich_content BLOB,                  -- NULL если нет форматирования
    extra        TEXT,                  -- JSON, NULL если пусто
    reply_to_id  INTEGER,              -- id сообщения-родителя (threads)
    updated_at   INTEGER NOT NULL
);

CREATE INDEX idx_messages_chat_id ON messages(chat_id, id);
CREATE INDEX idx_messages_updated ON messages(chat_id, updated_at);
```

### Поиск — только серверный (PostgreSQL FTS)

Клиентский FTS5 убран: увеличивает размер БД на диске, при этом у клиента недостаточно данных для полноценного поиска (не вся история синхронизирована).

Поиск всегда через серверный rpc → PostgreSQL `tsvector/tsquery`:

```sql
-- На сервере (PostgreSQL)
ALTER TABLE messages ADD COLUMN search_vector tsvector
    GENERATED ALWAYS AS (to_tsvector('simple', content)) STORED;

CREATE INDEX idx_messages_search ON messages USING GIN (search_vector);
```

```rust
// Серверный handler
async fn search(pool: &PgPool, chat_id: i64, query: &str, limit: i32) -> Vec<SearchResult> {
    sqlx::query_as!(SearchResult,
        r#"SELECT id, sender_id, content, ts,
           ts_headline('simple', content, websearch_to_tsquery('simple', $2)) as snippet
           FROM messages
           WHERE chat_id = $1 AND search_vector @@ websearch_to_tsquery('simple', $2)
           ORDER BY ts DESC LIMIT $3"#,
        chat_id, query, limit
    ).fetch_all(pool).await?
}
```

На клиенте `Search` — это rpc команда на сервер, при оффлайн — `Fail` (не LocalFallback как раньше).

### Очередь отправки (Outbox)

```sql
CREATE TABLE outbox (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    chat_id     INTEGER NOT NULL,
    kind        INTEGER NOT NULL,
    payload     BLOB NOT NULL,
    created_at  INTEGER NOT NULL,
    status      INTEGER NOT NULL DEFAULT 0,  -- 0=pending, 1=sending, 2=failed_permanent
    attempts    INTEGER NOT NULL DEFAULT 0,
    last_attempt INTEGER,
    error       TEXT
);
```

### Read Receipts

```sql
CREATE TABLE read_receipts (
    chat_id    INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    message_id INTEGER NOT NULL,
    ts         INTEGER NOT NULL,
    PRIMARY KEY (chat_id, user_id)
);
```

### Реакции

```sql
CREATE TABLE reactions (
    message_id INTEGER NOT NULL,
    user_id    INTEGER NOT NULL,
    emoji      TEXT NOT NULL,
    ts         INTEGER NOT NULL,
    PRIMARY KEY (message_id, user_id, emoji)
);
```

### Синхронизация

```sql
CREATE TABLE chat_sync (
    chat_id        INTEGER PRIMARY KEY,
    last_update_ts INTEGER NOT NULL DEFAULT 0
);
```

### WAL режим + Read/Write разделение

```sql
PRAGMA journal_mode = WAL;
```

SQLite в WAL режиме допускает concurrent reads при single writer. Архитектура:

- **Write-соединение**: одно, через `tokio-rusqlite` (выделенный поток). Все INSERT/UPDATE/DELETE.
- **Read-пул**: `r2d2` + `r2d2_sqlite`, 2-4 соединения. LoadWindow, Search, LoadChunk — не блокируют write и друг друга.

```rust
struct Database {
    writer: tokio_rusqlite::Connection,  // единственный writer
    reader: r2d2::Pool<SqliteConnectionManager>,  // read pool
}

impl Database {
    // Read операции — через пул, в spawn_blocking
    async fn load_window(&self, ...) -> Result<MessageWindow> {
        let pool = self.reader.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get()?;
            // SELECT ...
        }).await?
    }

    // Write операции — через единственный writer
    async fn insert_message(&self, msg: &Message) -> Result<()> {
        self.writer.call(|conn| {
            conn.execute("INSERT INTO messages ...", params![...])?;
            Ok(())
        }).await
    }
}
```

Это важно: при множественных concurrent rpc (LoadWindow + Search + outbox worker) read-запросы не сериализуются на одном потоке.

### Миграции

```rust
const MIGRATIONS: &[Migration] = &[
    Migration::new(1, include_str!("migrations/001_initial.sql")),
    Migration::new(2, include_str!("migrations/002_reactions.sql")),
    // ...
];
```

Версия схемы в `PRAGMA user_version`.

---

## 8. Пагинация и окно сообщений

### Ключевая идея: Window + Anchor

В памяти всегда живёт окно ~100-150 сообщений с якорным сообщением посередине. Прыжок на 10k сообщений — просто смена окна.

```
[  ...  ] [50 before anchor] [ANCHOR] [50 after anchor] [  ...  ]
               ↑ load older                  ↑ load newer
```

### Cursor-based запросы (не OFFSET!)

```sql
-- load_older: O(log n) вместо O(n)
SELECT * FROM messages WHERE chat_id = ? AND id < ? ORDER BY id DESC LIMIT ?

-- load_newer
SELECT * FROM messages WHERE chat_id = ? AND id > ? ORDER BY id ASC LIMIT ?
```

### API (реализован в chat_client_rs, вызывается через FFI или напрямую из Rust тестов)

```rust
pub struct MessageWindow {
    pub messages: Vec<Message>,
    pub anchor_id: i64,
    pub has_older: bool,
    pub has_newer: bool,
}

fn load_window(chat_id: i64, anchor_id: i64, before: usize, after: usize) -> MessageWindow;
fn load_older(chat_id: i64, oldest_id: i64, count: usize) -> Vec<Message>;
fn load_newer(chat_id: i64, newest_id: i64, count: usize) -> Vec<Message>;
```

Search — только через WS RPC (0x17 `Search`), результаты из PostgreSQL FTS.

### Chunking через битовый сдвиг (Dart сторона)

ID сообщений строго автоинкрементны в разрезе чата, сообщения не удаляются физически (меняют статус).

```dart
const int chunkShift = 6;               // 2^6 = 64 сообщения на чанк
const int chunkSize  = 1 << chunkShift; // 64
const int chunkMask  = chunkSize - 1;   // 0x3F

int chunkId(int messageId)    => messageId >> chunkShift;
int posInChunk(int messageId) => messageId & chunkMask;
int chunkStart(int chunkId)   => chunkId << chunkShift;
```

Разреженный массив чанков — `Map<int, MessagesChunk>`, не `List` (чанки загружаются непоследовательно).

### Eviction

Чанки дальше 3 от текущего якоря — удаляются из памяти.

---

## 9. Кастомный Viewport и скролл

### Почему не стандартный ListView

- Считает общую `scrollExtent` — невозможно при неизвестном количестве сообщений
- `ScrollPosition` на абсолютных пикселях — при вставке сообщений выше якоря всё прыгает
- Нет нативной поддержки anchor semantics

### Архитектура кастомного Viewport

Ключевая идея — пиксельная координата якоря фиксирована, всё остальное считается относительно:

```dart
class ChatViewport extends RenderBox {
  int _anchorId;
  double _anchorOffset; // пикселей от верха вьюпорта до верха anchor

  final Map<int, double> _heightCache = {};
  double? _cachedConstraintWidth;

  @override
  void performLayout() {
    if (constraints.maxWidth != _cachedConstraintWidth) {
      _heightCache.clear(); // инвалидация при смене ширины
      _cachedConstraintWidth = constraints.maxWidth;
    }
    _layoutVisible();
  }
}
```

### Height Cache с версионированием

```dart
class MessageHeightCache {
  final Map<int, _CacheEntry> _cache = {};

  double? get(int messageId, int contentVersion) {
    final entry = _cache[messageId];
    if (entry == null || entry.contentVersion != contentVersion) return null;
    return entry.height;
  }

  void record(int messageId, int contentVersion, double height);
  void invalidateAll() => _cache.clear(); // при изменении maxWidth
}
```

### Estimation до реального layout

```dart
double estimate(Message msg) => switch (msg.status) {
  MessageStatus.deleted => 40.0,  // фиксированная высота
  MessageStatus.normal  => _estimateByContent(msg),
};
```

Для plain text: `(msg.content.length / charsPerLine).ceil() * lineHeight + padding`.

### Условия relayout/repaint

| Событие                                | Layout | Paint | Cache              |
| -------------------------------------- | ------ | ----- | ------------------ |
| Новое сообщение в текущем чанке        | ✅      | ✅     | добавить           |
| Обновление видимого, высота изменилась | ✅      | ✅     | инвалидировать     |
| Обновление видимого, высота та же      | ❌      | ✅     | не трогать         |
| Обновление невидимого чанка            | ❌      | ❌     | инвалидировать     |
| Чанк не загружен                       | ❌      | ❌     | ❌                  |
| Смена ширины viewport                  | ✅      | ✅     | инвалидировать всё |

### Shimmer-плейсхолдеры для тяжёлого layout

Очередь layout живёт снаружи RenderObject (layout и paint синхронны, async внутри невозможен):

```dart
class LayoutQueue {
  static final LayoutQueue instance = LayoutQueue._();
  final _pending = PriorityQueue<_LayoutRequest>(); // приоритет: ближе к центру = выше

  Future<void> _process() async {
    final sw = Stopwatch()..start();
    while (_pending.isNotEmpty) {
      if (sw.elapsedMilliseconds >= 8) {
        await Future.delayed(Duration.zero); // event loop выводит кадр с шиммерами
        sw.reset();
      }
      final request = _pending.removeFirst();
      request.onReady();
    }
  }
}
```

RenderObject без готового layout отдаёт estimated размер и рисует shimmer. После yield'а — проверяем, на экране ли ещё элемент.

### Два режима скролла

1. **Обычный** — плавное движение между ближайшими сообщениями, viewport управляет `_anchorOffset`
2. **Fast scroll** (зажатие скроллбара) — быстрый переход по всей истории, запрос чанков через rpc

```
Скроллбар position = anchorId / totalCount  (O(1))
```

Кастомный scrollbar показывает relative position (не абсолютный), с визуализацией загруженных/незагруженных чанков.

---

## 10. Rich Text

### Хранение

- `content TEXT` — plain text (поиск серверный через PostgreSQL FTS)
- `rich_content BLOB` — NULL если нет форматирования (большинство сообщений)

### Формат BLOB — spans с оффсетами (не дублирование текста)

```
RichContent:
┌───────────┬──────────────────────┐
│ count: u16│ spans[count]         │
└───────────┴──────────────────────┘

Span (10 байт фиксированных + опциональная meta):
┌────────────┬──────────┬──────────┬──────────────────────┐
│ start: u32 │ end: u32 │ style:u16│ meta (если есть)     │
└────────────┴──────────┴──────────┴──────────────────────┘
```

`start/end` — байтовые оффсеты в plain text строке.

### Стили (bitflags)

```rust
bitflags::bitflags! {
    pub struct Style: u16 {
        const BOLD    = 0b0000_0001;
        const ITALIC  = 0b0000_0010;
        const CODE    = 0b0000_0100;
        const STRIKE  = 0b0000_1000;
        const SPOILER = 0b0001_0000;
        const LINK    = 0b0010_0000;  // meta = url_len: u16 + url: UTF-8
        const MENTION = 0b0100_0000;  // meta = user_id: i64
        const COLOR   = 0b1000_0000;  // meta = rgba: u32
    }
}
```

Для простых стилей (bold/italic/code) спан ровно 10 байт.

### Наложение спанов

Spans могут перекрываться (например **жирный курсив**). Нормализация — только на Dart стороне при рендере: собираем breakpoints, для каждого сегмента OR всех перекрывающих спанов.

В BLOB хранятся overlapping ranges без нормализации — компактнее, проще редактирование.

---

## 11. MessageExtra — кастомные метаданные

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

- В `Message` всегда non-null: `MessageExtra.empty()` если нет данных
- В SQLite: `extra TEXT` (JSON, NULL если пусто)
- В wire протоколе: `extra_len: u32` (0 = нет, никаких аллокаций)
- SDK не интерпретирует extra — передаёт as-is через весь стек
- Сервер передаёт другим пользователям тоже as-is

Примеры использования: вложения, геолокация, кнопки (боты), упоминания для инлайна.

### Ограничения размеров (не захардкожены, из конфига сервера)

Если пользователю нужно больше — он кладёт в extra ID и получает данные со своего бэкенда.

Лимиты передаются клиенту при создании сессии в `Welcome` фрейме (см. раздел 28).

---

## 12. Модель данных Message

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

  // Вычисляется viewport'ом при layout
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

Билдеры получают `Message` напрямую — без промежуточного MessageBuildContext (нет x2 объектов).

---

## 13. Outbox — гарантия доставки

> Outbox реализован в `chat_client_rs` (отдельный репозиторий). Здесь описана спецификация поведения, которую должны реализовать все клиентские платформы.

### ID сообщений — только сервер генерирует

Клиент **никогда** не генерирует серверный ID сообщения. При offline отправке:
- Сообщение помещается в outbox (это ещё не полноценное сообщение)
- В UI показывается как pending с локальным `outbox_id`
- После успешной доставки outbox entry удаляется
- Новое сообщение с серверным ID приходит через `Event::MessageNew` по WebSocket
- Dart заменяет pending элемент на реальное сообщение

### Жизненный цикл сообщения

```
Пользователь нажал "отправить"
         │
         ▼
INSERT INTO outbox (status=pending)
Поле ввода очищается сразу
Показать pending сообщение (из outbox)
         │
         ▼
Outbox worker: status=sending, attempts++
Отправить WS фрейм SendMessage
         │
    ┌────┴─────────────────────────┐
    │ Ack получен                  │ ошибка
    ▼                              ▼
DELETE FROM outbox          transient? → backoff retry
                            permanent? → status=failed_permanent
                                         показать ошибку пользователю
         │
         ▼
Сервер рассылает Event::MessageNew (с серверным id) по WS
INSERT INTO messages (серверный id, status=delivered)
Заменить pending элемент на реальное сообщение
```

### Порядок отправки из outbox

При переподключении outbox worker отправляет от старых к новым (`ORDER BY created_at ASC`). Если элемент упал с transient ошибкой — outbox worker **останавливается** и ждёт следующего тика (сохраняя порядок).

### Отмена из outbox

Пользователь может удалить сообщение из outbox пока оно в статусе pending. Если уже отправлено (имеет серверный id) — используется стандартный `DeleteMessage`.

Edit и Delete для outbox сообщений — только удаление из outbox (сообщения ещё нет на сервере, нечего редактировать/удалять).

### Retry стратегия

Exponential backoff: 1, 2, 4, 8, 16, 32, 60, 60, 60...

Permanent ошибки (не ретраим): Forbidden, ChatNotFound, MessageTooLarge.
Transient ошибки (ретраим): timeout, network error, 5xx.

### Восстановление после перезапуска

При старте: `UPDATE outbox SET status=0 WHERE status=1` (откатываем sending → pending).

**Главное преимущество**: гарантия доставки, пережившая перезапуск приложения. Написал в метро без интернета → открыл дома → сообщение отправилось само.

---

## 14. Offline-стратегии команд

### Классификация

| Команда         | Метод | persist | При оффлайн                                          |
| --------------- | ----- | ------- | ---------------------------------------------------- |
| `SendMessage`   | push  | ✅       | → outbox (SQLite или in-memory)                      |
| `EditMessage`   | push  | ✅       | → outbox                                             |
| `DeleteMessage` | push  | ✅       | → outbox                                             |
| `ReadReceipt`   | push  | ✅       | → outbox                                             |
| `Typing`        | push  | ❌       | drop (ephemeral)                                     |
| `Subscribe`     | push  | ❌       | drop, выполнить при reconnect                        |
| `LoadMessages`  | rpc   | —       | LocalFallback (SQLite через chat_client_rs) или Fail |
| `Search`        | rpc   | —       | Fail (серверный поиск, оффлайн недоступен)           |
| `GetPresence`   | rpc   | —       | Fail (немедленная ошибка)                            |

### Push — fire and forget

Команды с `persist=true` помещаются в outbox немедленно (seq=0 — не ждём Ack). Ack приходит как отдельное событие. Outbox переживает restart приложения (если используется SQLite через `chat_client_rs`).

Без `chat_client_rs` — outbox хранится in-memory, не переживает restart.

---

## 15. WebSocket протокол (Клиент ↔ Сервер)

### Формат фрейма

```
WebSocket binary frame:
┌──────────┬──────────┬───────────┬──────────────────┐
│ ver: u8  │ kind: u8 │  seq: u32 │ payload: bytes   │
└──────────┴──────────┴───────────┴──────────────────┘
```

- `ver` — версия протокола (сейчас 1)
- `kind` — тип фрейма (`FrameKind` enum)
- `seq` — клиентский порядковый номер; `seq=0` для fire-and-forget фреймов (Typing, Ping, ReadReceipt)

### Типы фреймов

```rust
#[repr(u8)]
enum FrameKind {
    // Handshake
    Hello        = 0x01,  // клиент → сервер
    Welcome      = 0x02,  // сервер → клиент

    // Keepalive
    Ping         = 0x03,
    Pong         = 0x04,

    // Команды (клиент → сервер)
    SendMessage  = 0x10,
    EditMessage  = 0x11,
    DeleteMessage= 0x12,
    ReadReceipt  = 0x13,
    Typing       = 0x14,

    // Queries (клиент → сервер)
    GetPresence  = 0x15,
    LoadChats    = 0x16,
    Search       = 0x17,
    Subscribe    = 0x18,  // подписка на чат, last_update_ts
    Unsubscribe  = 0x19,  // отписка от чата
    LoadMessages = 0x1A,  // загрузка истории (older/newer cursor)

    // События (сервер → клиент)
    MessageNew   = 0x20,
    MessageEdited= 0x21,
    MessageDeleted=0x22,
    ReceiptUpdate= 0x23,
    TypingUpdate = 0x24,
    MemberJoined = 0x25,
    MemberLeft   = 0x26,
    SyncBatch    = 0x27,
    SyncComplete = 0x28,
    PresenceResult = 0x29,
    ChatUpdated  = 0x2A,
    ChatCreated  = 0x2B,

    // Ответы
    Ack          = 0x30,
    Error        = 0x31,

    // Управление чатами (клиент → сервер, rpc)
    CreateChat       = 0x40,
    UpdateChat       = 0x41,
    DeleteChat       = 0x42,
    GetChatInfo      = 0x43,
    GetChatMembers   = 0x44,
    InviteMembers    = 0x45,
    KickMember       = 0x46,
    LeaveChat        = 0x47,
    UpdateMemberRole = 0x48,
    MuteMember       = 0x49,
    BanMember        = 0x4A,
}
```

### Handshake

**Hello** (клиент → сервер): protocol_version, sdk_version, platform, token (JWT), device_id (UUID 16 bytes).

**Welcome** (сервер → клиент): session_id, server_time (синхронизация часов), user_id, missed_messages count, `ServerLimits`, `ServerCapabilities` (см. раздел 28).

### Версионирование

`ver` в заголовке WS фрейма достаточен для всего стека. При несовместимой версии сервер:
- Отклоняет через `Error(1004: unsupported_version)` если версия ниже минимальной
- Обрабатывает в рамках поддерживаемого диапазона (fallback для старых клиентов)

### Общий crate `chat_protocol`

Схема фреймов в отдельном crate, используется сервером и может использоваться тестовыми клиентами — один источник правды.

---

## 16. Аутентификация и сессии

### Device ID

- Генерируется один раз (UUID v4), хранится в отдельном файле (не в chat_u{uid}.db)
- Переживает смену пользователя, разлогин, очистку БД
- Не секрет — просто стабильный идентификатор устройства

### JWT

При логине device_id передаётся для получения JWT. Payload:

```json
{
  "sub": 123456,             // internal user_id (i64)
  "did": "uuid...",          // device_id
  "sid": "sess_789",         // session_id
  "plt": "android",          // platform
  "iat": 1710000000,
  "exp": 1710086400
}
```

Ключ сессии на сервере: `(user_id: i64, device_id: Uuid)`.

### Два способа аутентификации для SDK

**Способ 1 — Token Exchange (dev/прототипирование):**
Клиент передаёт свой API токен → сервер чата верифицирует через webhook на бэкенд разработчика.
- Просто для интеграции, но расширяет поверхность атаки.

**Способ 2 — Pre-issued Chat Token (продакшн, рекомендуемый):**
Бэкенд разработчика запрашивает chat_token у сервера чата → передаёт фронтенду.
- Токен основного API никогда не покидает инфраструктуру разработчика
- JWT верификация на чат-сервере локальная
- Стандартная схема (Sendbird, Stream, Twilio)

Различие по `iss` claim или префиксу в JWT.

### tokenProvider (не статический токен)

SDK принимает **функцию получения токена**, а не токен:

```dart
ChatSDK.initialize(
  auth: PreIssuedTokenAuth(
    tokenProvider: () => myBackend.getChatToken(),
  ),
);
```

При reconnect клиент вызывает `tokenProvider()` самостоятельно (нет обратного RPC через Rust — Dart управляет транспортом напрямую).

### Reconnect логика

```
Разрыв соединения
      │
      ▼
DisconnectCode.should_reconnect() == true?
      │ да
      ▼
токен ещё свежий? ──да──► reconnect с кэшированным токеном
      │
      нет
      │
      ▼
Dart: await tokenProvider()
      │
  ┌───┴────────────────┐
  токен получен        ошибка
  ▼                    ▼
reconnect         emit AuthError → Dart решает
```

Exponential backoff: 1, 2, 4, 8, 16, 30, 30, 30...
При 2 подряд Unauthorized — `Event::AuthError`.

### Хранение токенов

- Access token — в памяти (не персистентно)
- Refresh token — в secure storage (keychain/keystore)
- Крейт `keyring` абстрагирует все платформы

---

## 17. Управление базой данных при смене пользователя

### Отдельный файл БД на каждый uid

```
/data/app/databases/
├── chat_u123456.db
├── chat_u789012.db
```

```rust
fn db_path(uid: i64) -> PathBuf {
    data_dir().join(format!("chat_u{}.db", uid))
}
```

### При смене uid

1. Flush outbox текущего пользователя
2. Закрыть текущее соединение с БД
3. Закрыть WebSocket
4. Открыть БД нового пользователя
5. Event в Dart → сброс всего UI state

### Политика

| Ситуация                             | Действие                     |
| ------------------------------------ | ---------------------------- |
| Разлогин, пользователь хочет удалить | Удалить файл БД              |
| Разлогин, оставляет данные           | Оставить, закрыть соединения |
| Смена uid                            | Открыть другой файл БД       |
| Старые БД (>30 дней)                 | Удалять при запуске          |

---

## 18. Подписки на каналы

### Механизм

Клиент подписывается на конкретный чат, передавая `last_update_ts`. Сервер шлёт пропущенные сообщения батчами.

```rust
struct Subscribe {
    chat_id: i64,
    last_update_ts: Option<i64>,
}
```

### Синхронизация пропущенных

Сервер: `SELECT ... WHERE updated_at > last_update_ts` батчами по 50 с паузой 16ms между ними.

Клиент обновляет `last_update_ts` при получении каждого батча — если соединение оборвётся, продолжит с того места.

### Push уведомления — только неподписанным

Если пользователь подписан на чат — шлём через WebSocket. Если нет — push уведомление.

### При переподключении

Переподписываемся на все активные чаты автоматически.

---

## 19. Состояния подключения

Клиентский код (Dart/TS) должен реализовать следующие состояния и передавать их в UI:

| Состояние                         | Описание                                    | UI                           |
| --------------------------------- | ------------------------------------------- | ---------------------------- |
| `Connecting`                      | Первое подключение                          | Полная заглушка              |
| `Connected`                       | Handshake успешен                           | Чат                          |
| `Reconnecting(attempt, delay_ms)` | Переподключение после разрыва               | Баннер, чат читаем (из кэша) |
| `Disconnected`                    | Потеря сети, timeout                        | Баннер                       |
| `AuthError`                       | 2× Unauthorized, token_expired не обновился | Экран авторизации            |

```dart
chat.events.on<ConnectionEvent>((e) => switch (e) {
  Disconnected()                   => showBanner('Проверьте интернет'),
  Reconnecting(:final attempt)     => showBanner('Переподключение... $attempt'),
  Connected()                      => hideBanner(),
  AuthError()                      => router.replace('/login'),
  _                                => null,
});
```

Disconnect code из WS close frame определяет, нужен ли reconnect (см. раздел 37).

---

## 20. SDK интерфейс для разработчиков

### Точки интеграции

```dart
abstract class ChatAuthProvider {
  Future<String> getChatToken();
  Future<String> refreshChatToken(String expiredToken);
  String? get deviceId => null; // SDK генерирует если не задан
}
```

### Что SDK берёт на себя vs что делает разработчик

```
Разработчик:                    SDK (Dart/TS):
├── Получение токена            ├── WebSocket соединение
├── Refresh логика              ├── Reconnect / backoff
├── UI аутентификации           ├── Seq numbering и RPC correlation
├── User profile                ├── Subscribe / SyncBatch handling
├── Push уведомления            ├── Пагинация сообщений
├── Свой дизайн                 ├── Decode бинарных фреймов
└── Бизнес-логика               └── device_id (если не задан)

                                SDK (chat_client_rs, опционально):
                                ├── SQLite локальный кэш
                                ├── Outbox с persist через restart
                                └── Cursor-based load из SQLite
```

### ChatConfig

```dart
class ChatConfig {
  final String wsUrl;
  final ChatAuthProvider auth;
  final Duration connectTimeout;
  final Duration messageTimeout;
  /// Опционально: путь к SQLite для локального кэша через chat_client_rs
  /// Если null — работает без персистентного кэша (in-memory only)
  final String? localCachePath;
}
```

---

## 21. Темизация и кастомизация UI

### Два механизма (разделённых)

```
ChatTheme (InheritedWidget)
├── ChatThemeData          ← стили, цвета, отступы
│   ├── MessagesThemeData  (bubbleColor, textStyle, padding...)
│   ├── InputThemeData     (backgroundColor, sendButtonColor...)
│   ├── ChatListThemeData  (tileBackground, avatarSize...)
│   └── ConnectionStateThemeData
│
└── ChatComponents         ← замена целых виджетов
    ├── MessageBuilder?
    ├── MessageBubbleBuilder?
    ├── MessageContentBuilder? (text, image, file, deleted — отдельные)
    ├── InputBuilder?
    ├── ChatTileBuilder?
    ├── ConnectionStateBuilder? (connecting, reconnecting, authError)
    └── ...
```

`ChatThemeData` — только стили (по аналогии с ThemeData). `ChatComponents` — типизированные builder callbacks.

### Использование

```dart
// Простая интеграция — только цвета
ChatTheme(
  data: ChatThemeData(
    messages: MessagesThemeData(outgoingBubbleColor: Colors.green),
  ),
  components: const ChatComponents(),
  child: ChatView(client: chat),
)

// Полная замена компонента
ChatTheme(
  data: ChatThemeData(),
  components: ChatComponents(
    messageBubbleBuilder: (context, msg, child) => MyBubble(child: child),
  ),
  child: ChatView(client: chat),
)
```

### Состояния подключения в UI

| Состояние    | UI                              |
| ------------ | ------------------------------- |
| Connecting   | Полная заглушка (первый запуск) |
| Reconnecting | Баннер сверху, чат читаем       |
| AuthError    | Полная заглушка + retry         |
| Connected    | Чат                             |

Все builder'ы переопределяемы, дефолты достаточно нейтральные.

---

## 22. Серверная расширяемость

### Уровень 1 — Webhooks (async)

```yaml
webhooks:
  url: https://api.example.com/chat/webhook
  secret: "hmac_secret"
  events: [message.created, message.updated, chat.created, ...]
```

Payload с HMAC-SHA256 подписью, дедупликация по event UUID.

### Уровень 2 — Interceptors (sync)

Модификация/блокировка сообщения до сохранения. Таймаут 500ms, потом Allow по умолчанию.

```rust
enum InterceptAction {
    Allow,
    Modify { content, extra },
    Reject { reason },
}
```

### Уровень 3 — Bot API

Боты — пользователи с `is_bot: true` и токеном. REST API (`/bot/v1/sendMessage`, `/bot/v1/answerCallback`). Получение событий через webhook или long polling.

### Pipeline обработки сообщения

```
Входящее сообщение
    ▼ Interceptor (sync, опционально)
    ▼ Сохранение в SQLite
    ▼ Доставка WebSocket клиентам
    ▼ Webhook (async)
    ▼ Bot API events
```

---

## 23. Тарифы и масштабирование

### Free — Single Node (self-hosted)

- Ограничение: ~50-100 concurrent users
- Всё в памяти одного процесса: `DashMap` для сессий, tokio `broadcast` для pub/sub
- Webhooks, Bot API, Interceptors — отключены

### Pro — Multi Node (Redis)

- Unlimited concurrent users
- Redis Pub/Sub для доставки между нодами
- Redis для присутствия (online status) с TTL + heartbeat
- Redis для distributed rate limiting

### Абстракция через трейт

```rust
#[async_trait]
trait ClusterBackend: Send + Sync {
    async fn broadcast(&self, chat_id: i64, msg: Arc<Message>) -> Result<()>;
    async fn set_online(&self, user_id: i64, device_id: Uuid) -> Result<()>;
    async fn is_online(&self, user_id: i64) -> Result<bool>;
    async fn increment_connections(&self) -> Result<u32>;
    // ...
}

struct SingleNodeBackend { ... }  // Free
struct RedisBackend { ... }       // Pro
```

### Лицензирование

JWT ключ с offline верификацией (подпись). Не нужен callback домой.

---

## 24. Дистрибуция

### Dart / Flutter пакеты

```
pub.dev (открытые):
├── chat_platform_interface   ← абстрактные типы, 0 native deps
└── chat_flutter              ← виджеты

GitHub Releases / S3 (бинари chat_client_rs — ОТДЕЛЬНЫЙ репозиторий):
└── v1.0.2/
    ├── libchat-android-arm64.so
    ├── libchat-android-armv7.so
    ├── libchat-android-x64.so
    ├── libchat-ios.a
    ├── libchat-macos.dylib
    ├── chat-windows.dll
    └── libchat-linux.so
```

### Native Assets hook

`hook/build.dart` в пакете `chat_core` скачивает pre-built бинарь `chat_client_rs` при `flutter build` (не при `pub get`). SHA256 checksum верификация. Кэш в `.dart_tool/chat_blobs/`.

Бинари необходимы только если используется локальный SQLite кэш. Без `chat_client_rs` приложение работает без native deps.

### TypeScript / npm

```
npm (открытый):
└── @chat-sdk/client          ← WS клиент + codec + EventBus
```

### Альтернатива — приватный pub сервер

Протокол: `GET /api/packages/{name}` (JSON), `GET .../versions/{version}.tar.gz` (архив).

Важно: pub.dev не разрешает зависимости от пакетов с других хостов.

---

## 25. Кроссплатформенность

### Транспорт — нативные средства каждой платформы

| Platform                        | WS transport                              |
| ------------------------------- | ----------------------------------------- |
| Flutter (Android, iOS, Desktop) | `web_socket_channel` или platform channel |
| Flutter Web                     | TypeScript клиент через `dart:js_interop` |
| TypeScript (browser)            | `WebSocket` API + `SharedWorker`          |
| TypeScript (Node.js)            | `ws` npm package                          |

### Сборка chat_client_rs (отдельный репозиторий, опционально)

| Platform | Tooling                                                          |
| -------- | ---------------------------------------------------------------- |
| Android  | `cargo-ndk` для arm64-v8a, armeabi-v7a, x86_64                   |
| iOS      | Universal XCFramework: aarch64-apple-ios + aarch64-apple-ios-sim |
| Desktop  | Native cargo                                                     |

### WASM — не реализовывать

TypeScript клиент полностью покрывает web use cases. WASM Rust клиент для web не нужен.

---

## 26. Дополнительные фичи

### Критичные для MVP

- **Медиа**: Upload через HTTP (не WS), chunked upload с resume, LRU кэш, thumbnails
- **Push уведомления**: FCM / APNs / WNS, только для неподписанных на чат
- **Read receipts**: таблица `(chat_id, user_id) → last_read_message_id`
- **Typing indicators**: только через WebSocket (ephemeral), debounce 3 сек на клиенте. Нет фрейма "stop typing" — сервер/клиент считает пользователя typing если последний typing пришёл < 5 сек назад (timeout-based)

### Важные но не блокирующие

- **Threads / Replies**: `reply_to_id` + денормализованный preview
- **Реакции**: таблица `(message_id, user_id, emoji)`
- **Права и роли**: bitflags Permission (send, delete, pin, invite, kick, manage)
- **Типы чатов**: Direct, Group, Channel
- **Pinned сообщения**, **Forwarding**

### SDK-специфичные

- **MockChatClient**: для тестирования UI без сервера
- **DB миграции**: `rusqlite_migration`, версия в `PRAGMA user_version`
- **Локализация**: стандартный Flutter механизм, переопределяемый
- **Аналитика**: абстрактный `ChatAnalytics` (no-op по умолчанию)

### За рамками SDK

| Фича           | Причина                           |
| -------------- | --------------------------------- |
| Voice / Video  | Отдельная область (WebRTC)        |
| Stickers / GIF | Контент-специфично, через extra   |
| E2EE           | Меняет всю архитектуру            |
| Link previews  | Через extra + сервис разработчика |

---

## 27. Приоритеты реализации

```
0. chat_protocol crate                                    ← контракт, первый crate
1. chat_server: WS + PostgreSQL + Hello/Welcome           ← ядро сервера
   + Dart/TS клиент: прямое WS подключение, codec, seq   ← ядро клиента
2. Outbox + read receipts + typing                        ← базовый UX
   (outbox — в chat_client_rs для Dart, in-memory для TS)
3. Media upload/download                                  ← без этого чат неполный
4. Push уведомления                                       ← retention
5. Replies + reactions                                    ← engagement
6. Роли + права                                           ← enterprise
7. Webhooks + Bot API                                     ← платформа
8. Clustering (Redis)                                     ← масштабирование
```

---

## 28. Серверные лимиты и конфигурация сессии

### Конфигурируемые лимиты (не захардкожены)

Лимиты задаются в конфиге сервера и передаются клиенту при установлении сессии в `Welcome` фрейме:

```rust
struct ServerLimits {
    max_message_content_length: u32,  // символов (default: 4096)
    max_extra_size: u32,              // байт JSON (default: 4096)
    max_frame_size: u32,              // байт всего пакета (default: 32768 = 32KB)
    max_rich_content_size: u32,       // байт BLOB (default: 8192)
    max_attachment_size: u64,         // байт для upload (default: 50MB)
    max_attachments_per_message: u8,  // default: 10
    // Rate limits (для клиентского debounce)
    messages_per_minute_per_chat: u32, // default: 30
    messages_burst: u32,               // default: 5
    // Capabilities (bitflags) — что сервер поддерживает
    capabilities: u32,                 // см. ServerCapabilities в разделе 41
}
```

```rust
// Welcome фрейм расширяется
struct Welcome {
    session_id: [u8; 16],
    server_time: i64,
    user_id_len: u8,
    user_id: Vec<u8>,
    missed_messages: u32,
    // Серверные лимиты и мета-информация
    limits: ServerLimits,
}
```

Клиентская библиотека валидирует лимиты **до отправки** — не тратит сетевой round-trip на заведомо невалидные запросы.

Если пользователю нужно больше данных чем позволяет `max_extra_size` — он кладёт в extra ID и получает данные со своего бэкенда по этому ID.

---

## 29. Integration тесты

### Обязательный E2E тест: два клиента

Минимальный `TestClient` в `tests/helpers/` использует `tokio-tungstenite` + `chat_protocol` codec напрямую — без `chat_client_rs`. Два клиента подключаются к тестовому серверу, один отправляет сообщение, другой получает.

```rust
#[tokio::test]
async fn two_clients_message_delivery() {
    let server = TestServer::start().await;

    let alice = TestClient::connect(server.url(), server.token_for("alice")).await;
    let bob   = TestClient::connect(server.url(), server.token_for("bob")).await;

    // Подписаться на чат
    alice.subscribe(chat_id, 0).await;
    bob.subscribe(chat_id, 0).await;

    // Bob отправляет сообщение
    bob.send_message(chat_id, "Hello Alice").await;

    // Alice получает MessageNew event
    let event = alice.recv_event::<MessageNewPayload>().await;
    assert_eq!(event.content, "Hello Alice");
}
```

Этот тест должен быть зелёным до начала работы над клиентскими SDK.

---

## 30. Серверная архитектура

### Стек

| Компонент     | Crate / Технология                                           | Назначение                                  |
| ------------- | ------------------------------------------------------------ | ------------------------------------------- |
| Web framework | `axum` (features = `ws`)                                     | HTTP + WebSocket, extractors                |
| WebSocket     | `axum` → `tokio-tungstenite` → `tungstenite`                 | Под капотом axum уже использует tungstenite |
| Async runtime | `tokio` (features = `full`)                                  | Единый runtime                              |
| БД            | PostgreSQL + `sqlx` (features = `postgres`, `runtime-tokio`) | Async, compile-time checked queries         |
| Миграции      | `sqlx` CLI (`sqlx migrate`)                                  | Версионирование серверной схемы             |
| Кэш/Pub-Sub   | Redis (через `deadpool-redis`)                               | Pro тариф: кластеризация, presence          |
| JWT           | `jsonwebtoken`                                               | Верификация токенов                         |
| Rate limiting | `governor`                                                   | GCRA, keyed, lock-free, конфигурируемый     |
| Конфигурация  | `toml` + `serde`                                             | Файл конфигурации сервера                   |
| Протокол      | `chat_protocol` (workspace crate)                            | Общие типы с клиентом                       |

### Почему axum + tungstenite (а не fastwebsockets и т.д.)

```
Server:  axum --ws--> tokio-tungstenite --> tungstenite
Client:  tokio-tungstenite ------------> tungstenite
```

- axum уже использует `tokio-tungstenite` под капотом — единый стек бесплатно
- Одинаковое поведение протокола на обоих концах — меньше класс багов "работает на клиенте, ломается на сервере"
- `fastwebsockets` — frame-level API, меньше экосистема (75 dependents vs 5196), нет compelling причин
- `tokio-tungstenite` + `rustls` на клиенте: нет зависимости от системного OpenSSL (критично для Android/iOS)

### Серверная структура

```
crates/chat_server/
├── src/
│   ├── main.rs           ← entry point, конфигурация
│   ├── config.rs         ← TOML конфиг, ServerLimits
│   ├── app.rs            ← axum Router, middleware
│   ├── ws/
│   │   ├── mod.rs        ← WebSocket upgrade handler
│   │   ├── session.rs    ← DeviceSession, подписки
│   │   └── dispatch.rs   ← диспетчеризация фреймов
│   ├── handlers/
│   │   ├── auth.rs       ← POST /auth/token (exchange), POST /auth/verify
│   │   ├── bot.rs        ← Bot API endpoints
│   │   └── upload.rs     ← Media upload (chunked)
│   ├── services/
│   │   ├── message.rs    ← бизнес-логика сообщений
│   │   ├── chat.rs       ← создание/управление чатами
│   │   ├── delivery.rs   ← доставка: WS + push + webhook
│   │   ├── presence.rs   ← онлайн статусы
│   │   └── outbox.rs     ← обработка входящих outbox команд
│   ├── db/
│   │   ├── mod.rs        ← пул соединений (sqlx::PgPool)
│   │   ├── queries/      ← SQL запросы (compile-time checked)
│   │   └── models.rs     ← серверные модели
│   ├── cluster/
│   │   ├── mod.rs        ← ClusterBackend trait
│   │   ├── single.rs     ← SingleNodeBackend (Free)
│   │   └── redis.rs      ← RedisBackend (Pro)
│   └── webhook/
│       ├── sender.rs     ← отправка webhook событий
│       └── interceptor.rs← синхронный interceptor
├── migrations/
│   ├── 001_initial.sql
│   ├── 002_sessions.sql
│   └── ...
└── config.example.toml
```

### ID пользователей: внутренние i64 + внешние string

Сторонние разработчики идентифицируют пользователей строковыми ID из своей системы (`"user_123"`, UUID и т.д.). Но бинарный wire-протокол и SQLite клиент работают с `i64` для эффективности. Решение — сервер поддерживает маппинг:

- `users.id BIGSERIAL` — внутренний `i64`, используется во всём wire протоколе и клиентском SQLite
- `users.external_id TEXT UNIQUE` — строковый ID из системы разработчика
- При создании пользователя через SDK/API: разработчик передаёт `external_id`, сервер возвращает `id: i64`
- JWT содержит внутренний `user_id: i64` (не строковый)
- REST API и Bot API принимают оба формата через роутинг: `/users/{id}` (i64) или `/users/ext/{external_id}` (string)

### Серверная PostgreSQL схема (ключевые таблицы)

```sql
-- Пользователи (минимально — SDK не управляет профилями)
CREATE TABLE users (
    id          BIGSERIAL PRIMARY KEY,   -- внутренний i64, используется в wire протоколе
    external_id TEXT NOT NULL UNIQUE,     -- string user_id от системы разработчика
    is_bot      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Чаты
CREATE TABLE chats (
    id          BIGSERIAL PRIMARY KEY,
    kind        SMALLINT NOT NULL,       -- 0=direct, 1=group, 2=channel
    title       TEXT,
    avatar_url  TEXT,
    created_by  BIGINT NOT NULL REFERENCES users(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Участники чатов с ролями
CREATE TABLE chat_members (
    chat_id     BIGINT NOT NULL REFERENCES chats(id),
    user_id     BIGINT NOT NULL REFERENCES users(id),
    role        SMALLINT NOT NULL DEFAULT 0,  -- см. раздел 31
    permissions BIGINT NOT NULL DEFAULT 0,    -- битовые флаги, override роли
    joined_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chat_id, user_id)
);

-- Сообщения
CREATE TABLE messages (
    id          BIGSERIAL PRIMARY KEY,
    chat_id     BIGINT NOT NULL REFERENCES chats(id),
    sender_id   BIGINT NOT NULL REFERENCES users(id),
    kind        SMALLINT NOT NULL,
    status      SMALLINT NOT NULL DEFAULT 0,
    content     TEXT NOT NULL,
    rich_content BYTEA,
    extra       JSONB,
    reply_to_id BIGINT REFERENCES messages(id),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_messages_chat ON messages(chat_id, id);
CREATE INDEX idx_messages_chat_updated ON messages(chat_id, updated_at);

-- Сессии устройств
CREATE TABLE device_sessions (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     BIGINT NOT NULL REFERENCES users(id),
    device_id   UUID NOT NULL,
    platform    SMALLINT NOT NULL,
    app_version TEXT,
    last_active_chat BIGINT REFERENCES chats(id),
    push_token  TEXT,
    connected_at TIMESTAMPTZ,
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (user_id, device_id)
);

-- Read receipts
CREATE TABLE read_receipts (
    chat_id     BIGINT NOT NULL REFERENCES chats(id),
    user_id     BIGINT NOT NULL REFERENCES users(id),
    message_id  BIGINT NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chat_id, user_id)
);

-- Реакции
CREATE TABLE reactions (
    message_id  BIGINT NOT NULL REFERENCES messages(id),
    user_id     BIGINT NOT NULL REFERENCES users(id),
    emoji       TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (message_id, user_id, emoji)
);
```

### Конфигурация сервера

```toml
# config.toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://chat:password@localhost/chat_db"
max_connections = 20

[auth]
# Способ 2 (рекомендуемый) — свой JWT
jwt_secret = "..."
# Способ 1 (опциональный) — exchange
exchange_url = "https://api.example.com/verify"
exchange_timeout_ms = 3000

[limits]
max_message_content_length = 4096
max_extra_size = 4096
max_frame_size = 32768
max_rich_content_size = 8192

[rate_limits]
connections_per_minute_per_ip = 10
connections_burst = 3
messages_per_minute_per_chat = 30
messages_burst = 5
commands_per_minute = 60
commands_burst = 10

[cluster]
enabled = false
redis_url = "redis://localhost:6379"

[webhooks]
enabled = false
url = ""
secret = ""
events = ["message.created"]

[license]
key = ""  # пусто = Free тариф
```

### Миграции (sqlx)

```bash
# Создание миграции
sqlx migrate add initial

# Применение
sqlx migrate run --database-url postgres://...

# В коде при старте сервера
sqlx::migrate!("./migrations").run(&pool).await?;
```

Compile-time проверка SQL запросов через `sqlx::query!` макрос — ошибки в SQL ловятся при компиляции, а не в рантайме.

---

## 31. Ролевая модель и права доступа

### Роли (по аналогии с Telegram/Discord)

```rust
#[repr(i16)]
enum ChatRole {
    Member    = 0,  // обычный участник
    Moderator = 1,  // модерация: удаление чужих сообщений, мут
    Admin     = 2,  // управление: приглашение, кик, настройки чата
    Owner     = 3,  // владелец: передача владения, удаление чата
}
```

### Права (битовые флаги)

```rust
bitflags::bitflags! {
    pub struct Permission: i64 {
        // Сообщения
        const SEND_MESSAGES     = 1 << 0;
        const SEND_MEDIA        = 1 << 1;
        const SEND_LINKS        = 1 << 2;
        const PIN_MESSAGES      = 1 << 3;
        const EDIT_OWN_MESSAGES = 1 << 4;
        const DELETE_OWN_MESSAGES = 1 << 5;

        // Модерация
        const DELETE_OTHERS_MESSAGES = 1 << 10;
        const MUTE_MEMBERS      = 1 << 11;
        const BAN_MEMBERS       = 1 << 12;

        // Управление
        const INVITE_MEMBERS    = 1 << 20;
        const KICK_MEMBERS      = 1 << 21;
        const MANAGE_CHAT_INFO  = 1 << 22;  // название, аватар
        const MANAGE_ROLES      = 1 << 23;

        // Владелец
        const TRANSFER_OWNERSHIP = 1 << 30;
        const DELETE_CHAT       = 1 << 31;
    }
}
```

### Дефолтные права по ролям

```rust
fn default_permissions(role: ChatRole, chat_kind: ChatKind) -> Permission {
    match (role, chat_kind) {
        (ChatRole::Owner, _) => Permission::all(),
        (ChatRole::Admin, _) => Permission::SEND_MESSAGES
            | Permission::SEND_MEDIA | Permission::SEND_LINKS
            | Permission::PIN_MESSAGES | Permission::EDIT_OWN_MESSAGES
            | Permission::DELETE_OWN_MESSAGES | Permission::DELETE_OTHERS_MESSAGES
            | Permission::MUTE_MEMBERS | Permission::BAN_MEMBERS
            | Permission::INVITE_MEMBERS | Permission::KICK_MEMBERS
            | Permission::MANAGE_CHAT_INFO | Permission::MANAGE_ROLES,
        (ChatRole::Moderator, _) => Permission::SEND_MESSAGES
            | Permission::SEND_MEDIA | Permission::SEND_LINKS
            | Permission::PIN_MESSAGES | Permission::EDIT_OWN_MESSAGES
            | Permission::DELETE_OWN_MESSAGES | Permission::DELETE_OTHERS_MESSAGES
            | Permission::MUTE_MEMBERS,
        (ChatRole::Member, ChatKind::Channel) => Permission::empty(),  // read-only
        (ChatRole::Member, _) => Permission::SEND_MESSAGES
            | Permission::SEND_MEDIA | Permission::SEND_LINKS
            | Permission::EDIT_OWN_MESSAGES | Permission::DELETE_OWN_MESSAGES,
    }
}
```

### Override через `chat_members.permissions`

Поле `permissions` в `chat_members` позволяет **переопределить** дефолтные права конкретного участника. Если не 0 — используется вместо дефолтных прав роли. Это позволяет, например, замутить конкретного пользователя (убрав `SEND_MESSAGES`) или дать обычному участнику право приглашать.

### Проверка прав

```rust
fn has_permission(member: &ChatMember, perm: Permission) -> bool {
    if member.permissions != 0 {
        // Есть override — используем его
        Permission::from_bits_truncate(member.permissions).contains(perm)
    } else {
        // Нет override — дефолт по роли
        default_permissions(member.role, chat_kind).contains(perm)
    }
}
```

### Конфигурация сервера — дефолтные роли

Разработчик может переопределить дефолтные права через конфиг сервера:

```toml
[roles.member]
permissions = ["send_messages", "send_media", "edit_own_messages", "delete_own_messages"]

[roles.moderator]
permissions = ["send_messages", "send_media", "delete_others_messages", "mute_members"]
```

---

## 32. Коды ошибок

### Структура

Каждая ошибка имеет:
- `code: u16` — машиночитаемый числовой код с ranges по категориям
- `slug: String` — человекочитаемый идентификатор (snake_case), стабильный для клиентской логики
- `message: String` — описание для разработчика (не для конечного пользователя)

### Wire формат

```
ErrorFrame:
┌──────────┬──────────┬───────────┬──────────────────┬──────────────────┐
│ seq: u32 │ code: u16│ slug_len:u8│ slug (UTF-8)    │ message (UTF-8)  │
└──────────┴──────────┴───────────┴──────────────────┴──────────────────┘
```

### Ranges кодов

```rust
#[repr(u16)]
enum ErrorCode {
    // 1xxx — Аутентификация и авторизация
    Unauthorized          = 1000,  // "unauthorized" — токен невалиден
    TokenExpired          = 1001,  // "token_expired" — токен истёк
    Forbidden             = 1002,  // "forbidden" — нет прав
    SessionRevoked        = 1003,  // "session_revoked" — сессия отозвана
    UnsupportedVersion    = 1004,  // "unsupported_version" — версия протокола не поддерживается

    // 2xxx — Чаты
    ChatNotFound          = 2000,  // "chat_not_found"
    ChatAlreadyExists     = 2001,  // "chat_already_exists" (для direct chats)
    NotChatMember         = 2002,  // "not_chat_member"
    ChatFull              = 2003,  // "chat_full" — лимит участников

    // 3xxx — Сообщения
    MessageNotFound       = 3000,  // "message_not_found"
    MessageTooLarge       = 3001,  // "message_too_large"
    ExtraTooLarge         = 3002,  // "extra_too_large"
    RateLimited           = 3003,  // "rate_limited" — слишком много сообщений
    ContentFiltered       = 3004,  // "content_filtered" — interceptor отклонил

    // 4xxx — Медиа
    FileTooLarge          = 4000,  // "file_too_large"
    UnsupportedMediaType  = 4001,  // "unsupported_media_type"
    UploadFailed          = 4002,  // "upload_failed"

    // 5xxx — Серверные
    InternalError         = 5000,  // "internal_error"
    ServiceUnavailable    = 5001,  // "service_unavailable"
    DatabaseError         = 5002,  // "database_error"

    // 9xxx — Протокол
    MalformedFrame        = 9000,  // "malformed_frame"
    UnknownCommand        = 9001,  // "unknown_command"
    FrameTooLarge         = 9002,  // "frame_too_large"
}
```

### Определение transient/permanent на клиенте

```rust
impl ErrorCode {
    /// Permanent ошибки — не ретраим в outbox
    fn is_permanent(&self) -> bool {
        matches!(self,
            ErrorCode::Forbidden
            | ErrorCode::ChatNotFound
            | ErrorCode::NotChatMember
            | ErrorCode::MessageTooLarge
            | ErrorCode::ExtraTooLarge
            | ErrorCode::ContentFiltered
            | ErrorCode::UnsupportedMediaType
        )
    }

    /// Transient ошибки — ретраим с backoff
    fn is_transient(&self) -> bool {
        matches!(self,
            ErrorCode::InternalError
            | ErrorCode::ServiceUnavailable
            | ErrorCode::DatabaseError
            | ErrorCode::RateLimited
        )
    }
}
```

### Использование на Dart стороне

```dart
chat.events.on<ErrorEvent>((e) {
  switch (e.slug) {
    case 'rate_limited':
      showSnackbar('Слишком много сообщений, подождите');
    case 'forbidden':
      showSnackbar('У вас нет прав для этого действия');
    case 'content_filtered':
      showSnackbar('Сообщение отклонено модерацией');
    default:
      if (e.code >= 5000 && e.code < 6000) {
        showSnackbar('Ошибка сервера, попробуйте позже');
      }
  }
});
```

Slug стабилен между версиями — клиентская логика привязывается к нему, а не к числовому коду.

---

## 33. Пагинация списка чатов

### Cursor-based по `last_message_at`

Список чатов сортируется по времени последнего сообщения (как в любом мессенджере). Пагинация через cursor, не OFFSET:

```sql
-- Серверный запрос: чаты пользователя, отсортированные по активности
SELECT c.id, c.kind, c.title, c.avatar_url,
       m.content AS last_message_content,
       m.sender_id AS last_message_sender,
       m.created_at AS last_message_at,
       COALESCE(ur.message_id, 0) AS last_read_id,
       (SELECT COUNT(*) FROM messages
        WHERE chat_id = c.id AND id > COALESCE(ur.message_id, 0)) AS unread_count
FROM chat_members cm
JOIN chats c ON c.id = cm.chat_id
LEFT JOIN LATERAL (
    SELECT content, sender_id, created_at FROM messages
    WHERE chat_id = c.id ORDER BY id DESC LIMIT 1
) m ON true
LEFT JOIN read_receipts ur ON ur.chat_id = c.id AND ur.user_id = cm.user_id
WHERE cm.user_id = $1
  AND ($2::timestamptz IS NULL OR m.created_at < $2)  -- cursor
ORDER BY m.created_at DESC NULLS LAST
LIMIT $3;
```

### Клиентская сторона

```dart
// Первая страница
final chats = await rpc(LoadChats(limit: 30));

// Следующая страница — cursor = last_message_at последнего чата
final more = await rpc(LoadChats(limit: 30, before: chats.last.lastMessageAt));
```

### Обновления в реальном времени

Чаты, в которых происходят изменения, приходят Event'ами через WebSocket:
- `Event::MessageNew` — чат поднимается вверх списка
- `Event::ChatCreated` — новый чат появляется
- `Event::ChatUpdated` — изменение названия/аватара
- `Event::UnreadCountChanged` — обновление счётчика

Клиент **не перезапрашивает** весь список — просто обновляет конкретный чат в UI по событию.

### Локальный кэш списка чатов (SQLite на клиенте)

```sql
CREATE TABLE chats (
    id              INTEGER PRIMARY KEY,
    kind            INTEGER NOT NULL,
    title           TEXT,
    avatar_url      TEXT,
    last_message_id INTEGER,
    last_message_content TEXT,
    last_message_sender_id INTEGER,
    last_message_at INTEGER,
    unread_count    INTEGER NOT NULL DEFAULT 0,
    updated_at      INTEGER NOT NULL
);
```

При открытии приложения — показываем из локального SQLite кэша мгновенно (если используется `chat_client_rs`), параллельно синхронизируем с сервером.

---

## 34. Rate Limiting

### Архитектура: два уровня

Rate limiting для WebSocket работает иначе чем для HTTP — после handshake middleware больше не вызывается. Поэтому два отдельных уровня:

| Уровень        | Что ограничивает                 | Где                             | Инструмент       |
| -------------- | -------------------------------- | ------------------------------- | ---------------- |
| **Соединения** | Подключения с одного IP          | axum Tower middleware           | `governor` crate |
| **Сообщения**  | Сообщения от пользователя в чате | Бизнес-логика (WS message loop) | `governor` crate |

### Crate: `governor`

- Алгоритм GCRA (Generic Cell Rate Algorithm) ≈ leaky bucket
- Lock-free atomics, 64 бит на лимитер
- Встроенный `KeyedRateLimiter` — ключ по user_id, chat_id, IP
- Конфигурируемые квоты: `Quota::per_minute(nonzero!(30u32)).allow_burst(nonzero!(5u32))`

### Конфигурация (не захардкожено)

```toml
[rate_limits]
# Подключения
connections_per_minute_per_ip = 10
connections_burst = 3

# Сообщения
messages_per_minute_per_chat = 30
messages_burst = 5

# Команды (edit, delete, reaction)
commands_per_minute = 60
commands_burst = 10
```

### Абстракция через трейт

```rust
#[async_trait]
trait RateLimiter: Send + Sync {
    /// Проверить лимит. Ok(()) = разрешено, Err = заблокировано
    async fn check_message(&self, user_id: i64, chat_id: i64) -> Result<(), RateLimitError>;
    async fn check_connection(&self, ip: IpAddr) -> Result<(), RateLimitError>;
    async fn check_command(&self, user_id: i64) -> Result<(), RateLimitError>;
}

struct RateLimitError {
    pub retry_after_ms: u64,
}
```

### Free tier — in-memory (`governor` default)

```rust
struct InMemoryRateLimiter {
    messages: RateLimiter<String, DashMapStateStore<String>, DefaultClock>,
    connections: RateLimiter<IpAddr, DashMapStateStore<IpAddr>, DefaultClock>,
    commands: RateLimiter<String, DashMapStateStore<String>, DefaultClock>,
}
```

Ключ для messages: `(user_id, chat_id)` — пара i64. Zero внешних зависимостей.

### Pro tier — Redis (для multi-node)

Lua-скрипт реализующий GCRA на vanilla Redis (без кастомных модулей):

```rust
struct RedisRateLimiter {
    pool: deadpool_redis::Pool,
    message_quota: Quota,
    connection_quota: Quota,
}

impl RateLimiter for RedisRateLimiter {
    async fn check_message(&self, user_id: i64, chat_id: i64) -> Result<(), RateLimitError> {
        let key = format!("rl:msg:{user_id}:{chat_id}");
        self.check_redis(&key, &self.message_quota).await
    }
}
```

### Где в потоке обработки

```
WS Connection Established
         │
         ▼
    Message Loop:
    ┌─────────────────────────────────────┐
    │  1. recv() WS frame                 │
    │  2. rate_limiter.check_message()    │
    │  3. if limited → ErrorFrame(3003)   │  ← "rate_limited" slug
    │  4. else → process message          │
    └─────────────────────────────────────┘
```

### Что передавать клиенту при блокировке

```rust
// ErrorFrame с retry_after
ErrorFrame {
    seq: frame.seq,
    code: ErrorCode::RateLimited,  // 3003
    slug: "rate_limited",
    message: "Rate limit exceeded, retry after 2000ms",
    // В payload дополнительно:
    retry_after_ms: 2000,
}
```

Клиент может показать таймер или просто подождать.

### Лимиты передаются в Welcome

Клиент получает текущие лимиты при установлении сессии (см. раздел 28, `ServerLimits`). Это позволяет SDK валидировать на клиенте и дебаунсить отправку до достижения лимита, а не упираться в серверную ошибку.

---

## 35. Модель соединений

### Одно WS соединение на клиент (устройство/приложение)

- Один пользователь с телефона и ноутбука — два WS соединения, две сессии
- Все подписки, события, команды мультиплексируются через единственное соединение
- Серверная сторона: `HashMap<(i64, Uuid), WsSession>` — ключ (user_id, device_id)

### Web-клиент (TypeScript)

TypeScript клиент — независимая библиотека, реализующая тот же WS протокол что и Dart клиент. Для браузера использует `SharedWorker`: одна вкладка держит WS соединение, остальные общаются с ней через `MessagePort` — одно соединение на все вкладки.

Flutter Web использует TypeScript клиент через `dart:js_interop` + `extension type`.

---

## 36. Deduplication / Idempotency

### Проблема

Сервер получил `SendMessage`, сохранил, но `Ack` не дошёл до клиента (обрыв). Клиент отправит повторно из outbox → дубль.

### Решение: `idempotency_key`

Каждая команда из outbox (persist: true) содержит UUID, сгенерированный клиентом при первой отправке:

```rust
// В SendMessage frame
struct SendMessagePayload {
    chat_id: i64,
    idempotency_key: [u8; 16],  // UUID v4, генерируется клиентом
    content_len: u32,
    content: Vec<u8>,
    // ...
}
```

### Серверная сторона

```sql
-- Таблица для дедупликации (TTL через cron или pg_cron)
CREATE TABLE idempotency_keys (
    key         UUID PRIMARY KEY,
    message_id  BIGINT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Очистка старых ключей (>24h)
DELETE FROM idempotency_keys WHERE created_at < NOW() - INTERVAL '24 hours';
```

```rust
async fn handle_send_message(&self, payload: SendMessagePayload) -> Result<AckFrame> {
    // Проверяем idempotency_key
    if let Some(existing) = self.db.get_idempotency_key(payload.idempotency_key).await? {
        // Уже обработано — возвращаем Ack с существующим message_id
        return Ok(AckFrame { message_id: existing.message_id, .. });
    }

    // Новое сообщение — сохраняем + регистрируем ключ в одной транзакции
    let message_id = self.db.insert_message_with_idempotency(
        &payload, payload.idempotency_key
    ).await?;

    Ok(AckFrame { message_id, .. })
}
```

### Клиентская сторона

UUID генерируется один раз при помещении в outbox. При повторных попытках отправки — тот же UUID.

```sql
-- Outbox на клиенте (SQLite) — добавляем поле
CREATE TABLE outbox (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    idempotency_key BLOB NOT NULL,  -- 16 bytes UUID
    chat_id         INTEGER NOT NULL,
    kind            INTEGER NOT NULL,
    payload         BLOB NOT NULL,
    created_at      INTEGER NOT NULL,
    status          INTEGER NOT NULL DEFAULT 0,
    attempts        INTEGER NOT NULL DEFAULT 0,
    last_attempt    INTEGER,
    error           TEXT
);
```

---

## 37. Disconnect коды

### Структура

Disconnect может прийти как через WS transport (close frame), так и через application-level фрейм перед закрытием. Код определяет, может ли клиент переподключиться.

### Ranges

| Range     | Категория            | Reconnect | Описание                              |
| --------- | -------------------- | --------- | ------------------------------------- |
| 0–999     | Internal / Transport | ✅ Да      | Клиентские и транспортные коды        |
| 1000–1999 | WebSocket стандарт   | По коду   | RFC 6455 close codes                  |
| 3000–3499 | Server non-terminal  | ✅ Да      | Серверные, переподключение допустимо  |
| 3500–3999 | Server terminal      | ❌ Нет     | Серверные, переподключение запрещено  |
| 4000–4499 | Custom non-terminal  | ✅ Да      | Пользовательские, переподключение     |
| 4500–4999 | Custom terminal      | ❌ Нет     | Пользовательские, без переподключения |

### Коды

```rust
#[repr(u16)]
enum DisconnectCode {
    // Internal (0–99) — reconnect: yes
    Normal              = 0,     // штатное отключение клиентом
    NoPing              = 1,     // сервер не отвечает на ping

    // Server non-terminal (3000–3499) — reconnect: yes
    ServerShutdown      = 3000,  // graceful shutdown, клиент переподключится к другой ноде
    ServerRestart       = 3001,  // перезапуск
    InsufficientState   = 3002,  // сервер потерял состояние сессии
    ServerError         = 3003,  // внутренняя ошибка
    BufferOverflow      = 3004,  // серверный буфер переполнен (backpressure), reconnect с backoff

    // Server terminal (3500–3999) — reconnect: no
    InvalidToken        = 3500,  // токен невалиден, нужен новый
    TokenExpired        = 3501,  // токен истёк, нужен refresh
    Forbidden           = 3502,  // доступ запрещён
    BadRequest          = 3503,  // невалидные данные от клиента
    ConnectionLimit     = 3504,  // лимит соединений превышен
    SessionRevoked      = 3505,  // сессия принудительно закрыта (например logout с другого устройства)
    TooManyRequests     = 3506,  // rate limit на уровне соединения
    MessageSizeLimit    = 3507,  // клиент отправляет фреймы больше лимита
}

impl DisconnectCode {
    fn should_reconnect(&self) -> bool {
        let code = *self as u16;
        match code {
            0..=999     => true,
            3000..=3499 => true,
            3500..=3999 => false,
            4000..=4499 => true,
            4500..=4999 => false,
            _           => true,  // unknown → try reconnect
        }
    }
}
```

### Flow на клиенте

```rust
fn on_disconnect(&mut self, code: DisconnectCode, reason: &str) {
    if code.should_reconnect() {
        // Переподключаемся с backoff
        self.reconnect_loop().await;
    } else {
        // Сообщаем Dart — нужно действие пользователя
        self.post_event(Event::Disconnected {
            code,
            reason: reason.to_string(),
            can_reconnect: false,
        });
    }
}
```

### Graceful server shutdown

При `SIGTERM`:
1. Перестать принимать новые соединения
2. Отправить всем подключённым клиентам `DisconnectCode::ServerShutdown` (3000)
3. Дождаться завершения in-flight сообщений (timeout 10 сек)
4. Flush pending webhooks
5. Закрыть все WS соединения
6. Exit

Клиенты получают 3000 → `should_reconnect() = true` → переподключаются (к другой ноде при кластере).

---

## 38. Backpressure

### Серверный буфер

Каждая WS сессия имеет ограниченный буфер исходящих событий:

```rust
struct WsSession {
    // Буфер исходящих фреймов
    tx: mpsc::Sender<WsFrame>,  // bounded channel
    // ...
}

// При создании сессии
let (tx, rx) = mpsc::channel(256);  // макс 256 фреймов в буфере
```

### При переполнении

```rust
match session.tx.try_send(frame) {
    Ok(()) => {},
    Err(TrySendError::Full(_)) => {
        // Буфер полон — клиент не успевает
        // Отключаем с кодом BufferOverflow
        session.disconnect(DisconnectCode::BufferOverflow, "send buffer full").await;
    }
    Err(TrySendError::Closed(_)) => {
        // Соединение уже закрыто
        self.remove_session(session.id);
    }
}
```

Размер буфера конфигурируемый:

```toml
[server]
ws_send_buffer_size = 256  # фреймов
```

### Клиентская сторона

Клиент получает `DisconnectCode::BufferOverflow` (3004) → `should_reconnect() = true` (non-terminal). Переполнение буфера — transient проблема (медленная сеть, зависшее приложение). SDK переподключается с увеличенным backoff.

---

## 39. Presence (онлайн/оффлайн статусы)

### Модель

Presence определяется по `last_seen_at` — последнему времени активности (WS heartbeat, отправка сообщения, любое действие).

```rust
enum PresenceStatus {
    Online,       // WS соединение активно прямо сейчас
    RecentlyOnline, // last_seen < 5 минут назад
    Offline,      // last_seen >= 5 минут назад
}
```

### Серверная сторона

```sql
-- device_sessions уже имеет last_seen_at
UPDATE device_sessions SET last_seen_at = NOW() WHERE id = $1;
```

Free tier: `last_seen_at` в PostgreSQL.
Pro tier: Redis с TTL для fast lookup (`SET presence:{user_id} {node_id} EX 30`).

### Клиентская сторона — не через подписку, а явным запросом

Presence **не** push'ится автоматически через WS (слишком много трафика при большом количестве контактов).

Клиент запрашивает presence явно:

```rust
// При открытии списка чатов — запрос онлайн статусов для видимых чатов
FrameKind::GetPresence = 0x15  // request: [user_id, ...]
FrameKind::PresenceResult = 0x29  // response: [(user_id, last_seen_ts), ...]
```

### Оптимизации

- В 1-на-1 диалоге: опрос presence собеседника раз в 5 минут (не чаще)
- В групповом чате: опрос первых N видимых участников при открытии
- Сервер может push'ить presence изменения для участников активного (подписанного) чата через WS Event

### Privacy

Настройка в профиле пользователя: показывать ли `last_seen` другим. Если скрыто — сервер возвращает `null` в `last_seen_ts`.

---

## 40. Создание чатов и управление участниками

### Все операции — через WS RPC + дублирование в REST API

Операции доступны через три канала:
1. **WS RPC** — основной способ из клиентских SDK
2. **REST API** — для серверных администраторов
3. **Bot API** — для ботов (подмножество REST)

### WS фреймы управления

```rust
#[repr(u8)]
enum FrameKind {
    // ... существующие фреймы ...

    // Управление чатами (клиент → сервер, rpc с Ack)
    CreateChat      = 0x40,
    UpdateChat      = 0x41,  // название, аватар
    DeleteChat      = 0x42,
    GetChatInfo     = 0x43,
    GetChatMembers  = 0x44,

    // Управление участниками
    InviteMembers   = 0x45,
    KickMember      = 0x46,
    LeaveChat        = 0x47,
    UpdateMemberRole = 0x48,
    MuteMember      = 0x49,
    BanMember       = 0x4A,
}
```

### REST API (те же операции)

```
POST   /api/v1/chats                    → CreateChat
PATCH  /api/v1/chats/:id                → UpdateChat
DELETE /api/v1/chats/:id                → DeleteChat
GET    /api/v1/chats/:id                → GetChatInfo
GET    /api/v1/chats/:id/members        → GetChatMembers
POST   /api/v1/chats/:id/members        → InviteMembers
DELETE /api/v1/chats/:id/members/:uid   → KickMember
PATCH  /api/v1/chats/:id/members/:uid   → UpdateMemberRole
```

### Проверка прав

Каждая операция проверяет permission через bitwise:

```rust
async fn handle_kick_member(&self, actor: &ChatMember, target_user_id: i64) -> Result<()> {
    // Проверяем permission
    if !has_permission(actor, Permission::KICK_MEMBERS) {
        return Err(ErrorCode::Forbidden);
    }

    // Нельзя кикнуть пользователя с ролью >= своей
    let target = self.db.get_member(chat_id, target_user_id).await?;
    if target.role >= actor.role {
        return Err(ErrorCode::Forbidden);
    }

    self.db.remove_member(chat_id, target_user_id).await?;
    self.broadcast_event(chat_id, Event::MemberLeft { user_id: target_user_id }).await;
    Ok(())
}
```

---

## 41. Media upload/download

### Архитектура

Media загружается через **отдельный HTTP endpoint** (не через WS — бинарные данные плохо ложатся на WS фреймы). Хранение в S3-совместимом хранилище.

### Наличие в конфиге → доступность для клиента

```toml
[storage]
enabled = true
backend = "s3"  # или "local" для dev
s3_bucket = "chat-media"
s3_region = "us-east-1"
s3_endpoint = "https://s3.amazonaws.com"  # или MinIO URL
s3_access_key = "..."
s3_secret_key = "..."
max_file_size = 52428800      # 50MB
allowed_types = ["image/*", "video/*", "audio/*", "application/pdf"]
generate_thumbnails = true
thumbnail_max_dimension = 320
```

Если `storage.enabled = false` — `ServerLimits` в Welcome не содержит media capabilities, клиент скрывает кнопку аттача.

### Проверка доступности через bitwise

```rust
// ServerCapabilities передаётся в Welcome
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

// Клиент проверяет: конфиг сервера & права пользователя в чате
let can_upload = server_capabilities.contains(ServerCapabilities::MEDIA_UPLOAD)
    && has_permission(member, Permission::SEND_MEDIA);
```

### Upload flow

```
Клиент                              Сервер
  │                                    │
  │ POST /api/v1/upload               │
  │ Authorization: Bearer {token}     │
  │ Content-Type: multipart/form-data │
  │ { file, chat_id }            ───► │
  │                                    │  валидация прав
  │                                    │  валидация размера/типа
  │                                    │  upload в S3
  │                                    │  генерация thumbnail (если изображение)
  │ ◄── { file_id, url,              │
  │       thumbnail_url,              │
  │       size, mime_type }           │
  │                                    │
  │ WS: SendMessage                   │
  │ { chat_id, content: "",           │
  │   extra: { attachments: [         │
  │     { file_id, url, ... }         │
  │   ]}}                        ───► │
  │                                    │  сохраняет сообщение
  │ ◄── Event::MessageNew             │
```

Upload и отправка сообщения — два отдельных шага. Это позволяет:
- Показывать прогресс загрузки до отправки сообщения
- Прикреплять несколько файлов
- Переиспользовать загруженный файл (forward)

### Chunked upload для больших файлов

Для файлов > 5MB — chunked upload с resume:

```
POST /api/v1/upload/init     → { upload_id, chunk_size }
PUT  /api/v1/upload/{id}/{n} → upload chunk N
POST /api/v1/upload/{id}/complete → { file_id, url, ... }
```

При обрыве: `GET /api/v1/upload/{id}/status` → последний успешный chunk → resume.

---

## 42. Кодогенерация и будущая TS библиотека

### Подход: Rust structs как source of truth

Rust типы в `chat_protocol` — единственный источник правды. Кастомный скрипт (`cargo xtask codegen`) парсит Rust структуры и генерирует data-классы для Dart и TypeScript с `toJson`/`fromJson` и `toBytes`/`fromBytes`.

Не используем промежуточный формат (RON, protobuf) — Rust код и есть schema.

### Что генерируется

Из каждой Rust структуры помеченной `#[derive(Protocol)]` (или через атрибуты):

```rust
// chat_protocol/src/frames.rs — это source of truth

/// Отправка сообщения
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

Генерируется:

**Dart:**
```dart
class SendMessage {
  final int chatId;
  final String idempotencyKey;
  final String content;
  final Uint8List? richContent;
  final Map<String, Object?>? extra;
  final int? replyToId;

  Uint8List toBytes() { ... }
  factory SendMessage.fromBytes(Uint8List bytes) { ... }
  Map<String, Object?> toJson() { ... }
  factory SendMessage.fromJson(Map<String, Object?> json) { ... }
}
```

**TypeScript:**
```typescript
export interface SendMessage {
  chatId: bigint;
  idempotencyKey: string;
  content: string;
  richContent?: Uint8Array;
  extra?: Record<string, unknown>;
  replyToId?: bigint;
}

export function encodeSendMessage(msg: SendMessage): Uint8Array { ... }
export function decodeSendMessage(bytes: Uint8Array): SendMessage { ... }
```

### Генерация через xtask

```bash
cargo xtask codegen              # генерирует Dart + TS
cargo xtask codegen --lang dart  # только Dart
cargo xtask codegen --lang ts    # только TypeScript
```

### Маппинг типов

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

### Что это даёт

- Один источник правды — Rust код в `chat_protocol`
- Невозможен рассинхрон: добавил поле в Rust → `cargo xtask codegen` → Dart и TS обновлены
- `toBytes`/`fromBytes` генерируются по тому же wire формату что Rust использует
- `toJson`/`fromJson` для REST API и дебага

### TypeScript библиотека

Независимая TS библиотека — полноценный клиент для браузера и Node.js. WS транспорт напрямую к серверу (не через Rust). Flutter Web использует её через `dart:js_interop` + `extension type`. Для браузера — `SharedWorker` как транспорт для общего WS соединения между вкладками.

> **Примечание**: Derive-макрос `Protocol` и xtask codegen — второй этап. На старте Rust типы и Dart классы пишутся вручную. Кодогенерация добавляется когда количество типов делает ручную синхронизацию обременительной.

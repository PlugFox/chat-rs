# Client Integration Guide

## Overview

Клиенты подключаются к серверу напрямую через WebSocket, используя бинарный протокол из `chat_protocol`. Dart и TypeScript работают нативными WS-средствами без какой-либо Rust-прослойки для транспорта.

**Опциональный локальный кэш** (персистентные сообщения, outbox) — отдельный репозиторий `chat_client_rs`.

## Connection Lifecycle

### 1. Connect

Открыть WS-соединение к `wss://<host>/ws`.

### 2. Handshake

Немедленно после подключения отправить **Hello** фрейм:

```
kind=Hello, seq=0, payload: { protocol_version, sdk_version, platform, token (JWT), device_id (UUID) }
```

Сервер отвечает **Welcome**:

```
payload: { session_id, server_time, user_id (i64), ServerLimits, ServerCapabilities }
```

Или **Error** → закрыть соединение (см. [error-codes.md](error-codes.md)).

### 3. Subscribe

После Welcome подписаться на нужные чаты (`Subscribe` фрейм с `chat_id`). Сервер начнёт доставлять real-time события (`MessageNew`, `MessageEdited`, `ChatUpdated` и т.д.).

История сообщений не пушится автоматически — клиент запрашивает её сам через `LoadMessages` по мере необходимости (при открытии чата, скролле).

### 4. Keepalive

Отправлять **Ping** каждые N секунд (N задаётся через `ServerLimits.ping_interval_ms`). Если Pong не пришёл за timeout — переподключиться.

## Message Loading & Chunk-Based Cache

Сервер не пушит историю автоматически. Клиент управляет загрузкой сам:

- `LoadMessages` mode 0 (Paginate) — загрузить страницу: `anchor_id`, `direction`, `limit`
- `LoadMessages` mode 2 (Chunk) — загрузить/обновить чанк: `chunk_id`, `since_ts`
- `anchor_id = 0` — загрузить с самого нового

Подробнее о режимах: [protocol.md — LoadMessages](protocol.md#loadmessages-0x1a).

### Lazy Cache Invalidation

Основные принципы:

1. **Никакой истории обновлений на сервере.** Сервер хранит только текущее состояние сущностей.
2. **Локальный кэш доверяем только в рамках активного WS-соединения.** При disconnect/reconnect — все кэшированные данные считаются stale.
3. **Ленивая инвалидация.** Обновления запрашиваются только для того, что пользователь сейчас видит.

### Trust Model — HashSet of Trusted Chunks

Каждый чат хранит in-memory `Set<int>` доверенных chunk_id. Чанк добавляется в set только после явного подтверждения от сервера в рамках текущего соединения.

**On disconnect:**
```
trustedChunkIds.clear()
```

**On chat switch:**
```
trustedChunkIds.clear()   // prevents unbounded growth
```

**On scroll to a chunk:**
```
if (chunkId not in trustedChunkIds) {
    // max_updated_at computed from messages table:
    // SELECT COALESCE(MAX(updated_at), 0)
    // FROM messages WHERE chat_id = ? AND id >= chunkStart AND id < chunkStart + 64
    sinceTs = getLocalMaxUpdatedAt(chatId, chunkId)
    loadMessages(mode=Chunk, chunkId, sinceTs)
}
```

**On response received:**
```
upsertMessages(response.messages)
trustedChunkIds.add(chunkId)
```

Чанк запрашивается максимум один раз за соединение. При повторном скролле к доверенному чанку — запрос не отправляется.

Сообщения, пришедшие через WS-события (`MessageNew`, `MessageEdited`) в текущей сессии, автоматически доверены — они обновляют чанк в памяти без повторного запроса.

### Bootstrap on Reconnect

Ленивая синхронизация чанков не покрывает события, не связанные с видимым контентом. Сразу после подключения клиент отправляет один запрос:

`LoadChats` (mode 0, limit) — возвращает метаданные чатов: `last_message_preview`, `unread_count`, `updated_at`. **Не** включает содержимое сообщений.

| Событие                              | Механизм                                              |
| ------------------------------------- | ----------------------------------------------------- |
| Новое сообщение в закрытом чате       | WS push → обновить `unread_count` в списке чатов     |
| Добавлен/удалён из чата               | `LoadChats` при reconnect                             |
| Изменилось название/аватар чата       | `LoadChats` при reconnect                             |
| Сообщение отредактировано в открытом  | WS push → обновить в памяти, чанк уже доверен        |

Сами модели сообщений иммутабельны — trust state хранится снаружи.

## Seq Numbers

Каждый клиент ведёт монотонный счётчик `seq: u32`. Каждому исходящему фрейму присваивается следующий seq. Сервер отвечает `Ack(seq)` или `Error(seq)` — клиент матчит ответ по seq.

- `seq=0` — fire-and-forget (Typing, Ping, Unsubscribe): ответ не ожидается
- `seq > 0` — RPC: клиент ждёт `Ack(seq)` или `Error(seq)`

## Idempotency

Каждая персистентная команда (SendMessage, EditMessage, DeleteMessage) содержит `idempotency_key: UUID (16 bytes)`, генерируемый клиентом при первой постановке в очередь. Сервер хранит ключи 24 часа — повторная отправка вернёт тот же результат без дублирования.

`ReadReceipt` idempotency_key не нужен — сервер делает `upsert` по `(chat_id, user_id)`, повторная отправка при reconnect безопасна по природе.

## Reconnection

При разрыве соединения:

1. Если токен валиден — переподключиться с экспоненциальным backoff: 1, 2, 4, 8, 16, 30, 30... сек
2. Если токен истёк — обновить токен через приложение → переподключиться
3. После двух подряд `Unauthorized` — показать пользователю экран авторизации
4. После `Welcome` — переподписаться на все чаты; все ранее загруженные сообщения становятся dirty

## Token Refresh

При получении `token_expired` или перед переподключением с истёкшим токеном:

1. Клиент (Dart/TS) запрашивает новый токен у своего бэкенда
2. Reconnect с новым токеном в Hello

Логика обновления токена — полностью на стороне клиентского приложения.

## Outbox (offline send)

Клиент **не должен** дропать неотправленные сообщения при разрыве сети:

- Хранить очередь неотправленных сообщений локально (`chat_client_rs` или собственный механизм)
- При статусе `pending`: отображать пользователю "отправляется"
- При reconnect: немедленно повторить отправку всех pending сообщений
- При постоянной ошибке: пометить как `failed_permanent`, уведомить пользователя

**Классификация ошибок outbox:**

- **Permanent** (не ретраить): `forbidden`, `chat_not_found`, `not_chat_member`, `message_too_large`, `extra_too_large`, `content_filtered`
- **Transient** (ретраить): `internal_error`, `service_unavailable`, `rate_limited`, сетевые ошибки

## Cache Interface (`CacheProvider`)

SDK не зависит от конкретного хранилища — только от интерфейса `CacheProvider`.

### Операции

| Операция                                  | Stub           | Full (`chat_client_rs`) |
| ----------------------------------------- | -------------- | ----------------------- |
| `getMessages(chatId, anchor, dir, limit)` | `[]`           | SQLite cursor query     |
| `saveMessages(messages)`                  | no-op          | INSERT OR REPLACE       |
| `getChats(cursor, limit)`                 | `[]`           | SQLite cursor query     |
| `saveChat(chat)`                          | in-memory Map  | SQLite upsert           |
| `getReadReceipt(chatId)`                  | in-memory Map  | SQLite                  |
| `saveReadReceipt(chatId, messageId)`      | in-memory Map  | SQLite upsert           |
| `getOutbox()`                             | `[]`           | SQLite pending rows     |
| `addToOutbox(item)`                       | in-memory List | SQLite INSERT           |
| `removeFromOutbox(id)`                    | in-memory List | SQLite DELETE           |

**Stub** — реализация по умолчанию. Подходит для клиентов без offline-first требований и для тестов.

**Full** — `chat_client_rs` (отдельный репозиторий, SQLite, WAL mode).

### База данных per-user (Full реализация)

```
/data/app/databases/
├── chat_u123456.db
├── chat_u789012.db
```

На смену пользователя: flush outbox → close DB → close WS → open new DB → reset UI state.

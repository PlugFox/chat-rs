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

## Message Loading & Dirty State

Сервер не пушит историю автоматически. Клиент управляет загрузкой сам:

- `LoadMessages(chat_id, anchor_id, direction, limit)` — загрузить страницу сообщений
- `anchor_id = 0` — загрузить с самого нового
- Cursor-based: `load_older` → `anchor_id = min_id_на_экране`, `load_newer` → `anchor_id = max_id_на_экране`

### Dirty State

Сообщения из локального кэша считаются **dirty** по умолчанию — они могли быть отредактированы или удалены пока клиент был offline. При reconnect все сообщения снова становятся dirty.

Рекомендуемый подход на клиенте:

- Держать `Set<int>` "чистых" message_id per chat — сбрасывается при каждом reconnect
- Сообщения, пришедшие через WS (MessageNew, MessageEdited) в текущей сессии — сразу чистые, добавляются в Set
- Сообщения загруженные через `LoadMessages` — чистые после получения от сервера
- Сообщения из локального кэша, не попавшие в Set — dirty, будут обновлены при скролле к ним

Сами модели сообщений иммутабельны — dirty state хранится снаружи.

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

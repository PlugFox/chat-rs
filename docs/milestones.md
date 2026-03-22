# Roadmap & Milestones

> Каждый milestone — это законченная, тестируемая единица. Следующий milestone не начинается пока предыдущий не закрыт по всем критериям приёмки.

## Overview

```
M0  Protocol         ██░░░░░░░░░░░░░░  contract
M1  Core             ████░░░░░░░░░░░░  server + client skeleton
M2  Basic UX         ██████░░░░░░░░░░  outbox, receipts, typing
M3  Media            ████████░░░░░░░░  upload/download
M4  Push             ██████████░░░░░░  FCM/APNs
M5  Engagement       ████████████░░░░  threads, reactions
M6  Enterprise       ██████████████░░  roles, permissions
M7  Platform         ████████████████  webhooks, bots
M8  Scaling          ████████████████  Redis clustering
```

---

## M0: Protocol Crate

> Контракт между клиентом и сервером. Определяет 80% дальнейшей работы. Ошибка здесь — самая дорогая.

### Scope

`chat_protocol` — единственный crate, zero external runtime deps (только serde, bytes, thiserror, bitflags, uuid).

### Deliverables

#### Codec
- [ ] `encode_header()` / `decode_header()` — frame header
- [ ] Encode/decode для каждого frame payload (SendMessage, Ack, Error, Welcome, Hello, etc.)
- [ ] Message batch encode/decode (count + N messages)
- [ ] Rich content BLOB encode/decode (spans with offsets)
- [ ] String/bytes/Option wire format helpers (u32 len prefix)

#### Frame Payloads (structs)
- [ ] `HelloPayload` — protocol_version, sdk_version, platform, token, device_id
- [ ] `WelcomePayload` — session_id, server_time, user_id, missed_messages, limits
- [ ] `SendMessagePayload` — chat_id, idempotency_key, content, rich_content, extra, reply_to_id
- [ ] `AckPayload` — message_id (опционально другие поля в зависимости от команды)
- [ ] `ErrorPayload` — code, slug, message, retry_after_ms (опционально)
- [ ] Payload structs для всех остальных frame kinds

### Tests

- [ ] **Unit**: `FrameKind::from_u8()` roundtrip для всех вариантов
- [ ] **Unit**: `ErrorCode` slug stability, permanent/transient classification
- [ ] **Unit**: `Permission` bitflags операции, `default_permissions()` для каждой роли × chat kind
- [ ] **Unit**: `DisconnectCode::should_reconnect()` для каждого range
- [ ] **Proptest**: codec roundtrip — `encode(x) |> decode == x` для каждого payload type
- [ ] **Proptest**: message batch roundtrip с произвольным количеством сообщений (0..100)
- [ ] **Proptest**: rich content roundtrip с overlapping spans
- [ ] **Proptest**: String/bytes с edge cases (empty, max u32 length prefix, unicode)
- [ ] **Unit**: error на truncated input (каждый decode path)
- [ ] **Unit**: error на unknown frame kind
- [ ] **Unit**: error на frame exceeding max size

### Benchmarks

- [ ] `criterion` bench: encode/decode 1000 messages batch
- [ ] `criterion` bench: single frame header encode/decode (latency)
- [ ] `criterion` bench: rich content with 50 spans

### Documentation

- [ ] Rustdoc для каждого public type и function
- [ ] Module-level docs с wire format diagrams
- [ ] Обновить `docs/protocol.md` и `docs/codec.md` если API изменился

### Acceptance Criteria

- `cargo test -p chat_protocol` — все тесты зелёные
- `cargo clippy -p chat_protocol` — zero warnings
- Proptest с 10000 iterations без failures
- Benchmarks baseline записаны (для regression tracking в будущих milestone'ах)
- Каждый public item имеет rustdoc

---

## M1: Core

> Минимальный работающий стек: сервер принимает соединения, клиент подключается, сообщения доставляются. Главный критерий — integration test "Alice sends, Bob receives".

### Scope

`chat_protocol` (уже готов) и `chat_server`. Клиентский crate в этом репозитории не разрабатывается — он в отдельном репозитории `chat_client_rs`. Для integration тестов используется встроенный WS-клиент на основе `tokio-tungstenite` + `chat_protocol` codec прямо в `tests/`.

### Deliverables — Server

#### Infrastructure
- [ ] `main.rs` — tokio runtime, signal handling (SIGTERM graceful shutdown)
- [ ] `config.rs` — TOML config parsing с serde, validation, defaults
- [ ] `app.rs` — axum Router с middleware stack (tracing, timeout, CORS)
- [ ] PostgreSQL connection pool (sqlx::PgPool)
- [ ] Database migrations: `001_initial.sql` (users, chats, chat_members, messages, device_sessions, read_receipts, reactions, idempotency_keys)
- [ ] Tracing/logging setup (tracing-subscriber с env-filter)

#### WebSocket
- [ ] WS upgrade endpoint (`GET /ws`)
- [ ] Frame parsing loop (decode header → dispatch by kind)
- [ ] `Hello` → JWT verification → `Welcome` response
- [ ] Ping/Pong keepalive с configurable interval
- [ ] Session registry: `DashMap<(i64, Uuid), WsSession>` (user_id, device_id)
- [ ] Bounded send buffer per session (mpsc channel)
- [ ] Graceful disconnect с proper `DisconnectCode`

#### Message Handling
- [ ] `SendMessage` → validate → insert into DB → broadcast to subscribed sessions
- [ ] `Ack` response с server-assigned message_id
- [ ] Idempotency key deduplication (INSERT ... ON CONFLICT)
- [ ] Basic delivery: fan-out to all sessions subscribed to chat_id

#### Auth
- [ ] JWT verification (jsonwebtoken crate)
- [ ] User creation/lookup by external_id
- [ ] Device session upsert

### Deliverables — Test Client (in `tests/`)

Минимальный WS-клиент для integration тестов, живущий в `tests/helpers/` данного репозитория. Не является production библиотекой.

- [ ] `TestClient` struct: WS connect + Hello/Welcome handshake
- [ ] Encode/decode фреймов через `chat_protocol` codec
- [ ] Seq counter + pending request map (для rpc)
- [ ] `send_message()` — отправить `SendMessage` фрейм, дождаться `Ack`
- [ ] `recv_event()` — получить следующий `Event` фрейм с таймаутом
- [ ] `subscribe(chat_id, last_ts)` — подписаться на чат

### Tests

#### Unit — Server
- [ ] Config parsing: valid config, missing fields с defaults, invalid values
- [ ] JWT verification: valid token, expired, invalid signature, malformed
- [ ] Frame dispatch: known kinds route correctly, unknown → Error frame
- [ ] Idempotency: duplicate key returns same message_id

#### Integration
- [ ] **Two-client message delivery**: Alice и Bob (`TestClient`) подключаются к test server, Bob отправляет сообщение, Alice получает `MessageNew` event
- [ ] **Idempotency**: клиент отправляет дважды с одним key → одно сообщение на сервере
- [ ] **Graceful shutdown**: сервер отправляет `DisconnectCode::ServerShutdown` → клиент видит disconnect

#### Property-based
- [ ] Proptest: произвольные WS frame sequences не паникуют сервер

### Benchmarks

- [ ] Server: messages/sec throughput (1 sender, 1 receiver, in-memory)
- [ ] Server: connection setup latency (Hello → Welcome)

### Documentation

- [ ] `docs/server.md` — обновить с реальной структурой модулей
- [ ] `docs/database.md` — обновить с финальной схемой PostgreSQL
- [ ] Rustdoc для public API обоих crate'ов (`chat_protocol`, `chat_server`)
- [ ] `config.example.toml` — финальная версия с комментариями

### Acceptance Criteria

- Integration test "two clients" (через `TestClient`) зелёный
- Server стартует с config.example.toml, принимает WS соединения
- `TestClient` подключается, отправляет сообщение, получает Ack
- `cargo xtask check` — всё зелёное
- Benchmarks baseline записаны

---

## M2: Basic UX

> Чат становится usable: read receipts показывают кто прочитал, typing indicators показывают кто печатает, unread count обновляется в реальном времени.

### Scope

Расширение `chat_server` и `chat_protocol`. Клиентская реализация кэша (SQLite) — в `chat_client_rs`.

### Cache Interface

В этом milestone определяется интерфейс кэша (`CacheProvider`) для клиентской стороны. Конкретная реализация (SQLite) — в `chat_client_rs`. В данном репозитории только:

- **Stub-реализация** — для тестов и клиентов без персистентного кэша:
  - Bulk-операции (история сообщений, список чатов, outbox) → всегда возвращают пустой результат / игнорируют запись
  - Простые операции (last read receipt per chat, typing state) → in-memory `Map`

### Deliverables

#### Server: Idempotency & Deduplication
- [ ] Серверная дедупликация `idempotency_key` с TTL 24h + очистка по scheduled task

#### Server: Read Receipts
- [ ] `ReadReceipt` frame handler → upsert в `read_receipts`
- [ ] Broadcast `ReceiptUpdate` event участникам чата
- [ ] Batching: несколько read receipts в одном frame (debounce 500ms)

#### Server: Typing Indicators
- [ ] `Typing` frame handler → broadcast `TypingUpdate` (ephemeral, не persist)
- [ ] No "stop typing" frame — timeout-based (5 sec server-side)

#### Server: Chat List
- [ ] `LoadChats` frame → cursor-based query (`ORDER BY last_message_at DESC`)
- [ ] Response: chat entries с `last_message` preview, `unread_count`
- [ ] `ChatUpdated` event при новых сообщениях (bump last message + unread count)

#### Server: Connection State
- [ ] `DisconnectCode` с причиной в каждом сценарии
- [ ] Поведение клиента при каждом `DisconnectCode` задокументировано в `docs/error-codes.md`

#### Server: Subscriptions
- [ ] `Subscribe(chat_id)` → Ack; сессия добавляется в fan-out для чата
- [ ] `Unsubscribe(chat_id)` → сессия удаляется из fan-out
- [ ] Auto-resubscribe при reconnect — ответственность клиента

### Tests

- [ ] **Unit**: idempotency dedup на сервере (duplicate → same result)
- [ ] **Unit**: read receipt batching (multiple reads → one frame)
- [ ] **Unit**: typing timeout expiry (5 sec)
- [ ] **Unit**: chat list cursor pagination (empty, one page, multi-page)
- [ ] **Integration**: read receipt flow — Alice sends, Bob reads, Alice sees ReceiptUpdate
- [ ] **Integration**: typing indicator — Alice types, Bob sees TypingUpdate, Alice stops → idle after 5 sec
- [ ] **Integration**: ChatUpdated delivered after new message
- [ ] **Integration**: subscribe → disconnect → reconnect → resubscribe → new messages delivered

### Benchmarks

- [ ] Chat list query latency (1000 chats)
- [ ] Fan-out latency: message → N subscribed sessions (N = 10, 100, 1000)

### Documentation

- [ ] `docs/protocol.md` — обновить если payload structs изменились
- [ ] `docs/client.md` — typing debounce, read receipt batching

### Acceptance Criteria

- Read receipts видны в real-time (integration test)
- Typing indicators работают с server-side timeout (integration test)
- ChatUpdated доставляется при новом сообщении (integration test)
- Subscribe/Unsubscribe корректно управляют fan-out (unit test)
- `cargo xtask check` — всё зелёное

---

## M3: Media

> Чат без картинок — не чат. Upload через HTTP (не WS), S3-compatible storage, chunked upload для больших файлов.

### Deliverables

- [ ] Server: `POST /api/v1/upload` endpoint (multipart/form-data)
- [ ] Server: S3-compatible storage backend (config-driven: s3 / local filesystem)
- [ ] Server: file validation (size, MIME type — по конфигу)
- [ ] Server: thumbnail generation для изображений (configurable max dimension)
- [ ] Server: chunked upload flow (`/upload/init`, `/upload/{id}/{n}`, `/upload/{id}/complete`, `/upload/{id}/status`)
- [ ] Server: `ServerCapabilities::MEDIA_UPLOAD` flag в Welcome
- [ ] Client: upload через HTTP (не через WS)
- [ ] Client: progress tracking (bytes sent / total)
- [ ] Client: resume after interruption
- [ ] Client: LRU cache для downloaded media

### Tests

- [ ] **Unit**: file validation (size limits, MIME types)
- [ ] **Unit**: thumbnail dimensions calculation
- [ ] **Integration**: upload → send message with attachment → receiver gets URL
- [ ] **Integration**: chunked upload → interrupt → resume → complete
- [ ] **Integration**: upload with invalid token → 401
- [ ] **Integration**: upload exceeding max_file_size → 413

### Benchmarks

- [ ] Upload throughput (10MB file, local S3/MinIO)
- [ ] Chunked upload overhead vs single upload

### Acceptance Criteria

- Изображение загружается, thumbnail генерируется, получатель видит оба URL
- Chunked upload resume работает после обрыва
- `storage.enabled = false` → клиент не показывает кнопку аттача (capability check)

---

## M4: Push Notifications

> Без push нотификаций нет retention. Push отправляется только для пользователей не подписанных на чат через WS.

### Deliverables

- [ ] Server: FCM integration (HTTP v1 API)
- [ ] Server: APNs integration (token-based auth)
- [ ] Server: push token registration в `device_sessions.push_token`
- [ ] Server: delivery decision — WS first, push only if not subscribed
- [ ] Server: push payload formatting (title, body, badge count, data payload)
- [ ] Client: push token передача на сервер при connect

### Tests

- [ ] **Unit**: delivery decision logic (subscribed → no push, not subscribed → push)
- [ ] **Unit**: push payload formatting
- [ ] **Integration**: user offline → receives push (mock FCM/APNs endpoint)

### Acceptance Criteria

- Пользователь offline → получает push → открывает app → видит сообщение
- Пользователь online и подписан → push не отправляется

---

## M5: Engagement

> Threads и reactions делают чат интерактивным и пригодным для командной работы.

### Deliverables

#### Threads / Replies
- [ ] Server: `reply_to_id` в SendMessage → denormalized preview в response
- [ ] Server: `MessageNew` event содержит reply preview (sender, first N chars)
- [ ] Client: reply_to tracking в messages table

#### Reactions
- [ ] Server: `AddReaction` / `RemoveReaction` frames
- [ ] Server: reactions table (message_id, user_id, emoji) с unique constraint
- [ ] Server: broadcast `ReactionUpdate` event
- [ ] Client: reactions storage + aggregation (emoji → count + user list)

#### Message Editing
- [ ] Server: `EditMessage` frame → update content + updated_at, broadcast `MessageEdited`
- [ ] Server: edit window (configurable, e.g. 24 hours)
- [ ] Client: handle `MessageEdited` → update local storage

#### Message Deletion
- [ ] Server: `DeleteMessage` frame → soft delete (status = deleted), broadcast `MessageDeleted`
- [ ] Client: handle `MessageDeleted` → update status locally

### Tests

- [ ] **Unit**: reply preview generation (truncation, empty content)
- [ ] **Unit**: reaction aggregation (add, remove, duplicate)
- [ ] **Unit**: edit window enforcement
- [ ] **Integration**: full reply thread flow
- [ ] **Integration**: reaction add/remove → real-time update
- [ ] **Integration**: edit → receivers see updated content
- [ ] **Integration**: delete → receivers see "deleted" placeholder

### Acceptance Criteria

- Reply отображает preview родительского сообщения
- Reactions обновляются в real-time
- Edit меняет контент, delete показывает placeholder

---

## M6: Enterprise

> Ролевая модель, управление чатами и участниками — необходимо для B2B/enterprise use cases.

### Deliverables

#### Roles & Permissions
- [ ] Server: `ChatRole` hierarchy enforcement (Member < Moderator < Admin < Owner)
- [ ] Server: `Permission` bitflags check на каждой операции
- [ ] Server: `chat_members.permissions` override mechanism
- [ ] Server: configurable default permissions per role (через config.toml)

#### Chat Management
- [ ] Server: `CreateChat` (direct, group, channel) с creator = Owner
- [ ] Server: `UpdateChat` (title, avatar) с permission check
- [ ] Server: `DeleteChat` с Owner-only check
- [ ] Server: `GetChatInfo`, `GetChatMembers` frames

#### Member Management
- [ ] Server: `InviteMembers` с batch invite + permission check
- [ ] Server: `KickMember` с role hierarchy check (нельзя кикнуть ≥ своей роли)
- [ ] Server: `LeaveChat` (Owner не может уйти без transfer)
- [ ] Server: `UpdateMemberRole` с role hierarchy check
- [ ] Server: `MuteMember` (temporary permission override)
- [ ] Server: `BanMember` (permanent removal + block rejoin)

#### REST API
- [ ] Server: REST endpoints duplicating WS RPC (для admin panels)
- [ ] Server: Bearer token auth для REST

### Tests

- [ ] **Unit**: permission checks для каждой операции × каждой роли
- [ ] **Unit**: role hierarchy enforcement (нельзя кикнуть/мутить ≥ своей роли)
- [ ] **Unit**: Owner restrictions (cannot leave, only transfer)
- [ ] **Unit**: permission override: member с кастомными правами
- [ ] **Integration**: full chat lifecycle (create → invite → chat → kick → leave → delete)
- [ ] **Integration**: permission denied → correct error code
- [ ] **Proptest**: произвольные последовательности member operations не оставляют inconsistent state

### Benchmarks

- [ ] Permission check latency (hot path — every message)
- [ ] Chat creation latency with initial member batch

### Acceptance Criteria

- Все 4 роли работают с корректной иерархией
- Permission override позволяет fine-grained контроль
- REST API зеркалит WS RPC
- `has_permission()` — O(1), не запрос в БД на каждое сообщение

---

## M7: Platform

> SDK становится платформой: внешние системы могут реагировать на события, модерировать контент и автоматизировать через ботов.

### Deliverables

#### Webhooks (async)
- [ ] Server: webhook event queue (in-memory или Redis)
- [ ] Server: HTTP POST с HMAC-SHA256 signature
- [ ] Server: configurable event filters
- [ ] Server: retry с exponential backoff (3 attempts)
- [ ] Server: deduplication по event UUID

#### Interceptors (sync)
- [ ] Server: pre-save interceptor endpoint (HTTP call)
- [ ] Server: 500ms timeout → Allow by default
- [ ] Server: `InterceptAction::Allow | Modify | Reject`
- [ ] Server: bypass для бот-сообщений (избежать loop)

#### Bot API
- [ ] Server: REST API (`/bot/v1/sendMessage`, `/bot/v1/answerCallback`, etc.)
- [ ] Server: bot token authentication
- [ ] Server: `users.is_bot = true` flag
- [ ] Server: event delivery через webhook или long polling

### Tests

- [ ] **Unit**: HMAC signature generation и verification
- [ ] **Unit**: interceptor timeout → Allow fallback
- [ ] **Unit**: webhook retry logic
- [ ] **Integration**: webhook receives event after message send
- [ ] **Integration**: interceptor rejects message → sender gets ContentFiltered error
- [ ] **Integration**: interceptor modifies message → receiver gets modified content
- [ ] **Integration**: bot sends message via REST → delivered to chat members

### Acceptance Criteria

- Webhook'и доставляются с корректной подписью
- Interceptor может заблокировать сообщение
- Bot API позволяет отправлять и получать сообщения

---

## M8: Scaling

> Multi-node deployment через Redis. Free tier остаётся single-node.

### Deliverables

#### Redis Integration
- [ ] Server: `ClusterBackend` trait implementation для Redis
- [ ] Server: Redis Pub/Sub для cross-node message delivery
- [ ] Server: Redis presence (SET с TTL + heartbeat)
- [ ] Server: Redis distributed rate limiting (Lua GCRA script)

#### License
- [ ] Server: JWT-based license verification (offline, no callback)
- [ ] Server: feature gating по license tier (webhooks, bots, clustering)

#### Operational
- [ ] Server: health check endpoint (`GET /health`)
- [ ] Server: readiness probe (`GET /ready` — includes DB и Redis connectivity)
- [ ] Server: metrics endpoint (Prometheus format)
- [ ] Server: configurable logging levels per module

### Tests

- [ ] **Integration**: два сервера + Redis → сообщение доставлено через cross-node
- [ ] **Integration**: node shutdown → clients reconnect to other node
- [ ] **Integration**: presence consistency across nodes
- [ ] **Load test**: 1000 concurrent connections, 100 msg/sec sustained

### Benchmarks

- [ ] Cross-node delivery latency (sender → Redis → receiver node → client)
- [ ] Redis rate limiter throughput
- [ ] Connection scaling: latency at 100, 500, 1000, 5000 connections

### Acceptance Criteria

- Два сервера за load balancer доставляют сообщения cross-node
- Rate limiting работает distributed
- Graceful shutdown одной ноды не теряет сообщения
- Load test: 1000 connections × 100 msg/sec × 10 min — zero message loss

---

## Cross-Cutting Concerns (every milestone)

### Quality Gates (apply to every milestone)

Каждый milestone должен пройти перед закрытием:

| Gate                  | Requirement                                                     |
| --------------------- | --------------------------------------------------------------- |
| **Compilation**       | `cargo check --workspace` — zero errors                         |
| **Linting**           | `cargo clippy --workspace -- -D warnings` — zero warnings       |
| **Formatting**        | `cargo fmt --all --check` — no diff                             |
| **Unit tests**        | `cargo test --workspace` — all pass                             |
| **Integration tests** | Все integration tests milestone'а — pass                        |
| **Proptest**          | 10000+ iterations без failures                                  |
| **Documentation**     | Rustdoc для public API, docs/ files обновлены                   |
| **Benchmarks**        | Baseline записан (или regression check vs предыдущий milestone) |
| **No regressions**    | Тесты предыдущих milestone'ов всё ещё зелёные                   |

### Testing Strategy

| Тип                   | Где                                    | Что покрывает                               |
| --------------------- | -------------------------------------- | ------------------------------------------- |
| Unit (`#[cfg(test)]`) | Каждый модуль                          | Чистая логика, edge cases, error paths      |
| Proptest              | `chat_protocol` codec, outbox ordering | Invariants over random inputs               |
| Integration           | `tests/` директория                    | Multi-component scenarios (client ↔ server) |
| Load test             | Отдельный binary / script              | Throughput, latency under load              |
| Benchmark             | `benches/` (criterion)                 | Performance regression tracking             |

### Documentation Lifecycle

1. **Before implementation**: прочитать docs/ для затрагиваемой области
2. **During**: обновлять если API изменился
3. **After**: проверить что docs соответствуют реальности
4. **Rustdoc**: каждый public type/function/module

### Performance Tracking

Benchmarks записываются при закрытии каждого milestone. При следующем milestone — regression check:
- Codec throughput не должен деградировать
- Message delivery latency не должна расти более чем на 10%
- SQLite query latency — отслеживаем при росте schema complexity

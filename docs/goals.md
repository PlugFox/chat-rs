# Project Goals

## Vision

Chat SDK — кроссплатформенный SDK для чата, позиционируемый как альтернатива Sendbird / Stream Chat с возможностью self-hosted и SaaS.

## Core Principles

### 1. Rust-first (server side)

Вся серверная бизнес-логика, транспорт и хранение данных на сервере — в Rust. Клиентские платформы (Flutter/Dart, TypeScript) подключаются к серверу напрямую через WebSocket, используя протокол из `chat_protocol`. Локальный кэш и хранение сообщений на клиенте вынесены в отдельный Rust-репозиторий, используемый платформами по необходимости.

Это даёт:
- Высокопроизводительный сервер с низкой латентностью
- Единый wire-протокол для всех платформ
- Dart и TypeScript клиенты с минимальной логикой, фокус на UI и интеграцию

### 2. Offline-first

Сообщения отправляются через персистентный outbox в SQLite. Отправка переживает перезапуск приложения, потерю сети, смену Wi-Fi/LTE. Пользователь набрал сообщение в метро → открыл приложение дома → сообщение доставлено автоматически.

### 3. SDK, not an app

Это библиотека для сторонних разработчиков, не готовое приложение. Каждое архитектурное решение оценивается через призму: "сможет ли разработчик с нашим SDK построить свой чат за день?". Расширяемость через:
- `extra` JSON поля на сообщениях (кастомные метаданные)
- Webhooks и Interceptors на сервере
- Bot API для автоматизации
- Полная темизация и замена UI компонентов на Flutter

### 4. Performance at scale

Целевые метрики:
- 100k+ сообщений в чате без деградации UI
- Cursor-based пагинация (O(log n), не OFFSET)
- Батчинг WS сообщений (до 20 событий за кадр)
- WAL mode SQLite с read/write split для concurrent access

### 5. Minimal surface, maximum power

WS протокол — один бинарный формат с 6-байтным заголовком. Простота контракта при богатой функциональности за ним. REST API для медиа-загрузки и Bot API. Никаких лишних слоёв между клиентом и сервером.

## Target Platforms

| Tier            | Платформа                        | Транспорт                         | Локальное хранилище                       |
| --------------- | -------------------------------- | --------------------------------- | ----------------------------------------- |
| **1 (day-one)** | Android, iOS                     | Dart → WebSocket (напрямую)       | `chat_client_rs` (отдельный репо) via FFI |
| **1 (day-one)** | Linux, macOS, Windows            | Dart → WebSocket (напрямую)       | `chat_client_rs` (отдельный репо) via FFI |
| **2 (planned)** | Web (Flutter / React)            | TypeScript → WebSocket (напрямую) | IndexedDB / без персистентности           |
| **3 (future)**  | React Native, native iOS/Android | Native WS                         | `chat_client_rs` via FFI                  |

## Target Users

1. **Mobile/desktop app developers** — встраивают чат в своё приложение через Flutter/Dart SDK (WS напрямую + опционально `chat_client_rs` для локального кэша)
2. **Web/TS developers** — подключают TS-клиент к серверу через WebSocket
3. **Backend developers** — self-host сервер, интегрируют через webhooks и Bot API
4. **Enterprise** — кастомные роли, права, модерация, compliance

## Non-Goals

| Что                     | Почему                                                                    |
| ----------------------- | ------------------------------------------------------------------------- |
| Voice / Video calls     | Отдельный домен (WebRTC), другая архитектура                              |
| End-to-end encryption   | Фундаментально меняет всю архитектуру (key exchange, device verification) |
| Stickers / GIF search   | Контент-специфичная фича, реализуется через `extra` поля                  |
| Link previews           | Через `extra` + сервис на стороне разработчика                            |
| Email/SMS notifications | Через webhooks на стороне разработчика                                    |
| User profile management | SDK управляет чатами, не профилями. Профили — на стороне разработчика     |

## Success Criteria

### Pre-alpha (Milestone 0–1)
- Два Rust-клиента обмениваются сообщениями через сервер в integration test
- Codec roundtrip тесты на proptest
- Сервер стартует, принимает WS соединения, обрабатывает Hello/Welcome

### Alpha (Milestone 2–3)
- Flutter demo app с отправкой/получением сообщений (Dart → WS напрямую)
- Offline отправка через outbox (в отдельном `chat_client_rs` репо)
- Media upload работает
- Документация достаточна для внешнего разработчика

### Beta (Milestone 4–6)
- Push уведомления
- Threads, reactions
- Полная ролевая модель
- Нагрузочные тесты: 1000 concurrent connections, 100 msg/sec

### 1.0 (Milestone 7–8)
- Webhooks + Bot API
- Redis clustering
- pub.dev пакеты Dart-клиента опубликованы
- npm пакет TypeScript-клиента опубликован
- Security audit пройден

# Cross-Platform Notes

## Client Architecture

Клиенты подключаются к серверу напрямую через WebSocket — нет Rust FFI прослойки для транспорта. Каждая платформа использует нативные WS-средства:

| Platform                        | WS transport                       | Local cache                            |
| ------------------------------- | ---------------------------------- | -------------------------------------- |
| Flutter (Android, iOS, Desktop) | `web_socket_channel` / native      | `chat_client_rs` via FFI (опционально) |
| TypeScript (Web)                | `WebSocket` API / `ws` npm package | IndexedDB или без персистентности      |
| TypeScript (Node.js)            | `ws` npm package                   | —                                      |

## Dart / Flutter Client

- Транспорт: `web_socket_channel` или нативный WS
- Протокол: бинарные фреймы `chat_protocol` (Dart-decoder написан вручную или через codegen)
- Локальный кэш: `chat_client_rs` (отдельный репозиторий) via `dart:ffi`
- Темизация: `InheritedWidget` (`ChatTheme`)
- Кастомизация UI: `ChatComponents` — типизированные builder callbacks

### Build Notes (для `chat_client_rs`, отдельный репо)

| Platform | Tooling                                                                   |
| -------- | ------------------------------------------------------------------------- |
| Android  | `cargo-ndk` for arm64-v8a, armeabi-v7a, x86_64                            |
| iOS      | Universal binary (XCFramework): aarch64-apple-ios + aarch64-apple-ios-sim |
| Desktop  | Native cargo                                                              |

> Артефакты (`libchat.so`, `libchat.a`, etc.) относятся к `chat_client_rs`, не к этому репозиторию.

## TypeScript Client

Независимая TS-библиотека, реализующая:
- WS-подключение и handshake (Hello/Welcome)
- Codec: encode/decode бинарных фреймов `chat_protocol`
- Seq numbering и RPC correlation table
- Reconnection с exponential backoff
- SharedWorker для общего WS-соединения между вкладками браузера

Flutter Web использует эту библиотеку через `extension type` + `dart:js_interop`.

**Do not implement WASM Rust client for web.** TypeScript покрывает все web use cases нативно.

## Distribution

```
pub.dev (open):
├── chat_platform_interface   ← abstract types
└── chat_flutter              ← widgets

npm (open):
└── @chat-sdk/client-ts       ← TypeScript client

GitHub Releases / S3 (бинари chat_client_rs — отдельный репо):
└── v1.0.2/
    ├── libchat-android-arm64.so
    ├── libchat-android-armv7.so
    ├── libchat-android-x64.so
    ├── libchat-ios.a
    ├── libchat-macos.dylib
    ├── chat-windows.dll
    └── libchat-linux.so
```

Native Assets hook (`hook/build.dart`) скачивает бинарь `chat_client_rs` при `flutter build`, не при `pub get`. SHA256 checksum верификация.

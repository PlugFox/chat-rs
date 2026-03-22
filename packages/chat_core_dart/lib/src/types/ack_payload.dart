// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'dart:typed_data';

/// Ack payload — command-specific response data.
///
/// The variant is determined by the `FrameKind` of the original request.
/// Some variants carry raw bytes that must be decoded with the appropriate
/// codec function (e.g. `decode_message_batch` for `MessageBatch`).
/// This is intentional: the codec layer does not track which request
/// generated the Ack, so the caller provides the context.
sealed class AckPayload {
  const AckPayload();
}

/// Empty ack (Subscribe, UpdateMember, Leave, etc.).
class AckEmpty extends AckPayload {
  const AckEmpty();
}

/// SendMessage ack: server-assigned message ID.
class AckMessageId extends AckPayload {
  const AckMessageId({
    required this.value,
  });

  final int value;
}

/// CreateChat ack: server-assigned chat ID.
class AckChatId extends AckPayload {
  const AckChatId({
    required this.value,
  });

  final int value;
}

/// LoadMessages: message batch (raw bytes, decode with `decode_message_batch`).
class AckMessageBatch extends AckPayload {
  const AckMessageBatch({
    required this.value,
  });

  final Uint8List value;
}

/// LoadChats: next cursor + chat entries (raw bytes).
class AckChatList extends AckPayload {
  const AckChatList({
    required this.value,
  });

  final Uint8List value;
}

/// GetChatInfo: single chat entry (raw bytes).
class AckChatInfo extends AckPayload {
  const AckChatInfo({
    required this.value,
  });

  final Uint8List value;
}

/// GetChatMembers: member list (raw bytes).
class AckMemberList extends AckPayload {
  const AckMemberList({
    required this.value,
  });

  final Uint8List value;
}

/// Search results (raw bytes).
class AckSearchResults extends AckPayload {
  const AckSearchResults({
    required this.value,
  });

  final Uint8List value;
}

/// GetUser: single user entry (raw bytes, decode with `decode_user_entry`).
class AckUserInfo extends AckPayload {
  const AckUserInfo({
    required this.value,
  });

  final Uint8List value;
}

/// GetUsers: user entries list (raw bytes).
class AckUserList extends AckPayload {
  const AckUserList({
    required this.value,
  });

  final Uint8List value;
}

/// GetBlockList: blocked user IDs (raw bytes).
class AckBlockList extends AckPayload {
  const AckBlockList({
    required this.value,
  });

  final Uint8List value;
}

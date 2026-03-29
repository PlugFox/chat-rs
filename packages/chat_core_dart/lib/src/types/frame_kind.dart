// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Frame type identifier — first byte of every WS binary frame.
///
/// Values are stable and must never be renumbered.
enum FrameKind {
  /// Client → server: protocol version, token, device_id.
  hello(1),

  /// Server → client: session_id, server_time, limits.
  welcome(2),

  /// Keepalive ping (both directions).
  ping(3),

  /// Keepalive pong (both directions).
  pong(4),

  /// Refresh JWT token without disconnecting (client → server).
  refreshToken(5),

  /// Send a new message (persist, needs Ack).
  sendMessage(16),

  /// Edit an existing message (persist, needs Ack).
  editMessage(17),

  /// Soft-delete a message (persist, needs Ack).
  deleteMessage(18),

  /// Mark messages as read (persist, fire-and-forget).
  readReceipt(19),

  /// Typing indicator (ephemeral, fire-and-forget).
  typing(20),

  /// Request online/offline status (RPC).
  getPresence(21),

  /// Load chat list (RPC).
  loadChats(22),

  /// Full-text message search (RPC).
  search(23),

  /// Subscribe to real-time events for a chat (RPC).
  subscribe(24),

  /// Unsubscribe from a chat (fire-and-forget).
  unsubscribe(25),

  /// Load message history (RPC).
  loadMessages(26),

  /// Add a reaction to a message (needs Ack).
  addReaction(27),

  /// Remove a reaction from a message (needs Ack).
  removeReaction(28),

  /// Pin a message in a chat (needs Ack).
  pinMessage(29),

  /// Unpin a message in a chat (needs Ack).
  unpinMessage(30),

  /// Forward a message to another chat (persist, needs Ack).
  forwardMessage(31),

  /// New message delivered in real-time. Payload: single `Message`.
  messageNew(32),

  /// Message content changed. Payload: single `Message` with updated fields.
  messageEdited(33),

  /// Message marked deleted. Payload: `chat_id: u32, message_id: u32`.
  messageDeleted(34),

  /// Read receipt update.
  receiptUpdate(35),

  /// Typing indicator broadcast.
  typingUpdate(36),

  /// Member joined chat.
  memberJoined(37),

  /// Member left chat.
  memberLeft(38),

  /// Response to GetPresence.
  presenceResult(39),

  /// Chat metadata changed (title, avatar). Payload: full `ChatEntry`.
  chatUpdated(40),

  /// New chat the user is a member of. Payload: full `ChatEntry`.
  chatCreated(41),

  /// Reaction added or removed on a message.
  reactionUpdate(42),

  /// User profile changed (server → client push).
  userUpdated(43),

  /// Chat was deleted (server → client push).
  chatDeleted(44),

  /// Chat member's role or permissions changed (server → client push).
  memberUpdated(45),

  /// Command acknowledged.
  ack(48),

  /// Error response.
  error(49),

  /// Create a new chat.
  createChat(64),

  /// Update chat info (title, avatar).
  updateChat(65),

  /// Delete a chat.
  deleteChat(66),

  /// Get chat details.
  getChatInfo(67),

  /// List chat members.
  getChatMembers(68),

  /// Invite users to a chat.
  inviteMembers(69),

  /// Kick, ban, mute, change role, or update permissions for a member.
  updateMember(70),

  /// Leave a chat.
  leaveChat(71),

  /// Mute chat notifications (client → server, RPC).
  muteChat(72),

  /// Unmute chat notifications (client → server, RPC).
  unmuteChat(73),

  /// Get a single user's profile.
  getUser(80),

  /// Get multiple users' profiles.
  getUsers(81),

  /// Update own profile.
  updateProfile(82),

  /// Block a user.
  blockUser(83),

  /// Unblock a user.
  unblockUser(84),

  /// Get block list.
  getBlockList(85);

  const FrameKind(this.value);
  final int value;

  static FrameKind? fromValue(int value) => switch (value) {
    1 => hello,
    2 => welcome,
    3 => ping,
    4 => pong,
    5 => refreshToken,
    16 => sendMessage,
    17 => editMessage,
    18 => deleteMessage,
    19 => readReceipt,
    20 => typing,
    21 => getPresence,
    22 => loadChats,
    23 => search,
    24 => subscribe,
    25 => unsubscribe,
    26 => loadMessages,
    27 => addReaction,
    28 => removeReaction,
    29 => pinMessage,
    30 => unpinMessage,
    31 => forwardMessage,
    32 => messageNew,
    33 => messageEdited,
    34 => messageDeleted,
    35 => receiptUpdate,
    36 => typingUpdate,
    37 => memberJoined,
    38 => memberLeft,
    39 => presenceResult,
    40 => chatUpdated,
    41 => chatCreated,
    42 => reactionUpdate,
    43 => userUpdated,
    44 => chatDeleted,
    45 => memberUpdated,
    48 => ack,
    49 => error,
    64 => createChat,
    65 => updateChat,
    66 => deleteChat,
    67 => getChatInfo,
    68 => getChatMembers,
    69 => inviteMembers,
    70 => updateMember,
    71 => leaveChat,
    72 => muteChat,
    73 => unmuteChat,
    80 => getUser,
    81 => getUsers,
    82 => updateProfile,
    83 => blockUser,
    84 => unblockUser,
    85 => getBlockList,
    _ => null,
  };
}

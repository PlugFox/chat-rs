// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

/// Rich text style flags (u16 on wire).
///
/// Inline styles are freely combinable. Block-level styles (`CODE_BLOCK`,
/// `BLOCKQUOTE`) have special semantics — see docs/messages.md.
extension type const RichStyle(int value) implements int {
  /// Bold text.
  static const RichStyle bold = RichStyle(0x0001);

  /// Italic text.
  static const RichStyle italic = RichStyle(0x0002);

  /// Underlined text.
  static const RichStyle underline = RichStyle(0x0004);

  /// Strikethrough text.
  static const RichStyle strike = RichStyle(0x0008);

  /// Spoiler text (hidden until tapped/clicked).
  static const RichStyle spoiler = RichStyle(0x0010);

  /// Inline monospace code.
  static const RichStyle code = RichStyle(0x0020);

  /// Hyperlink. Meta: `{"url": "..."}`.
  static const RichStyle link = RichStyle(0x0040);

  /// User mention. Meta: `{"user_id": u32}`.
  static const RichStyle mention = RichStyle(0x0080);

  /// Colored text. Meta: `{"rgba": u32}`.
  static const RichStyle color = RichStyle(0x0100);

  /// Fenced code block. Meta: `{"lang": "rust"}`.
  /// When set, client ignores inline style bits on this span.
  static const RichStyle codeBlock = RichStyle(0x0200);

  /// Block quote (`>` prefixed text).
  static const RichStyle blockquote = RichStyle(0x0400);

  static const List<RichStyle> values = [
    bold,
    italic,
    underline,
    strike,
    spoiler,
    code,
    link,
    mention,
    color,
    codeBlock,
    blockquote,
  ];

  bool contains(RichStyle flag) => (value & flag.value) != 0;
  RichStyle add(RichStyle flag) => RichStyle(value | flag.value);
  RichStyle remove(RichStyle flag) => RichStyle(value & ~flag.value);
  RichStyle toggle(RichStyle flag) => RichStyle(value ^ flag.value);
  bool get isEmpty => value == 0;
  bool get isNotEmpty => value != 0;
  RichStyle operator ^(RichStyle other) => RichStyle(value ^ other.value);
}

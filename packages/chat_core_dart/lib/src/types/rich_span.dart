// GENERATED CODE — DO NOT MODIFY BY HAND
// Source: chat_protocol

import 'package:chat_core/src/types/rich_style.dart';

/// A rich text span — a styled range within the plain-text content.
///
/// Wire format: 10 bytes fixed (start: u32, end: u32, style: u16)
/// + meta_len: u32 + optional JSON meta.
class RichSpan {
  const RichSpan({
    required this.start,
    required this.end,
    required this.style,
    this.meta,
  });

  /// Start byte offset into the plain-text content (inclusive).
  final int start;

  /// End byte offset into the plain-text content (exclusive).
  final int end;

  /// Style flags for this span.
  final RichStyle style;

  /// Optional JSON metadata. `None` when no meta-bearing style bits are set.
  final String? meta;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is RichSpan &&
          start == other.start &&
          end == other.end &&
          style == other.style &&
          meta == other.meta;

  @override
  int get hashCode => Object.hash(start, end, style, meta);

  @override
  String toString() =>
      'RichSpan(start: $start, end: $end, style: $style, meta: $meta)';
}

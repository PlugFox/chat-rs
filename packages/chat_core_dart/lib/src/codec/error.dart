class CodecError implements Exception {
  const CodecError(this.message);
  final String message;
  @override
  String toString() => 'CodecError: $message';
}

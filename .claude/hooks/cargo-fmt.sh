#!/bin/bash
# Post-edit hook: format Rust files after Edit/Write
# Receives tool result JSON on stdin

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Only run on .rs files
if [[ "$FILE_PATH" == *.rs ]]; then
  cargo fmt --quiet 2>/dev/null || true
fi

exit 0

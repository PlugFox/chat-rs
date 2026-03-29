#!/bin/bash
# Pre-commit hook: verify workspace compiles before committing
# Receives tool input JSON on stdin

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only intercept git commit commands
if echo "$COMMAND" | grep -qE '^git\s+commit'; then
  OUTPUT=$(cargo check --workspace --quiet 2>&1)
  if [ $? -ne 0 ]; then
    jq -n --arg reason "cargo check failed — fix compilation errors before committing:
$OUTPUT" '{
      decision: "block",
      reason: $reason
    }'
    exit 2
  fi
fi

exit 0

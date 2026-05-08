#!/usr/bin/env bash
# Hook: session-compact
# Triggered when context window is approaching its limit.
# Generates a compact summary of current session state for handoff.

set -euo pipefail

PROJECT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
cd "$PROJECT_ROOT"

TIMESTAMP=$(date -u +%Y%m%d_%H%M%S)
OUTPUT_DIR="production/session-state"
mkdir -p "$OUTPUT_DIR"

COMPACT_FILE="$OUTPUT_DIR/compact_${TIMESTAMP}.md"

cat > "$COMPACT_FILE" << 'HEADER'
# Session Compact Summary

Auto-generated when context window approached capacity.
Use this file with the `resume` skill to restore session context.

HEADER

{
  echo "## Git State"
  echo ""
  echo "- Branch: $(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'unknown')"
  echo "- Last commit: $(git log --oneline -1 2>/dev/null || echo 'none')"
  echo ""

  DIRTY=$(git status --porcelain 2>/dev/null)
  if [[ -n "$DIRTY" ]]; then
    echo "### Uncommitted Changes"
    echo '```'
    echo "$DIRTY"
    echo '```'
    echo ""
  fi

  echo "### Recent Commits (this branch)"
  echo '```'
  git log --oneline -10 2>/dev/null || echo "none"
  echo '```'
  echo ""

  # Capture any active sprint contract
  if [[ -f "$OUTPUT_DIR/active-contract.md" ]]; then
    echo "## Active Sprint Contract"
    echo ""
    cat "$OUTPUT_DIR/active-contract.md"
    echo ""
  fi

  echo "## Files Changed in Session"
  echo '```'
  git diff --name-only HEAD~5..HEAD 2>/dev/null || echo "none"
  echo '```'

} >> "$COMPACT_FILE"

echo "Compact summary written to: $COMPACT_FILE"

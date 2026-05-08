#!/usr/bin/env bash
# Hook: secret-detection
# Runs before commits to detect accidentally committed secrets.
# Exit 1 to block the commit if secrets are found.

set -euo pipefail

STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null || true)

if [[ -z "$STAGED_FILES" ]]; then
  exit 0
fi

SECRET_PATTERNS=(
  'AKIA[0-9A-Z]{16}'                    # AWS Access Key
  'sk-[a-zA-Z0-9]{20,}'                 # OpenAI / Stripe secret key
  'ghp_[a-zA-Z0-9]{36}'                 # GitHub personal access token
  'gho_[a-zA-Z0-9]{36}'                 # GitHub OAuth token
  'github_pat_[a-zA-Z0-9_]{22,}'        # GitHub fine-grained PAT
  'xox[bpors]-[a-zA-Z0-9-]+'            # Slack token
  'sk_live_[a-zA-Z0-9]+'                # Stripe live key
  'rk_live_[a-zA-Z0-9]+'                # Stripe restricted key
  'SG\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+' # SendGrid API key
  'AIza[0-9A-Za-z_-]{35}'               # Google API key
  'ya29\.[0-9A-Za-z_-]+'                # Google OAuth token
  'eyJ[a-zA-Z0-9_-]*\.eyJ'              # JWT token (header.payload)
  'BEGIN (RSA |EC |DSA |OPENSSH )?PRIVATE KEY' # Private keys
  'password\s*=\s*["\x27][^"\x27]{8,}'  # Hardcoded passwords
)

COMBINED_PATTERN=$(IFS='|'; echo "${SECRET_PATTERNS[*]}")
FOUND=0

for FILE in $STAGED_FILES; do
  if [[ ! -f "$FILE" ]]; then
    continue
  fi

  # Skip binary files and lock files
  if file "$FILE" | grep -q 'binary\|executable'; then
    continue
  fi
  if [[ "$FILE" == *.lock || "$FILE" == "Cargo.lock" ]]; then
    continue
  fi

  MATCHES=$(grep -nEo "$COMBINED_PATTERN" "$FILE" 2>/dev/null || true)
  if [[ -n "$MATCHES" ]]; then
    echo "⚠️  Potential secret detected in $FILE:"
    echo "$MATCHES" | head -5
    FOUND=1
  fi
done

if [[ $FOUND -eq 1 ]]; then
  echo ""
  echo "❌ Commit blocked: potential secrets detected."
  echo "   If these are false positives, review and use --no-verify to bypass."
  exit 1
fi

exit 0

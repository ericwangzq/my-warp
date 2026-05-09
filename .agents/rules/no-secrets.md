# Rule: No Secrets

**Never hardcode secrets, tokens, API keys, or credentials in source code.**

## What Counts as a Secret

- API keys and tokens (AWS, GitHub, Stripe, OpenAI, Google, etc.)
- Passwords and passphrases
- Private keys (RSA, EC, SSH)
- JWT tokens
- Database connection strings with credentials
- OAuth client secrets
- Webhook signing keys

## Rules

1. **Never commit secrets** to any file in this repository
2. **Use environment variables** for local development secrets
3. **Use the platform's secret management** for production secrets
4. **Never log secrets** — even in debug or trace-level logging
5. **Never include secrets in error messages** or user-facing output

## Detection

The `secret-detection` hook (`.agents/hooks/secret-detection.sh`) runs before commits and scans for common secret patterns. If it blocks your commit and the match is a false positive, review carefully before bypassing.

## If You Find a Secret

1. **Do not commit it** — if already committed, rotate the secret immediately
2. Notify the security team
3. Use `git filter-branch` or BFG Repo-Cleaner to remove from history if pushed

## Source

OWASP Top 10: A07:2021 — Identification and Authentication Failures

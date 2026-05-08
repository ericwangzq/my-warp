# Rule: Exhaustive Matching

When writing or editing `match` statements in Rust, **always list all variants explicitly**. Do not use the wildcard `_` catch-all pattern.

## Why

Exhaustive matching ensures that when new variants are added to an enum, the compiler forces all match sites to be updated. Wildcard patterns silently swallow new variants, leading to bugs that are hard to find.

## Examples

```rust
// ❌ BAD — new variants will be silently ignored
match action {
    Action::Open => handle_open(),
    Action::Close => handle_close(),
    _ => {}
}

// ✅ GOOD — compiler will error when new variants are added
match action {
    Action::Open => handle_open(),
    Action::Close => handle_close(),
    Action::Save => handle_save(),
    Action::Delete => handle_delete(),
}
```

## Exceptions

The wildcard pattern is acceptable only when:
- Matching on primitive types (`i32`, `char`, etc.) where exhaustive listing is impossible
- Matching on external types where listing all variants is impractical and the catch-all behavior is intentional
- The match is in test code and the untested variants are irrelevant to the test

In all other cases, list every variant.

## Source

WARP.md § Exhaustive Matching

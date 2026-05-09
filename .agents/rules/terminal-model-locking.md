# Rule: Terminal Model Locking

When working with `TerminalModel`, **never nest `model.lock()` calls**. Acquiring multiple locks on the same model from different call sites causes a deadlock, resulting in a UI freeze (beach ball on macOS).

## Rules

1. **Before adding a new `model.lock()` call**, verify that no caller in the current call stack already holds the lock
2. **Pass already-locked model references** down the call stack rather than acquiring new locks
3. **Keep lock scopes as short as possible** and avoid calling other functions that might also attempt to lock
4. **Never call `model.lock()` inside a closure** passed to a method that already holds the lock

## Examples

```rust
// ❌ BAD — nested locks will deadlock
fn update_terminal(model: &ModelHandle<TerminalModel>) {
    let locked = model.lock();
    // ... some work ...
    self.helper(model); // helper also calls model.lock() → DEADLOCK
}

// ✅ GOOD — pass the locked reference
fn update_terminal(model: &ModelHandle<TerminalModel>) {
    let locked = model.lock();
    self.helper(&locked); // pass the already-locked reference
}
```

## Verification

When reviewing code that touches `TerminalModel`:
1. Search for all `model.lock()` calls in the changed files
2. Trace the call stack upward to check for existing locks
3. If in doubt, grep for `.lock()` in the same module

## Source

WARP.md § Terminal Model Locking

# Rule: UI Component Reuse

When writing UI code in Warp, **always reuse existing components and themes**. Do not create feature-specific variants of shared components.

## Button Themes

Use the existing `ActionButtonTheme` implementations: `PrimaryTheme`, `SecondaryTheme`, `NakedTheme`, `DangerPrimaryTheme`, etc.

**Red flags:**
- Creating a new `impl ActionButtonTheme for FooTheme` that wraps an existing theme with one tweak
- Hard-coding `ColorU::new(...)` instead of using `appearance.theme()` accessors
- Setting `should_opt_out_of_contrast_adjustment` to `true` to force a specific label color
- Naming a theme after a feature (`BarSubmitTheme`) instead of a design-system role

If no existing theme fits, surface the gap to the user before creating a new one.

## MouseStateHandle

`MouseStateHandle` must be created **once during view construction**, then referenced/cloned wherever mouse input tracking is needed.

```rust
// ❌ BAD — inline default during render breaks all mouse interactions
fn render(&self, ctx: &mut ViewContext) -> Element {
    Button::new("click", MouseStateHandle::default()) // NO
}

// ✅ GOOD — created once in constructor, cloned in render
fn new() -> Self {
    Self {
        mouse_state: MouseStateHandle::default(),
    }
}
fn render(&self, ctx: &mut ViewContext) -> Element {
    Button::new("click", self.mouse_state.clone()) // YES
}
```

## General Principle

Before creating any new UI abstraction:
1. Search `crates/ui_components/` and `app/src/view_components/` for existing implementations
2. If something similar exists, use it
3. If it doesn't quite fit, check if the existing component should be extended (ask first)
4. Only create new components when nothing suitable exists

## Source

`.agents/skills/warp-ui-guidelines/SKILL.md`

# diagnostic/ssr-panic-message

Improved panic diagnostic for the tachys SSR feature unification bug.

**Branch:** [`DrudgeDance/leptos@fix/improve-ssr-panic-diagnostic`](https://github.com/DrudgeDance/leptos/tree/fix/improve-ssr-panic-diagnostic)

## What it does

Replaces 8 opaque `.expect()` messages across `event.rs`, `directive.rs`, and `property.rs` with a shared `FEATURE_CONFLICT_DIAGNOSTIC` constant that tells you exactly what's wrong and how to fix it.

## Before / after

**Before:**
```
panicked at tachys/src/html/event.rs:190:30: callback removed before attaching
```

**After:**
```
panicked at tachys/src/html/event.rs:190:30: Value is None because the `ssr`
feature is active. When `ssr` is enabled, tachys skips creating client-side
values (event handlers, directives, properties) to avoid cross-thread panics
on multithreaded servers. If you are building a client-side (CSR or hydrate)
target, this means the `ssr` feature is being activated unintentionally
via Cargo feature unification; another dependency in your workspace is
enabling it. Run `cargo tree -e features -i tachys` to identify the source.
```

## Quick test

```bash
cd ssr-panic-message && trunk serve --port 8103
# → page panics with the improved diagnostic
```

This app patches `tachys` (and the rest of the leptos workspace) from the fork branch to pick up the improved messages.

## Background

Follow-up to the bug reproduction in [`leptos/fix/always-create-client-values`](../fix/always-create-client-values/). The original PR ([#4587](https://github.com/leptos-rs/leptos/pull/4587)) was closed after maintainer review. Only the diagnostic improvement was accepted: no constructor changes (thread safety), no `compile_error!` (breaks rust-analyzer).

## Links

| Resource | URL |
|---|---|
| Fork branch | [`fix/improve-ssr-panic-diagnostic`](https://github.com/DrudgeDance/leptos/tree/fix/improve-ssr-panic-diagnostic) |
| Issue | [#4586](https://github.com/leptos-rs/leptos/issues/4586) |
| PR | [#4588](https://github.com/leptos-rs/leptos/pull/4588) |
| Closed PR | [~~#4587~~](https://github.com/leptos-rs/leptos/pull/4587) |
| Bug reproduction | [`leptos/fix/always-create-client-values`](../fix/always-create-client-values/) |

## Deprecated branches

| Branch | Why deprecated |
|---|---|
| [`deprecated/fix/always-create-client-values`](https://github.com/DrudgeDance/leptos/tree/deprecated/fix/always-create-client-values) | Constructor changes cause `SendWrapper` panics on multithreaded servers; `compile_error!` breaks rust-analyzer |
| [`deprecated/fix/defensive-plus-rebuild`](https://github.com/DrudgeDance/leptos/tree/deprecated/fix/defensive-plus-rebuild) | Masked the bug with defensive None-handling |
| [`deprecated/fix/non-optional-cb`](https://github.com/DrudgeDance/leptos/tree/deprecated/fix/non-optional-cb) | Overly complex cfg-gated struct, missed directive.rs and property.rs |

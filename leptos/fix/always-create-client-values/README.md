# fix/always-create-client-values

**Bug:** Cargo feature unification silently poisons CSR apps when any dependency
enables `leptos/ssr`, causing all `on:click`, `use:*`, and `prop:*` handlers to
panic at runtime.

**Fix branch:** [`DrudgeDance/leptos@fix/always-create-client-values`](https://github.com/DrudgeDance/leptos/tree/fix/always-create-client-values)

## Structure

```
issues/
  clean/    — Stock Leptos 0.8.16 CSR. Everything works.        → trunk serve --port 8100
  broken/   — Same app + tachys/ssr feature. Panics at load.    → trunk serve --port 8101
fix/
              — Same app + tachys/ssr + patched fork. Works.     → trunk serve --port 8102
```

All three share the **same `src/main.rs`** — a counter app with event handlers,
input bindings, and conditional views. Only the `Cargo.toml` differs.

## Root Cause

In `tachys/src/html/event.rs`, the `on()` constructor uses a compile-time check:

```rust
cb: (!cfg!(feature = "ssr")).then(|| SendWrapper::new(cb))
```

When `cfg!(feature = "ssr")` is true, `cb = None` — the callback is never stored.
Later, `On::attach()` panics with `.expect("callback removed before attaching")`.

The same pattern exists in `directive.rs` and `property.rs`.

## The Fix (5 commits)

| Commit | Description |
|---|---|
| [`2d535dd`](https://github.com/DrudgeDance/leptos/commit/2d535dd) | Always create values: `Some(SendWrapper::new(cb))` |
| [`0bdac8e`](https://github.com/DrudgeDance/leptos/commit/0bdac8e) | Add `compile_error!` guards for csr+ssr, csr+hydrate, ssr+hydrate |
| [`e17c37a`](https://github.com/DrudgeDance/leptos/commit/e17c37a) | Replace opaque `.expect()` messages with shared DRY diagnostic |
| [`409e30b`](https://github.com/DrudgeDance/leptos/commit/409e30b) | 15 WASM regression tests (build/hydrate/rebuild/to_html × 3 types) |
| [`624cdfd`](https://github.com/DrudgeDance/leptos/commit/624cdfd) | CI workflow for feature-safety verification |

## Quick Test

```bash
# 1. Clean — works
cd issues/clean && trunk serve --port 8100
# → Click counter works ✅

# 2. Broken — panics
cd issues/broken && trunk serve --port 8101
# → Blank page, console: "callback removed before attaching" ❌

# 3. Fixed — works despite SSR feature
cd fix && trunk serve --port 8102
# → Click counter works ✅
```

## Real-World Impact

[Cargo feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification)
means if **any** dependency in your workspace enables `leptos/ssr`, the `ssr`
feature is activated for **all** crates. Known culprits:
- `radix-leptos v0.9.0` (enables `leptos/ssr` unconditionally)
- Any SSR-focused crate alongside a CSR app in the same workspace

## Environment

- Leptos: 0.8.16
- Tachys: 0.2.12 (crates.io)
- Target: `wasm32-unknown-unknown` (CSR/WASM)
- Trunk: 0.21.x

## Links

| Resource | URL |
|---|---|
| **Fix branch** | [`DrudgeDance/leptos@fix/always-create-client-values`](https://github.com/DrudgeDance/leptos/tree/fix/always-create-client-values) |
| **Upstream repo** | [`leptos-rs/leptos`](https://github.com/leptos-rs/leptos) |
| **GitHub issue** | [Draft ready to file](github-issue.md) |

## Deprecated Branches

Previous fix attempts preserved under `deprecated/` prefix on the fork:

| Branch | Why deprecated |
|---|---|
| [`deprecated/fix/defensive-plus-rebuild`](https://github.com/DrudgeDance/leptos/tree/deprecated/fix/defensive-plus-rebuild) | Masked the bug with defensive None-handling in `attach()` |
| [`deprecated/fix/non-optional-cb`](https://github.com/DrudgeDance/leptos/tree/deprecated/fix/non-optional-cb) | Overly complex cfg-gated struct approach, missed directive.rs and property.rs |

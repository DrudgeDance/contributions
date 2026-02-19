<!--
GitHub issue for: https://github.com/leptos-rs/leptos
Labels: bug, tachys
-->

**Describe the bug**

When the `ssr` feature is activated on `tachys` — even in a CSR-only WASM app — all `on:click`, `use:*`, and `prop:*` handlers silently become `None` and panic at runtime. The `on()`, `directive()`, and `prop()` constructors conditionally skip value creation based on `cfg!(feature = "ssr")`, so when the feature is active via Cargo feature unification, client-side values are never created.

The practical trigger is Cargo feature unification: any workspace dependency that enables `leptos/ssr` (e.g., `radix-leptos v0.9.0`) poisons the entire workspace, activating `tachys/ssr` for CSR apps too.

Runtime panics:

```
panicked at tachys/src/html/event.rs:190:30: callback removed before attaching
panicked at tachys/src/html/directive.rs:122:30: directive removed early
panicked at tachys/src/html/property.rs:81:30: property removed early
```

**Leptos Dependencies**

```toml
leptos = { version = "0.8.16", features = ["csr"] }
console_error_panic_hook = "0.1"

# THIS SINGLE LINE CAUSES THE BUG:
# Simulates what happens when any workspace dependency enables ssr
tachys = { version = "0.2", features = ["ssr"] }
```

**To Reproduce**

1. Create a CSR Leptos app with any `on:click` handler
2. Add `tachys = { version = "0.2", features = ["ssr"] }` to `Cargo.toml` (simulating feature unification from another workspace crate)
3. Run `trunk serve`
4. Page is blank; console shows `panicked at tachys/src/html/event.rs:190:30: callback removed before attaching`

Standalone reproduction with 3 variants (clean baseline, broken, and fixed): [DrudgeDance/contributions](https://github.com/DrudgeDance/contributions/tree/main/leptos/fix/always-create-client-values)

**Root cause**

Three constructor functions use `cfg!(feature = "ssr")` to decide whether to create values:

`tachys/src/html/event.rs:116` — `on()`:
```rust
cb: (!cfg!(feature = "ssr")).then(|| SendWrapper::new(cb)),
//   ^^^ when ssr feature is active: (!true) → false → .then() = None
```

Same pattern in `directive.rs:50` and `property.rs:25`.

The SSR rendering path (`to_html()`) never accesses these values — it writes nothing for events, directives, and properties. So skipping creation is an unnecessary optimization that breaks CSR apps when `ssr` is activated by feature unification.

**Why this affects real apps**

[Cargo feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification) means if any dependency in a workspace enables `leptos/ssr`, the `ssr` feature is activated for all crates. Known real-world triggers:

- `radix-leptos v0.9.0` — unconditionally enables `leptos/ssr`
- Any library that depends on `leptos` with `features = ["ssr"]` in a mixed workspace

The DX is very poor: no compile-time error, no runtime warning before the panic, and the error message ("callback removed before attaching") gives no hint about the SSR feature being the cause.

**Next Steps**

- [x] I will make a PR

PR branch ready: [`DrudgeDance/leptos@fix/always-create-client-values`](https://github.com/DrudgeDance/leptos/tree/fix/always-create-client-values)

**Additional context**

- Leptos: 0.8.16, Tachys: 0.2.12
- Target: `wasm32-unknown-unknown` (CSR/WASM via Trunk)
- Rust: stable
- Originally discovered when `radix-leptos v0.9.0` activated `leptos/ssr` via feature unification in a CSR workspace

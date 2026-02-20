use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

/// Counter app that exercises event handlers, input bindings, and conditional
/// AnyView — all three break when tachys/ssr is active.
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let (text, set_text) = signal(String::from("Hello"));

    view! {
        <div style="padding: 40px; font-family: system-ui, -apple-system, sans-serif; max-width: 600px; margin: 0 auto;">
            <h1 style="color: #1a1a2e;">"Tachys SSR Feature Safety"</h1>

            <p style="color: #666; line-height: 1.6;">
                "This app exercises all three tachys construct types affected by the "
                "SSR feature unification bug. If you can see this page and the controls "
                "work, the build is healthy."
            </p>

            // Event handler (on:click)
            <section style="margin: 24px 0; padding: 16px; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2 style="margin-top: 0; color: #2196f3;">"1. Event Handler (on:click)"</h2>
                <p style="font-size: 14px; color: #888;">
                    "Panics with: \"callback removed before attaching\" when SSR is active."
                </p>
                <button
                    on:click=move |_| set_count.update(|n| *n += 1)
                    style="padding: 10px 20px; font-size: 16px; background: #2196f3; color: white; border: none; border-radius: 6px; cursor: pointer;"
                >
                    {move || format!("Clicked {} times", count.get())}
                </button>
            </section>

            // Input binding (on:input)
            <section style="margin: 24px 0; padding: 16px; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2 style="margin-top: 0; color: #4caf50;">"2. Reactive Input"</h2>
                <p style="font-size: 14px; color: #888;">
                    "Tests on:input event handler. Type below — the echo should update."
                </p>
                <input
                    type="text"
                    on:input=move |ev| {
                        use leptos::prelude::event_target_value;
                        set_text.set(event_target_value(&ev));
                    }
                    style="padding: 8px 12px; font-size: 16px; border: 2px solid #4caf50; border-radius: 6px; width: 300px;"
                />
                <p style="margin-top: 8px;">
                    "Echo: " <strong>{move || text.get()}</strong>
                </p>
            </section>

            // Conditional AnyView
            <section style="margin: 24px 0; padding: 16px; border: 1px solid #e0e0e0; border-radius: 8px;">
                <h2 style="margin-top: 0; color: #ff9800;">"3. Conditional View (AnyView pattern)"</h2>
                <p style="font-size: 14px; color: #888;">
                    "The button below appears/disappears based on click count. "
                    "This exercises the AnyView type-switching path."
                </p>
                {move || if count.get() > 2 {
                    view! {
                        <button
                            on:click=move |_| set_count.set(0)
                            style="padding: 10px 20px; font-size: 16px; background: #ff9800; color: white; border: none; border-radius: 6px; cursor: pointer;"
                        >
                            "Reset counter (appeared at count > 2)"
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <span style="color: #999; font-style: italic;">
                            {move || format!("Click {} more times to reveal reset button", 3 - count.get())}
                        </span>
                    }.into_any()
                }}
            </section>


            <footer style="margin-top: 32px; padding-top: 16px; border-top: 1px solid #e0e0e0; font-size: 12px; color: #aaa;">
                "Leptos 0.8.16 · tachys 0.2.12 · CSR mode · "
                <a href="https://github.com/DrudgeDance/leptos/tree/fix/always-create-client-values"
                   style="color: #2196f3;"
                   target="_blank"
                >
                    "fix/always-create-client-values"
                </a>
            </footer>
        </div>
    }
}

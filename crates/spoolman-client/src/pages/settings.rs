use leptos::*;
use stylers::style;
use crate::api;

#[component]
pub fn SettingsPage() -> impl IntoView {
    let settings = create_resource(|| (), |_| async { api::fetch_settings().await });
    let currency = create_rw_signal(String::new());
    let saved = create_rw_signal(false);
    let error = create_rw_signal(Option::<String>::None);

    create_effect(move |_| {
        if let Some(Ok(s)) = settings.get() {
            currency.set(s.get("currency_symbol").cloned().unwrap_or_else(|| "€".into()));
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            match api::put_setting("currency_symbol", currency.get()).await {
                Ok(_) => saved.set(true),
                Err(e) => error.set(Some(e.to_string())),
            }
        });
    };

    let class_name = style! {"SettingsPage",
        div.settings-page form {
            display: flex;
            flex-direction: column;
            max-width: 360px;
            gap: 0.5rem;
        }
        p.success {
            padding: 10px 14px;
            background: #edfaed;
            color: #1a7a1a;
            border: 1px solid #b3e6b3;
            border-radius: var(--radius);
            margin-bottom: 0.5rem;
            font-size: 0.9rem;
        }
    };

    view! { class = class_name,
        <div class="page settings-page">
            <h1>"Settings"</h1>
            {move || error.get().map(|e| view! { <p class="error">{e}</p> })}
            {move || saved.get().then(|| view! { <p class="success ">"Saved."</p> })}
            <form on:submit=on_submit>
                <label>
                    "Currency symbol"
                    <input type="text" maxlength="4"
                        prop:value=move || currency.get()
                        on:input=move |ev| {
                            saved.set(false);
                            currency.set(event_target_value(&ev));
                        }
                    />
                </label>
                <button type="submit" class="btn btn-primary ">"Save"</button>
            </form>
        </div>
    }
}

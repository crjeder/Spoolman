use leptos::*;
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

    view! {
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

use leptos::*;
use stylers::style;

/// Column header with sort indicator and optional text filter.
#[component]
pub fn ColHeader(
    label: &'static str,
    field: &'static str,
    sort_field: RwSignal<String>,
    sort_asc: RwSignal<bool>,
    #[prop(optional)] filter: Option<RwSignal<String>>,
) -> impl IntoView {
    let is_active = move || sort_field.get() == field;
    let indicator = move || {
        if is_active() {
            if sort_asc.get() { " ↑" } else { " ↓" }
        } else {
            ""
        }
    };
    let toggle_sort = move |_| {
        if sort_field.get() == field {
            sort_asc.update(|a| *a = !*a);
        } else {
            sort_field.set(field.to_string());
            sort_asc.set(true);
        }
    };

    let class_name = style! {"ColHeader",
        th.col-header {
            padding: 8px 12px;
            text-align: left;
            background: var(--sidebar-bg);
            border-bottom: 2px solid var(--border);
            white-space: nowrap;
            font-weight: 600;
            font-size: 0.875rem;
            color: var(--fg);
        }
        th.col-header.active {
            color: var(--accent);
        }
        button.sort-btn {
            background: none;
            border: none;
            padding: 0;
            cursor: pointer;
            font-weight: inherit;
            font-size: inherit;
            color: inherit;
        }
        button.sort-btn:hover {
            text-decoration: underline;
        }
        input.col-filter {
            display: block;
            margin-top: 4px;
            padding: 3px 6px;
            width: 100%;
            font-size: 0.8rem;
            border: 1px solid var(--border);
            border-radius: var(--radius);
            background: var(--bg);
            color: var(--fg);
        }
    };

    view! { class = class_name,
        <th class=move || if is_active() { "col-header active" } else { "col-header" }>
            <button class="sort-btn" on:click=toggle_sort>
                {label}{indicator}
            </button>
            {filter.map(|f| view! {
                <input
                    class="col-filter"
                    type="text"
                    placeholder="filter…"
                    prop:value=move || f.get()
                    on:input=move |ev| f.set(event_target_value(&ev))
                />
            })}
        </th>
    }
}

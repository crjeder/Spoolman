use leptos::*;
use stylers::style;

#[component]
pub fn Pagination(
    page: RwSignal<usize>,
    page_size: RwSignal<usize>,
    total: Signal<usize>,
) -> impl IntoView {
    let total_pages = move || {
        let ps = page_size.get();
        if ps == 0 { 1 } else { (total.get() + ps - 1) / ps }
    };

    let next_disabled = Signal::derive(move || page.get() + 1 >= total_pages());

    let class_name = style! {"Pagination",
        div.pagination {
            display: flex;
            align-items: center;
            gap: 8px;
            padding: 0.75rem 0;
            font-size: 0.875rem;
        }
        div.pagination button {
            padding: 4px 10px;
            border: 1px solid var(--border);
            border-radius: var(--radius);
            background: var(--bg);
            color: var(--fg);
            cursor: pointer;
        }
        div.pagination button:hover:not(:disabled) {
            background: var(--row-hover);
        }
        div.pagination button:disabled {
            opacity: 0.4;
            cursor: not-allowed;
        }
        div.pagination span {
            color: var(--muted);
        }
        div.pagination select {
            padding: 4px 6px;
            border: 1px solid var(--border);
            border-radius: var(--radius);
            background: var(--bg);
            color: var(--fg);
        }
    };

    view! { class = class_name,
        <div class="pagination">
            <button
                disabled=move || page.get() == 0
                on:click=move |_| page.update(|p| *p = p.saturating_sub(1))
            >"← Prev"</button>
            <span>{move || format!("Page {} / {}", page.get() + 1, total_pages())}</span>
            <button
                disabled=next_disabled
                on:click=move |_| page.update(|p| *p += 1)
            >"Next →"</button>
            <select
                on:change=move |ev| {
                    let v = event_target_value(&ev).parse::<usize>().unwrap_or(25);
                    page_size.set(v);
                    page.set(0);
                }
            >
                <option value="10"  selected=move || page_size.get() == 10>"10 / page"</option>
                <option value="25"  selected=move || page_size.get() == 25>"25 / page"</option>
                <option value="50"  selected=move || page_size.get() == 50>"50 / page"</option>
                <option value="100" selected=move || page_size.get() == 100>"100 / page"</option>
            </select>
        </div>
    }
}

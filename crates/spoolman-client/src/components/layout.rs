use leptos::*;
use leptos_router::{use_location, A};
use stylers::style;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let dark = use_context::<RwSignal<bool>>().expect("dark mode signal");

    // Apply or remove the `dark` class on <body> whenever the signal changes.
    create_effect(move |_| {
        let body = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body());
        if let Some(body) = body {
            if dark.get() {
                let _ = body.class_list().add_1("dark");
            } else {
                let _ = body.class_list().remove_1("dark");
            }
            // Persist preference.
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
            {
                let _ = storage.set_item("dark_mode", if dark.get() { "true" } else { "false" });
            }
        }
    });

    let class_name = style! {"Layout",
        div.app-shell {
            display: flex;
            flex-direction: row;
            height: 100vh;
            overflow: hidden;
            background: var(--bg);
            color: var(--fg);
        }
        main.main-content {
            flex: 1;
            overflow-y: auto;
            padding: 0;
            min-width: 0;
            background: var(--bg);
        }
    };

    view! { class = class_name,
        <div class="app-shell">
            <Sidebar />
            <main class="main-content">
                {children()}
            </main>
        </div>
    }
}

#[component]
fn Sidebar() -> impl IntoView {
    let dark = use_context::<RwSignal<bool>>().expect("dark mode signal");
    let location = use_location();
    let spools_active = move || {
        let path = location.pathname.get();
        path == "/" || path.starts_with("/spools")
    };

    let class_name = style! {"Sidebar",
        nav.sidebar {
            display: flex;
            flex-direction: column;
            width: 200px;
            min-width: 200px;
            background: var(--sidebar-bg);
            color: var(--sidebar-fg);
            border-right: 1px solid var(--border);
            overflow-y: auto;
        }
        div.sidebar-header {
            padding: 1rem;
            border-bottom: 1px solid var(--border);
        }
        span.logo {
            font-size: 1.1rem;
            font-weight: 700;
            letter-spacing: 0.02em;
            color: var(--accent);
        }
        ul.nav-links {
            list-style: none;
            margin: 0;
            padding: 0.5rem 0;
            flex: 1;
        }
        .nav-links li {
            margin: 0;
        }
        .nav-links li a {
            display: block;
            padding: 8px 1rem;
            color: var(--sidebar-fg);
            font-size: 0.9rem;
            text-decoration: none;
            border-left: 3px solid transparent;
            transition: background 0.1s;
        }
        .nav-links li a:hover {
            background: var(--row-hover);
            text-decoration: none;
        }
        .nav-links li.active a {
            color: var(--accent);
            border-left-color: var(--accent);
            background: var(--row-even);
            font-weight: 600;
        }
        div.sidebar-footer {
            padding: 0.75rem 1rem;
            border-top: 1px solid var(--border);
            display: flex;
            flex-direction: column;
            gap: 6px;
        }
        button.dark-toggle {
            background: transparent;
            border: 1px solid var(--border);
            border-radius: var(--radius);
            padding: 4px 8px;
            color: var(--sidebar-fg);
            cursor: pointer;
            font-size: 0.8rem;
            width: 100%;
            text-align: left;
        }
        button.dark-toggle:hover {
            background: var(--row-hover);
        }
        span.version {
            font-size: 0.7rem;
            color: var(--muted);
        }
    };

    view! { class = class_name,
        <nav class="sidebar">
            <div class="sidebar-header">
                <span class="logo">"Spoolman"</span>
            </div>
            <ul class="nav-links">
                <li class=move || if spools_active() { "active" } else { "" }><A href="/spools">"Spools"</A></li>
                <li><A href="/filaments">"Filaments"</A></li>
                <li><A href="/locations">"Locations"</A></li>
                <li><A href="/settings">"Settings"</A></li>
                <li><A href="/help">"Help"</A></li>
            </ul>
            <div class="sidebar-footer">
                <button
                    class="dark-toggle"
                    on:click=move |_| dark.update(|d| *d = !*d)
                >
                    {move || if dark.get() { "☀ Light" } else { "☾ Dark" }}
                </button>
                <span class="version">{format!("v{}", env!("CARGO_PKG_VERSION"))}</span>
            </div>
        </nav>
    }
}

## Context

`SpoolList` holds its filters in component-local `RwSignal`s (`ts.filter`, `material_filter`, `location_filter`, `color_level`, `color_pick`, `show_archived`). Leptos re-creates route components on navigation, so every signal resets to its default. `TableState` already persists sort/page/page_size to `localStorage` via per-key `Effect`s but explicitly keeps `filter` session-only in memory (`state.rs:139`). `FilamentList` has the same text-filter problem.

Constraint: client is WASM, no server round-trip wanted for UI state. `web_sys::Storage` is already used in `state.rs`.

## Goals / Non-Goals

**Goals:**
- Spool + filament list filters survive in-app navigation and tab reload.
- Filters reset when the tab session ends.
- One "Clear filters" button per list, visible only when a filter is active.

**Non-Goals:**
- Persisting filters across tab sessions or devices (explicitly not wanted).
- Persisting per-column visibility or any non-filter table state beyond what exists.
- Sharing filter state via URL query params.

## Decisions

### Decision: `sessionStorage`, not `localStorage`
`sessionStorage` is scoped to the tab session and cleared on close — exactly "until session end". `localStorage` would leak filters into every future visit. Alternative (in-memory context provider hoisted above the router) rejected: does not survive reload and adds a context for one screen's state.

### Decision: Add a generic session get/set helper pair in `state.rs`, mirroring the existing `storage_get`/`storage_set`
`session_get(key)` / `session_set(key, value)` using `w.session_storage()`. Keeps the storage-access pattern in one module. Keys namespaced `filter.spools.*` and `filter.filaments.*`.

### Decision: Extend `use_table_state` to load/persist `filter` from session storage
`filter` currently starts empty with a "session-only" comment. Change it to `RwSignal::new(session_get("filter.<ns>.text").unwrap_or_default())` plus an `Effect` writing back. This covers the text search for both lists with no per-page code.

### Decision: Spool-specific filters persisted inline in `SpoolList`
`material_filter`, `location_filter`, `color_level`, `color_pick`, `show_archived` are not part of `TableState` and only exist on the spool list. Initialise each from `session_get` and add one `Effect` each to write back. Six short blocks, no new abstraction — a `FilterState` struct for a single call site would be premature.

### Decision: "Clear filters" button derives visibility from an `any_active` closure
`move || !ts.filter.get().is_empty() || !material_filter.get().is_empty() || location_filter.get().is_some() || color_level.get() != "off" || show_archived.get()`. Click handler sets each signal back to default; the existing persist `Effect`s then clear/overwrite storage. No separate "clear storage" code path needed. `color_pick` is left as-is (it is only meaningful when `color_level != "off"`), matching the existing per-control clear buttons.

### Decision: Restored active color level must trigger filter + delta sort on first render
The current code already keys the `filtered`/`sorted` closures off `color_level` / `color_pick` signals, so initialising those signals from storage is sufficient — no extra wiring.

## Risks / Trade-offs

- [`sessionStorage` unavailable (private mode, disabled)] → helpers already return `Option` / no-op on failure, same as the existing `localStorage` helpers; filters silently fall back to in-memory-only behaviour (today's behaviour).
- [Stored `location_id` points to a deleted location] → the location dropdown falls through to showing the raw id and the list shows nothing for that filter; acceptable and self-correcting once the user re-picks or clears. Same exposure as the existing sort-by-location.
- [Key schema churn] → keys are internal and per-session; a format change just means one session starts with defaults. No migration needed.
- [Effect write on every keystroke of the text filter] → `sessionStorage.setItem` is synchronous but cheap for short strings; the existing `localStorage` sort/page effects already do this.

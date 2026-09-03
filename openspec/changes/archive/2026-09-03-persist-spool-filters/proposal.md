## Why

Spool and filament list filters (text search, material, location, color level/pick, show-archived) live in per-component signals that are discarded whenever the user navigates to a detail page and back, or reloads the tab. Users lose their filter context constantly, and there is no single control to reset all filters at once.

## What Changes

- Persist all spool list filters for the lifetime of the browser tab session (survives in-app navigation and reload, cleared when the tab closes): text search, material, location, color level, color pick, and show-archived.
- Persist the filament list text search filter for the same session lifetime.
- Add a "Clear filters" button to the spool list and filament list headers. It is shown only when at least one filter is active and resets every filter to its default in one click.
- Filter state is stored in `sessionStorage` (not `localStorage`) so it does not leak across tab sessions.

## Capabilities

### New Capabilities
- `list-filter-persistence`: list filters persist in `sessionStorage` for the tab session and are cleared together by a "Clear filters" control that appears only when a filter is active.

### Modified Capabilities
- `spool-material-filter`: the selected material persists across navigation and reload within the session, and is cleared by the "Clear filters" control.
- `color-search-selector`: the color level and picked color persist across navigation and reload within the session, and are cleared by the "Clear filters" control.

## Impact

- `crates/spoolman-client/src/state.rs` — `TableState` gains session-backed filter storage; new `sessionStorage` get/set helpers.
- `crates/spoolman-client/src/pages/spool.rs` — `SpoolList` filter signals initialise from and persist to session storage; new Clear-filters button.
- `crates/spoolman-client/src/pages/filament.rs` — `FilamentList` text filter persistence; new Clear-filters button.
- `style/spoolman.css` — button styling reuse (`.btn`), no new dependency.
- No server, API, or data-model changes.

## 1. Session storage helpers

- [x] 1.1 In `crates/spoolman-client/src/state.rs` add `session_get(key: &str) -> Option<String>` and `session_set(key: &str, value: &str)` mirroring the existing `storage_get`/`storage_set` but using `w.session_storage()`.
- [x] 1.2 Convention chosen: `session_set` always writes; storing a filter's default value represents "cleared". No separate remove helper.

## 2. Persist the text filter in TableState

- [x] 2.1 In `use_table_state`, initialise `filter` from `session_get(&format!("filter.{namespace}.text"))` instead of always-empty; drop the "session-only" comment.
- [x] 2.2 Add an `Effect` that writes `filter` back to `filter.<ns>.text` on change (empty string clears it).
- [x] 2.3 Verified both `SpoolList` (namespace `spools`) and `FilamentList` (namespace `filaments`) pick this up with no page-level change.

## 3. Persist spool-specific filters in SpoolList

- [x] 3.1 Initialise `material_filter` from `filter.spools.material`, add a write-back `Effect`. Material dropdown options also get per-option `selected` so the restored value shows through Suspense.
- [x] 3.2 Initialise `location_filter` from `filter.spools.location` (parse `u32`), add a write-back `Effect`.
- [x] 3.3 Initialise `color_level` from `filter.spools.color_level` (default `"off"`), add a write-back `Effect`.
- [x] 3.4 Initialise `color_pick` from `filter.spools.color_pick` (default `"#000000"`), add a write-back `Effect`.
- [x] 3.5 Initialise `show_archived` from `filter.spools.show_archived` (`"true"`), add a write-back `Effect`, and bind the checkbox `prop:checked` to it.

## 4. Clear filters button

- [x] 4.1 Add a `filters_active` closure in `SpoolList` covering text, material, location, color level, show-archived.
- [x] 4.2 Render a `<button class="btn">"Clear filters"</button>` in the spool list `page-actions`, shown only when `filters_active()` is true; click resets every filter signal to its default (persist effects clear storage).
- [x] 4.3 Add the equivalent button to `FilamentList` for the text filter only.

## 5. Verification

- [x] 5.1 `cargo check -p spoolman-client --target wasm32-unknown-unknown` passes (clippy: no new warnings).
- [ ] 5.2 Manual/Playwright: set each spool filter, navigate to a detail page and back — filters retained; reload tab — filters retained; open a fresh tab — filters at defaults. (needs a running app)
- [ ] 5.3 Manual/Playwright: "Clear filters" appears only with an active filter and resets all of them; restored active color level filters + delta-sorts on first render. (needs a running app)
- [x] 5.4 Update `CHANGELOG.md` and mark the TODO.md item done.

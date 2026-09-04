## 1. Server: store reload

- [x] 1.1 Add `JsonStore::reload(&self) -> Result<()>` in `crates/spoolman-server/src/store.rs`: read `&self.path`, parse JSON, run the same migration step `load()` uses, then swap the parsed `DataStore` into `self.inner` via a write-lock assignment. On any read/parse error, return the error and leave `self.inner` untouched.
- [x] 1.2 Refactor `load()` if needed so the read+parse+migrate logic is shared with `reload()` rather than duplicated.

## 2. Server: route

- [x] 2.1 Add `POST /reload` to the router in `crates/spoolman-server/src/routes/other.rs`, calling `store.reload()` and returning `204 No Content` on success or the mapped `StoreError` on failure.

## 3. Client: API call

- [x] 3.1 Add `reload_database()` to `crates/spoolman-client/src/api/mod.rs` that POSTs to `/api/v1/reload` with no body and returns `Result<(), ...>` matching the existing API error type used by `put_setting`.

## 4. Client: Settings page button

- [x] 4.1 Add a "Reload database" button to `crates/spoolman-client/src/pages/settings.rs`, outside the existing settings `<form>`, with its own click handler calling `reload_database()`.
- [x] 4.2 Show a success message on success and an error message (reusing the existing `.success`/`.error` paragraph styles) on failure, using separate signals from the settings-save `saved`/`error` signals so the two actions don't clobber each other's status.

## 5. Verification

- [x] 5.1 `cargo check -p spoolman-server` and `cargo check -p spoolman-types` pass.
- [x] 5.2 Verified via the API directly (cargo-leptos is blocked on Windows and no Docker/WSL was available in this environment to build the full browser UI): started `spoolman-server`, edited the data file on disk, called `POST /api/v1/reload`, confirmed `GET /api/v1/location` reflects the new file's contents without restarting.
- [x] 5.3 Verified via the API directly (same constraint as 5.2): replaced the data file with invalid JSON, called `POST /api/v1/reload`, got a 500 error response, and confirmed `GET /api/v1/location` still served the last successfully-loaded data.

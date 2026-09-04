## Why

If the JSON data file (`spoolman.json`) is replaced on disk — restoring a backup, syncing from another machine, editing it by hand — the running server keeps serving the old in-memory copy until it's restarted. There's no way to pick up the new file short of a container/process restart, which isn't always convenient (e.g. no shell access, Docker restart policy delays). A button that re-reads the data file into memory closes that gap.

## What Changes

- Add a `POST /api/v1/reload` server endpoint that re-reads the configured data file from disk and replaces the in-memory store, running the same migration path used at startup.
- Add a "Reload database" button to the Settings page that calls this endpoint and reports success/failure to the user.
- Reject the reload (return an error, leave the in-memory store untouched) if the file on disk fails to parse, so a partial/corrupt file can't wipe good in-memory data.

## Capabilities

### New Capabilities
- `database-reload`: server support for re-reading the data file into memory on demand, and a Settings-page control to trigger it.

### Modified Capabilities
(none — no existing capability's requirements change)

## Impact

- `crates/spoolman-server/src/store.rs`: new `JsonStore::reload()` method reusing `load()`'s read+migrate logic against the already-resolved path, swapping the `RwLock<DataStore>` contents on success.
- `crates/spoolman-server/src/routes/other.rs`: new `POST /reload` route.
- `crates/spoolman-client/src/api/mod.rs`: new `reload_database()` API call.
- `crates/spoolman-client/src/pages/settings.rs`: new button + status message.

## Context

`JsonStore::load(path)` (`crates/spoolman-server/src/store.rs:56`) resolves the data file path once at startup, reads it, runs schema migration, and stores the result in `Arc<RwLock<DataStore>>`. All later reads/writes go through that lock; writes also call `flush()` to persist back to the same resolved `self.path`. There is currently no code path that re-reads `self.path` after startup — the only way to pick up an externally-changed file is a process restart.

## Goals / Non-Goals

**Goals:**
- Let an operator replace `spoolman.json` on disk and pick up the new contents without restarting the server.
- Reuse the existing load/migrate logic so reload behaves identically to a fresh startup read.
- Fail safe: if the on-disk file is missing, unreadable, or invalid JSON, leave the current in-memory data untouched and report the error.

**Non-Goals:**
- No file-watching/auto-reload — this is a manual, user-triggered action only.
- No merge of on-disk and in-memory state — reload is a full replace.
- No change to how data is normally persisted (`flush()`/atomic write path is untouched).

## Decisions

- **New `JsonStore::reload()` method** rather than calling `load()` again: `load()` constructs a brand new `JsonStore` (new `Arc`), which would orphan the one already held by Axum's router state. `reload()` instead reads `&self.path`, runs the same migration step used in `load()`, and on success replaces the contents of the existing `RwLock` via a write-lock swap — so the same `Arc<RwLock<DataStore>>` (and thus the same `Clone`d handles held by request tasks) sees the new data immediately.
- **Endpoint: `POST /api/v1/reload`**, added to `crates/spoolman-server/src/routes/other.rs` alongside the other store-level routes (`/info`, `/setting`). POST because it's a state-changing action with no request body: `204 No Content` on success, mapped `StoreError` on failure via the existing `routes::error::Result` conversion.
- **No new automatic backup step before reload.** Reload doesn't touch the file on disk, only replaces memory, so nothing needs backing up before it runs. (Existing automatic-backup logic guards writes/flushes, not reads.)
- **Settings-page placement**: a plain button (not inside the existing settings `<form>`, so it doesn't get swept up by the "Save" submit handler) with its own click handler, success/error message reusing the same `.success`/`.error` paragraph styling already on the page.

## Risks / Trade-offs

- [Concurrent request reads stale/mixed data mid-reload] → the write-lock swap is a single atomic assignment (`*inner.write().unwrap() = data`), so readers either see the fully-old or fully-new `DataStore`, never a partial one.
- [User reloads a file with IDs that collide with recently-created-but-not-yet-flushed spools] → not a new risk: all writes go through `flush()` synchronously, so on-disk and in-memory are already consistent before any reload; a reload restores exactly what's on disk.
- [Silent data loss if reload succeeds but the file was an old/wrong backup] → out of scope for this change; the button is an explicit, deliberate operator action, same trust level as manually replacing the file and restarting today.

## Migration Plan

No data migration. Purely additive: one server route, one store method, one client button. No rollback concerns beyond reverting the change.

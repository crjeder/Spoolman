## Why

The "remaining %" metric is redundant noise: it is always derivable from remaining weight and net weight, and only meaningful when net weight is known — making it an unreliable field that often displays "unknown". Removing it simplifies the UI and reduces cognitive load.

## What Changes

- Remove the `remaining_pct` computed field from `SpoolResponse`
- Remove `remaining_pct` from the spool list table column
- Remove `remaining_pct` from the spool detail view row
- Remove the `remaining_pct` calculation in `SpoolResponse::new`

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `spool-management`: remove `remaining_pct` field from spool response and all display surfaces

## Impact

- `crates/spoolman-types/src/responses.rs` — remove `remaining_pct` field and its calculation from `SpoolResponse`
- `crates/spoolman-server/` — any handler that builds or serialises `SpoolResponse`
- `crates/spoolman-client/src/pages/spool.rs` — remove column header, table cell, and detail row referencing `remaining_pct`
- No API consumers outside this repo; no breaking DB migration needed (computed field only)

## Context

`SpoolResponse` currently carries three derived weight fields: `used_weight`, `remaining_filament`, and `remaining_pct`. The last one (`remaining_pct`) is a percentage of filament remaining relative to net weight. It is only computable when `net_weight` is set on the filament; otherwise it serialises as `null`. In the UI it appears as a table column ("Remaining%") and a detail row — displaying either a percentage or "unknown".

## Goals / Non-Goals

**Goals:**
- Remove `remaining_pct` from `SpoolResponse` (type definition + calculation)
- Remove every UI surface that renders it (list column header, list table cell, detail row)
- Keep `remaining_filament` and `used_weight` — those remain useful

**Non-Goals:**
- Removing `remaining_filament` (still useful in grams)
- Changing the sort logic for `remaining_pct` (the sort key can be removed as dead code, but the sort itself goes away naturally)
- Changing the JSON storage format (no persisted data is affected — this is a computed field)

## Decisions

**Delete the field entirely rather than hide it behind a flag.**
`remaining_pct` is trivially computable client-side from `remaining_filament / net_weight * 100` if anyone ever needs it. Keeping a dead field in the API type pollutes the contract. Removal is clean and reversible if needed later.

**Update the sort comparator in `SpoolList`.**
The `"remaining_pct"` arm in the sort `match` becomes dead code once the column is removed. It should be deleted to keep the sort logic clean.

## Risks / Trade-offs

- **E2E tests that assert on the column** — any Playwright test asserting `remaining_pct` column presence or value will fail. Check `tests/e2e/` for references. → Mitigation: update or drop those assertions as part of this change.
- **Rust compile errors** — removing the field will cause compiler errors at every use site; the compiler guides the remaining deletions. → Mitigation: follow compiler output to find all use sites.

## Migration Plan

1. Remove `remaining_pct` field and calculation from `SpoolResponse` in `spoolman-types`.
2. Fix compile errors in `spoolman-server` (any code referencing the field).
3. Remove UI references in `spoolman-client` (column header, table cell, detail row, sort arm).
4. Update E2E tests if any assert on the column.
5. `cargo check` clean across all crates.

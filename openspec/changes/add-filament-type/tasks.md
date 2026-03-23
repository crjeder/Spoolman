## 1. API client

- [ ] 1.1 Add `list_materials() -> Result<Vec<String>, ApiError>` to `crates/spoolman-client/src/api/filament.rs` calling `GET /api/v1/material`

## 2. Material autocomplete on filament forms

- [ ] 2.1 In `FilamentCreate` (`pages/filament.rs`): fetch materials with `create_resource(|| (), |_| async { api::list_materials().await })`
- [ ] 2.2 Add `<datalist id="filament-materials">` populated from the resource, and add `list="filament-materials"` attribute to the Material `<input>`
- [ ] 2.3 Apply the same datalist wiring to `FilamentEdit` (same file, reuse the same datalist id — both components are never on-screen simultaneously)

## 3. Material filter on filament list

- [ ] 3.1 In `FilamentList` (`pages/filament.rs`): fetch materials with a resource (`list_materials`)
- [ ] 3.2 Add a `material_filter: RwSignal<String>` (default: empty = "All")
- [ ] 3.3 Render a `<select>` dropdown in the page-actions bar: first option `<option value="">"All materials"</option>`, then one `<option>` per material from the resource
- [ ] 3.4 When `material_filter` changes, reset `ts.page` to 0 to avoid empty pages
- [ ] 3.5 Change `list_filaments()` to accept an optional `material: Option<&str>` param and append `?material=<value>` when set; update `api/filament.rs` accordingly
- [ ] 3.6 Wire the resource to re-fetch when `material_filter` changes: `create_resource(move || material_filter.get(), |mat| async move { api::list_filaments(if mat.is_empty() { None } else { Some(&mat) }).await })`

## 4. Material in spool list text filter

- [ ] 4.1 In the `filtered` closure in `SpoolList` (`pages/spool.rs`): add `|| s.filament.material.as_deref().unwrap_or("").to_lowercase().contains(&f)` to the filter condition so typing "PLA" narrows the spool list

## 5. Update CHANGELOG and TODO

- [ ] 5.1 Add entry to `CHANGELOG.md` under `[Unreleased] → Added`: "Material autocomplete on filament create/edit forms, using `GET /api/v1/material` for suggestions."
- [ ] 5.2 Add entry under `[Unreleased] → Added`: "Material filter dropdown on the Filament list; spool list text filter now matches on `filament.material`."
- [ ] 5.3 Remove "Add `filament_type` field to Filament" from `TODO.md` (covered by this change)

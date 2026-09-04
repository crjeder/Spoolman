## Why

The data model already stores 1–4 colors per spool (`Spool.colors: Vec<Rgba>`), and the spool list and detail views already render every color as a swatch. But the create and edit forms only expose a single color picker, so multi-color spools (co-extrusion, silk dual-tone, gradient) can only be entered via the API. Users need a way to add extra color rows in the UI.

## What Changes

- The spool edit form gains a "+" button at the end of the color row that appends another color picker row (hex input + opacity slider), up to a maximum of 4.
- Each added color row gets a "−" button to remove it; the first row cannot be removed.
- On submit, all populated color rows are sent as the `colors` array (order preserved) instead of just the first.
- The spool create form gets the same multi-row color editor for consistency.
- `color_name` remains a single field describing the spool's overall color.

## Capabilities

### New Capabilities

_None._

### Modified Capabilities

- `spool-management`: the "Edit spool" and "Create spool" requirements gain scenarios for editing multiple colors through the form (add row, remove row, cap at 4, order preserved on save).

## Impact

- `crates/spoolman-client/src/pages/spool.rs`: `SpoolEdit` and `SpoolNew` color editing — replace the single `color_hex`/`color_alpha` signals with a list of rows; add/remove handlers; build the `colors` vec from all rows on submit.
- CSS: minor additions for the add/remove buttons and stacked color rows (reuse existing `.color-alpha-row`).
- No API, data model, or server changes. Server already accepts a `colors` array; a length cap of 4 may be added as a defensive validation (optional).
- E2E: add a Playwright case covering adding a second color in the edit form.

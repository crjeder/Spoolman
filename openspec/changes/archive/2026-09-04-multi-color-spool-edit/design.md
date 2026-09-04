## Context

`Spool.colors` is already `Vec<Rgba>` (documented 1–4). `SpoolCreate.colors: Vec<Rgba>` and `UpdateSpool.colors: Option<Vec<Rgba>>` already carry the full array. The list and detail views already map every entry to a `.color-swatch`. Only the forms in `crates/spoolman-client/src/pages/spool.rs` are single-color: `SpoolEdit` and `SpoolNew` each hold `color_hex: RwSignal<String>` + `color_alpha: RwSignal<u8>` and build `vec![c]` on submit.

## Goals / Non-Goals

**Goals:**
- Add/remove color rows in both forms, 1–4, order preserved.
- Reuse existing markup/CSS (`.color-alpha-row`) and the existing `hex_to_rgba` helper.
- Prefill all rows when editing an existing multi-color spool.

**Non-Goals:**
- No changes to display (list/detail already handle multiple colors).
- No gradient rendering or per-color naming.
- No new API, data-model, or store changes.

## Decisions

### Represent rows as `RwSignal<Vec<(hex, alpha)>>`
Replace the two scalar signals with one `RwSignal<Vec<(RwSignal<String>, RwSignal<u8>)>>` (or a `Vec` of a small struct). Each row renders from its own nested signals so per-row inputs stay independent without re-rendering siblings. Add = push a default row; remove = retain by index. Submit maps the vec through `hex_to_rgba`, applying alpha, filtering `None`.

Alternative considered: a flat `Vec<Rgba>` signal with inputs writing back by index. Rejected — every keystroke would clone and replace the whole vec and churn all rows.

### Cap at 4 in the client; optional server guard
The "+" control is hidden when `rows.len() == 4`. A defensive `colors.len() <= 4` check in `store.rs` on create/update is listed as an optional task, not required — the UI is the only writer today and the model comment already states the limit.

### Keep `color_name` as-is
One name field below the color rows, unchanged.

## Risks / Trade-offs

- [Empty/invalid hex row on submit] → rows failing `hex_to_rgba` are dropped; if all rows fail, `colors` is empty, matching current behavior.
- [Edit form currently only reads `colors.first()`] → change to iterate; a spool with 0 colors still yields 1 default row so the form always has one.
- [E2E selectors] → the existing single color input keeps its structure for row 1, so only new tests for the "+" path are needed.

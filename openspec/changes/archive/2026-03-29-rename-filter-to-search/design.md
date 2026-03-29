## Context

The filament and spool list pages each have a text input with `placeholder="Filter…"` that filters the displayed rows client-side. The label "Filter" is non-standard for this kind of control; "Search" better matches user expectations. There is currently no way to clear the input without manually selecting and deleting the text.

## Goals / Non-Goals

**Goals:**
- Change the placeholder text from "Filter…" to "Search…" on both pages
- Add an inline "×" clear button that appears when the input has a value
- Clicking the clear button empties the input and resets the list

**Non-Goals:**
- Renaming internal Rust signal/variable names (e.g. `ts.filter`) — not a user-facing change
- Adding debounce or async search
- Changing filtering logic

## Decisions

**Inline clear button vs. separate button outside the input**
Inline "×" button positioned inside the input (absolute/flex layout) is the conventional pattern — it keeps the control compact and visually coupled to the input. Chosen over a separate button.

**Conditional visibility**
The "×" button is hidden when the input is empty (no value) and shown when it has content. This avoids visual clutter on page load.

**Implementation**
- Wrap the `<input>` in a `<div class="search-input-wrapper">` (or equivalent flex container)
- Render a `<button>` sibling with a "×" character (or `&times;`)
- Show/hide via Leptos reactive signal: `{move || (!ts.filter.get().is_empty()).then(|| view! { <button …> })}`
- On click: `ts.filter.set(String::new())`

## Risks / Trade-offs

- **CSS coordination** — the clear button needs to overlap or sit flush with the input; minor styling effort. No external dependency needed.
- Minimal risk overall — purely additive frontend change with no backend or data impact.

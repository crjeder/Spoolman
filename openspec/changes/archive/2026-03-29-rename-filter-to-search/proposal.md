## Why

The "Filter" label on the filament and spool list pages is ambiguous — users expect a text input that searches by name or keyword to be called "Search". Adding a clear button reduces friction when users want to reset the search without manually selecting and deleting the text.

## What Changes

- Rename the "Filter" input label/placeholder to "Search" on the Filaments page
- Rename the "Filter" input label/placeholder to "Search" on the Spools page
- Add an "X" (clear) button inside the search input that appears when the field has a value
- Clicking the "X" button clears the input and resets the list

## Capabilities

### New Capabilities
<!-- None — this change modifies existing UI behavior only -->

### Modified Capabilities
- `filament-management`: Search input renamed from "Filter" to "Search"; clear button added
- `spool-management`: Search input renamed from "Filter" to "Search"; clear button added

## Impact

- Frontend only — Leptos components for the filament and spool list pages
- No API changes
- No data model changes

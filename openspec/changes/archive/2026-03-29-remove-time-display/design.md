## Context

Spool timestamps (`registered`, `first_used`, `last_used`) are stored as `DateTime<Utc>` in the data model. The UI currently shows them with full time-of-day (`%Y-%m-%d %H:%M UTC`) and uses `<input type="datetime-local">` for editable fields. All changes are confined to the Leptos frontend — no server or data-model changes are needed.

## Goals / Non-Goals

**Goals:**
- Display all spool timestamps as date-only (`YYYY-MM-DD`) everywhere in the UI
- Replace `datetime-local` inputs with `date` inputs for `first_used` / `last_used`
- When constructing a `DateTime<Utc>` from a date-only input, use `00:05:00 UTC` as the fixed time

**Non-Goals:**
- Changing the stored data format (still `DateTime<Utc>` in JSON)
- Modifying server routes or API contracts
- Altering `registered` (it is set server-side to `Utc::now()` on creation, never edited)

## Decisions

### Date-only display
Format all displayed timestamps with `%Y-%m-%d`. Remove `%H:%M UTC` from the detail panel and any other display sites. Simple format string change — no logic change.

### Date input instead of datetime-local
Switch `<input type="datetime-local">` → `<input type="date">` for `first_used` and `last_used` in the edit form. The `date` input yields strings in `YYYY-MM-DD` format.

Alternatives considered:
- Keep `datetime-local` but hide the time — messy, browser-dependent UI.
- Strip time on display only — misleads the user if the stored time differs from midnight.

### Fixed time component: 00:05:00
When parsing a date string for the API call, construct the `NaiveDateTime` as `date + NaiveTime::from_hms(0, 5, 0)` then `.and_utc()`.

Why 00:05 instead of 00:00: avoids ambiguity with zero-value sentinels and is the user's explicit preference.

### Form init format
When populating the edit form from existing spool data, format `first_used`/`last_used` as `%Y-%m-%d` (date only) to match the new input type.

## Risks / Trade-offs

- **Existing data with non-midnight times** — timestamps already in the JSON with meaningful times will display as date-only; on next save they will be rounded to 00:05 UTC. Acceptable: the user has stated they don't care about time.
- **`registered` in detail view** — currently shows `%Y-%m-%d %H:%M UTC`; trimming to date-only is purely cosmetic and safe.

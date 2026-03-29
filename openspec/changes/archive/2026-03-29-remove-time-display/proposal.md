## Why

Time-of-day is meaningless for spool tracking — users care about dates, not hours and minutes. The current UI displays `%Y-%m-%d %H:%M UTC` timestamps in both the detail panel and uses `datetime-local` inputs for `first_used`/`last_used`, adding noise without value.

## What Changes

- Remove time portion from all displayed timestamps (registered, first_used, last_used) — show only `YYYY-MM-DD`
- Replace `datetime-local` form inputs with `date` inputs for `first_used` and `last_used`
- When a date-only value must be converted to a `DateTime<Utc>` for the API, fix the time component to `00:05:00 UTC` (5 minutes past midnight)

## Capabilities

### New Capabilities
<!-- none -->

### Modified Capabilities
- `spool-management`: Display and edit of spool timestamps changes from datetime to date-only

## Impact

- `crates/spoolman-client/src/pages/spool.rs` — display formats, form input types, parse logic
- No API, data model, or server changes required

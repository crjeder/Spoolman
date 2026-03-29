## 1. Display — remove time from spool detail panel

- [x] 1.1 In `spool.rs` detail panel, change `registered` format from `%Y-%m-%d %H:%M UTC` to `%Y-%m-%d`
- [x] 1.2 In `spool.rs` detail panel, change `first_used` format from `%Y-%m-%d %H:%M UTC` to `%Y-%m-%d`
- [x] 1.3 In `spool.rs` detail panel, change `last_used` format from `%Y-%m-%d %H:%M UTC` to `%Y-%m-%d`

## 2. Edit form — switch to date-only inputs

- [x] 2.1 Change `first_used` input from `type="datetime-local"` to `type="date"` in the spool edit form
- [x] 2.2 Change `last_used` input from `type="datetime-local"` to `type="date"` in the spool edit form
- [x] 2.3 Change form init format for `first_used` from `%Y-%m-%dT%H:%M` to `%Y-%m-%d`
- [x] 2.4 Change form init format for `last_used` from `%Y-%m-%dT%H:%M` to `%Y-%m-%d`

## 3. Parse — inject fixed time on submit

- [x] 3.1 Replace `NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M")` with date-only parse: `NaiveDate::parse_from_str(&s, "%Y-%m-%d")` then `.and_hms_opt(0, 5, 0).unwrap().and_utc()`
- [x] 3.2 Add `chrono::NaiveDate` to the `use chrono::` import in `spool.rs` (remove unused `NaiveDateTime` if no longer needed)

## 4. Verify

- [x] 4.1 Run `cargo check -p spoolman-client` (requires wasm32 target) or confirm no compile errors in the client crate
- [ ] 4.2 Manually verify in the browser: detail panel shows dates without time, edit form accepts date-only, saved values come back as `YYYY-MM-DD`

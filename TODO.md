# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

### Bugs (found via Playwright Docker test, 2026-03-27)
1. Settings page: currency symbol renders as `â,¬` instead of `€` — UTF-8 double-encoding in settings page
2. Spool detail (`/spools/<id>`): assigned Location not displayed — Location field missing from detail view
3. /api/v1/info is empty
4. delete button in locations is broken
5. "sure?" button (after delete) has no effect
7. HTTP 404: Not Found on save after edit
8. save in edit location broken

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- [ ] "clear search" button
- [ ] rename "filter" to "search"
- [ ] table headers contain filter button
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- [ ] remove "remaining %"
- [ ] location must not be empty or "none"
- [ ] don't care about time. remove from display. if it must be set then set it to 5 min past midnight

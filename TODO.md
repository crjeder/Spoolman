# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### Bugs (found via Playwright Docker test, 2026-03-24)

### B7 Color's alpha value is not used anywhere
- the color picker should allow to select an alpha value. if that's not possible add an extra selector elsewhere

### B14 no date in Spool view
- does not show date information (creation, last use)
- edit of date is not possible

### B15 delete buttons broken
- when the delete button of a location is pressed, it does not disappear until reload. before removal add a "Sure?" dialog. same for the other entities
- when deleting a spool / filament in detail view "HTTP 404: Not Found" is shown. (because the element was just deleted) jump to list view instead.

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`

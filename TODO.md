# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending
- [x] B20: color square (U+25A0) does not show the currently selected color. 
- [ ] B21 /api/v1/info is empty

### OpenSpec Implementation Discrepancies

Items where the actual code does NOT match the OpenSpec specification:

**Summary: 4 active OpenSpec changes checked**
- ✓ 2 fully implemented (`color-hex-display`, `sort-spools-by-color-delta`)
- ✗ 1 incompletely implemented (`alternative-color-distance` — spec requires enum/dispatcher, code lacks it)
- ⚠️ 1 design mismatch (`add-filament-type` — spec says datalist/free-text, code uses enum/select)
- ✓ 1 bonus implemented (`validate-location-required-spool` — not yet in spec, already in code)

**Details:**

1. **[color-hex-display]** ✓ IMPLEMENTED — hex formatted as `#{:02x}{:02x}{:02x}` in `spool.rs` line 426, CSS styling in `spoolman.css` line 484. **Tracking issue:** TODO.md line 23 "add the hex value..." not marked complete. **ACTION: Mark as [x].**

2. **[alternative-color-distance]** ✗ INCOMPLETE — Spec requires `ColorAlgorithm` enum, `color_distance(a, b, algo)` dispatcher, per-algorithm thresholds, and Settings page UI. Tasks in `tasks.md` marked complete, but actual code shows old signature `color_distance(a: &Rgba, b: &Rgba) -> f32` (line 68 in `utils/color.rs`). No enum exists. This blocks full algorithm selection in Settings and forces `sort-spools-by-color-delta` to use CIEDE2000-only. **ACTION: Either revert tasks.md to mark incomplete, or implement per spec.**

3. **[add-filament-type]** ⚠️ DESIGN MISMATCH — Spec requires free-text Material input with `<datalist>` autocomplete fed by `GET /api/v1/material`. Actual code uses `MaterialType` enum (closed set) with `<select>` dropdown and `MaterialType::from_abbreviation()` in `filament.rs` lines 316–395. Backend supports `list_materials()` but frontend never uses it. This is a valid alternative design (cleaner enum validation) but contradicts the spec. **ACTION: Clarify if enum-based approach is intentional (in design doc) or if datalist implementation is still pending.**

4. **[sort-spools-by-color-delta]** ✓ IMPLEMENTED — `sorted()` closure in `spool.rs` lines 105–130 correctly computes `min_delta()` for each spool and sorts ascending by delta when `color_level != "off"`. Includes `hex_to_rgba()` and `color_distance()` calls. **NOTE: Works with CIEDE2000-only until alternative-color-distance is completed.**

5. **[validate-location-required-spool]** ✓ BONUS IMPLEMENTATION — Not yet a formal spec, but frontend validation already exists in `SpoolCreate` (line 489) and `SpoolEdit` (line 646): checks `if location_id.get().is_none()` and sets error "Location is required." Forms prevent submission without location. **ACTION: Consider archiving or formalizing as spec if feature is stable.**

### Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] use locale to format date and time. fall back to what is configured in settings. add a setting for date / time format
- [x] rename "filter" to "search"
- [ ] table headers contain filter button (partial: Color column header activates color picker filter)
- [ ] Manual verify `remove-time-display`: detail panel shows date-only and form retains `YYYY-MM-DD` semantics
- [ ] Manual verify `format-currency-date-numbers-intl`: locale formatting is active for dates/weights/density in browser rendering
- [ ] move Filament.net_weight to spool.net_weight
- [ ] add delete buttons wherever edit buttons are
- ~~remove "remaining %"~~
- [x] location must not be empty or "none"
- [x] don't care about time. remove from display. if it must be set then set it to 5 min past midnight
- [x] the sensitivity slider in color search is not intuitive. replace it by a selector for distinct values e. g. off = no color match (default), fine, medium and coarse
- [x] remove the color button from above the table
- [x] place a little square unicode U+25A0 in the current color if color search is not "off"
- [ ] extend search to location
- [ ] place a filter icon in location table head. user can select a location from drop down. filter table to show only entries which match the selected location
- [ ] color search for multi color is not intuitive
- [x] pop-up color selector would be better than the selector on top of the page. color should not change when changing the threshold
- [x] add the hex value to the color display in spool details
- [x] sort spools according to delta when a color is selected
- [ ] implement alternative color distance calculation oklab (or din99d). make them configurable in settings (oklab crate)#
- [ ] handling of alpha value in color search needs to be better
- [ ] take surface finish into account for color search
- [ ] add material column in spools. table head links to a filter (drop down) to select materials to display
- [ ] make the threshold values configurable per calculation algorithm (in settings)

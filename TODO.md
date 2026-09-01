# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Enhancements
- [ ] NFC / QR sticker integration — [OpenSpoolMan](https://github.com/drndos/openspoolman) or [OpenTag3D](https://opentag3d.com/) compatible; spool NFC URL already maps to `/api/v1/spool/<id>`
- [ ] filament create/edit + spool create: SpoolmanDB lookup — fetch https://donkie.github.io/SpoolmanDB/filaments.json, cache in localStorage (24h TTL + ETag), client-side search, auto-fill filament fields; in spool create auto-create missing filament and notify user
- [ ] filament/spool: filamentcolors.xyz color lookup — deferred: API CORS headers absent from their Django app, direct WASM fetch will be blocked; needs a server-side proxy endpoint (/api/v1/proxy/filamentcolors) before this is viable
- [ ] test on mobile

## Deficits
- [~] "HTTP 500: Internal Server Error" error on edit — root cause found: `/data` volume created root-owned, non-root container user can't write; fixed in Dockerfile, not yet verified against a real Docker build
- [ ] date selector not always defaults to today
- [ ] detail view does not show if it's spool or filament
- [x] click on column 1 in spool view should lead to spool details (not filament)
- [ ] color name on spool?
- [x] drop down fields in edit spool / filament do not show the current value
- [x] edit filament dialog: should be Edit <current filament> not "Filament"
- [ ] "search filament" only in new filament dialog not in edit
- [ ] new spool should lookup local filaments, too
- [ ] date format setting not respected
- [ ] enforce location
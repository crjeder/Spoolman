# TODO

Items to address. Move completed items to [CHANGELOG.md](CHANGELOG.md) under the appropriate release.

## Pending

### In Progress
- [ ] Complete Rust rewrite (`openspec/changes/migrate-to-rust/`) — code written, pending build verification
  - [ ] Install `cargo-leptos` on Windows (blocked: OpenSSL dev headers missing — build in WSL/Linux/Docker instead)
  - [ ] `cargo leptos build --release` first successful build
  - [ ] Verify single binary serves API + WASM frontend (task 12.2)
  - [ ] Update `docker-compose.yml` for new binary entrypoint (task 12.3)
  - [ ] Verify `SPOOLMAN_DATA_FILE` env var mounts correctly in container (task 12.4)
- [x] Legacy export converter — `scripts/convert_export.py` complete with smoke tests

### Enhancements
- [ ] NFC tag write support — use spool URL `/api/v1/spool/<id>` as OpenTag3D Online Data URL
- [ ] Color picker UI for `Vec<Rgba>` on spool create/edit (currently accepts color name only)
- [ ] SpoolmanDB search modal in filament create form (infrastructure in place at `GET /api/v1/filament/search`)
- [ ] Make spool list the default landing page (currently home/dashboard)
- [ ] Color search / filter by swatch
- [ ] Light theme matching the logo
- [ ] `.env` file support at startup

### Cleanup (after Rust rewrite merges)
- [ ] Remove `spoolman/` (Python backend)
- [ ] Remove `client/` (React/TypeScript frontend)
- [ ] Remove `pyproject.toml`, `pdm.lock`, `uv.lock`
- [ ] Remove `entrypoint.sh`
- [ ] Archive `openspec/changes/migrate-to-rust/` via `/opsx:archive`

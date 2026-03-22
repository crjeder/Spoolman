## Why

This is a home-use fork stripped of multi-user and enterprise features; WebSocket real-time push notifications add complexity with no practical benefit when a single user is querying the API directly. Removing WebSocket support simplifies the backend, eliminates dead code paths, and keeps the codebase aligned with the fork's stated scope.

## What Changes

- Delete `spoolman/ws.py` (the `WebsocketManager` and `SubscriptionTree` classes)
- Remove all WebSocket route handlers from `spoolman/api/v1/router.py`, `spool.py`, `filament.py`, and `setting.py`
- Remove all `websocket_manager.send(...)` calls from CRUD mutation handlers in `spool.py`, `filament.py`, and `setting.py`
- Remove unused `WebSocket`, `WebSocketDisconnect` imports from affected modules
- Update `README.md` to remove any mention of WebSocket support
- Update `CLAUDE.md` to remove the `ws.py` architecture entry

## Capabilities

### New Capabilities

_(none — this is a removal-only change)_

### Modified Capabilities

_(no spec-level behavior changes to track — WebSocket endpoints are being removed entirely, not altered)_

## Impact

- **Backend files changed:** `spoolman/ws.py` (deleted), `spoolman/api/v1/router.py`, `spoolman/api/v1/spool.py`, `spoolman/api/v1/filament.py`, `spoolman/api/v1/setting.py`
- **Docs changed:** `README.md`, `CLAUDE.md`
- **API surface:** WebSocket endpoints at `/api/v1/`, `/api/v1/spool`, `/api/v1/spool/{id}`, `/api/v1/filament`, `/api/v1/filament/{id}`, `/api/v1/setting`, `/api/v1/setting/{key}` are removed — **BREAKING** for any client relying on WS push
- **Dependencies:** No new dependencies; `websockets` transitive dep from `starlette` remains (not removed, as starlette needs it for its own internals)
- **Tests:** Integration tests should be checked for any WS-specific test cases

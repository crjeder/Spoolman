## 1. Delete WebSocket core module

- [x] 1.1 Delete `spoolman/ws.py`

## 2. Clean up `spoolman/api/v1/router.py`

- [x] 2.1 Remove `WebSocket`, `WebSocketDisconnect` imports from `fastapi`
- [x] 2.2 Remove `from spoolman.ws import websocket_manager` import
- [x] 2.3 Remove the root-level `@app.websocket(...)` handler function and its docstring

## 3. Clean up `spoolman/api/v1/spool.py`

- [x] 3.1 Remove `WebSocket`, `WebSocketDisconnect` imports from `fastapi`
- [x] 3.2 Remove `from spoolman.ws import websocket_manager` import
- [x] 3.3 Remove `websocket_manager.send(...)` call (and its surrounding try/except) from the create-spool handler
- [x] 3.4 Remove `websocket_manager.send(...)` call (and its surrounding try/except) from the update-spool handler
- [x] 3.5 Remove `websocket_manager.send(...)` call (and its surrounding try/except) from the use-filament/spool handler
- [x] 3.6 Remove the `notify_any` WebSocket route handler (`@router.websocket("")`)
- [x] 3.7 Remove the `notify` WebSocket route handler (`@router.websocket("/{spool_id}")`)

## 4. Clean up `spoolman/api/v1/filament.py`

- [x] 4.1 Remove `WebSocket`, `WebSocketDisconnect` imports from `fastapi`
- [x] 4.2 Remove `from spoolman.ws import websocket_manager` import
- [x] 4.3 Remove `websocket_manager.send(...)` call (and its surrounding try/except) from the create-filament handler
- [x] 4.4 Remove `websocket_manager.send(...)` call (and its surrounding try/except) from the update-filament handler
- [x] 4.5 Remove the `notify_any` WebSocket route handler (`@router.websocket("")`)
- [x] 4.6 Remove the `notify` WebSocket route handler (`@router.websocket("/{filament_id}")`)

## 5. Clean up `spoolman/api/v1/setting.py`

- [x] 5.1 Remove `WebSocket`, `WebSocketDisconnect` imports from `fastapi`
- [x] 5.2 Remove `from spoolman.ws import websocket_manager` import
- [x] 5.3 Remove `websocket_manager.send(...)` call (and its surrounding try/except) from the set-setting handler
- [x] 5.4 Remove the `notify_any` WebSocket route handler (`@router.websocket("")`)
- [x] 5.5 Remove the `notify` WebSocket route handler (`@router.websocket("/{key}")`)

## 6. Verify and lint

- [x] 6.1 Run `ruff check .` and fix any remaining import or unused-variable warnings
- [x] 6.2 Start the dev server (`uvicorn spoolman.main:app --reload`) and confirm it starts without errors
- [x] 6.3 Smoke-test a REST endpoint (e.g., `GET /api/v1/spool`) to confirm normal operation

## 7. Update documentation

- [x] 7.1 Remove `ws.py` entry from `CLAUDE.md` architecture table
- [x] 7.2 Remove any WebSocket references from `README.md`
- [x] 7.3 Add a BREAKING entry to `CHANGELOG.md` under `## [Unreleased] > Removed`

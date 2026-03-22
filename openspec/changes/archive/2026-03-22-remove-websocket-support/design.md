## Context

WebSocket support spans four backend files. The `WebsocketManager` in `spoolman/ws.py` is a module-level singleton imported by `router.py`, `spool.py`, `filament.py`, and `setting.py`. Each of those files has two concerns to untangle: WebSocket route handlers (separate `@router.websocket(...)` endpoints) and `websocket_manager.send(...)` calls inside ordinary CRUD mutation handlers.

The frontend (Refine/React) does not appear to consume these WebSocket endpoints; the UI refreshes via TanStack Query polling. No integration tests exercise WebSocket paths.

## Goals / Non-Goals

**Goals:**
- Delete `spoolman/ws.py` entirely
- Remove all WebSocket route handlers and `websocket_manager` imports/calls from the four affected modules
- Leave all REST CRUD logic untouched
- Update `README.md` and `CLAUDE.md` to reflect the removal

**Non-Goals:**
- Replacing WebSocket with SSE or polling — no real-time push mechanism is needed
- Touching the frontend
- Removing `starlette`'s transitive WebSocket dependency (it's part of the framework)

## Decisions

**Delete rather than stub out `ws.py`**
Keeping a no-op module would leave dead import paths and invite future confusion. Full deletion makes the intent clear.

**Remove `websocket_manager.send()` from CRUD handlers rather than guarding with `if False`**
Silent dead code is worse than no code. The try/except blocks around each `send()` call should be removed entirely along with the send call — they exist only to catch WebSocket send failures.

**No deprecation period**
This is a single-user home-use fork with no downstream consumers to notify. A deprecation window adds no value.

## Risks / Trade-offs

[Risk] A future integrator may have depended on WebSocket endpoints → Mitigation: document the removal clearly in CHANGELOG.md as a BREAKING change; the README already scopes this as a personal-use fork.

[Risk] Missed import cleanup causes an `ImportError` at startup → Mitigation: tasks include a `ruff check .` run to catch stale imports before the change is considered done.

## Migration Plan

1. Delete `spoolman/ws.py`
2. Strip WebSocket route handlers and `websocket_manager` usage from `router.py`, `spool.py`, `filament.py`, `setting.py`
3. Run `ruff check .` and fix any remaining import warnings
4. Update `README.md` and `CLAUDE.md`
5. Update `CHANGELOG.md` with a BREAKING removal entry
6. Smoke-test: start the server and confirm REST endpoints still respond

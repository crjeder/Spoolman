## REMOVED Requirements

### Requirement: Real-time spool change notifications via WebSocket
The system SHALL no longer expose WebSocket endpoints for spool change events.

**Reason**: This is a single-user home-use fork; real-time push is unnecessary complexity with no active consumers.
**Migration**: Clients that relied on WS push for spool updates should switch to polling the REST `GET /api/v1/spool` endpoint.

#### Scenario: WebSocket endpoint for spools removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/spool`
- **THEN** the server SHALL NOT accept the connection (endpoint no longer exists)

#### Scenario: WebSocket endpoint for specific spool removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/spool/{id}`
- **THEN** the server SHALL NOT accept the connection (endpoint no longer exists)

### Requirement: Real-time filament change notifications via WebSocket
The system SHALL no longer expose WebSocket endpoints for filament change events.

**Reason**: Same as above — single-user scope, no consumers.
**Migration**: Poll `GET /api/v1/filament` instead.

#### Scenario: Filament WebSocket endpoint removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/filament`
- **THEN** the server SHALL NOT accept the connection

#### Scenario: Filament-specific WebSocket endpoint removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/filament/{id}`
- **THEN** the server SHALL NOT accept the connection

### Requirement: Real-time setting change notifications via WebSocket
The system SHALL no longer expose WebSocket endpoints for setting change events.

**Reason**: Same as above.
**Migration**: Poll `GET /api/v1/setting` instead.

#### Scenario: Setting WebSocket endpoint removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/setting`
- **THEN** the server SHALL NOT accept the connection

#### Scenario: Setting-key WebSocket endpoint removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/setting/{key}`
- **THEN** the server SHALL NOT accept the connection

### Requirement: Root-level catch-all WebSocket health endpoint
The system SHALL no longer expose a root-level WebSocket health/heartbeat endpoint.

**Reason**: Removed with all other WebSocket infrastructure.
**Migration**: Use `GET /api/v1/health` (REST) for health checks.

#### Scenario: Root WebSocket endpoint removed
- **WHEN** a client attempts to open a WebSocket connection to `/api/v1/`
- **THEN** the server SHALL NOT accept the connection

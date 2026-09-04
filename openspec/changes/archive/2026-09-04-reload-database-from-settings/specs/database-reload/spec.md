## ADDED Requirements

### Requirement: Server can reload the data file on demand
The system SHALL provide a `POST /api/v1/reload` endpoint that re-reads the configured data file from disk, applies any pending schema migration, and replaces the in-memory data store with the result.

#### Scenario: Reload picks up externally-changed data
- **WHEN** the data file on disk has been replaced (e.g. a backup restored) since the server started, and a client sends `POST /api/v1/reload`
- **THEN** the server re-reads that file, runs migration if needed, and subsequent API responses reflect the new file's contents

#### Scenario: Reload rejects an unreadable or invalid file
- **WHEN** the data file on disk is missing, unreadable, or not valid JSON, and a client sends `POST /api/v1/reload`
- **THEN** the server returns an error response and the in-memory data store is left unchanged

### Requirement: Settings page exposes a reload control
The Settings page SHALL include a "Reload database" button that triggers the reload endpoint and reports the outcome to the user.

#### Scenario: User triggers a successful reload
- **WHEN** the user clicks "Reload database" and the server reload succeeds
- **THEN** the page shows a success message and reflects the reloaded data on next navigation/query

#### Scenario: User triggers a failed reload
- **WHEN** the user clicks "Reload database" and the server reload fails
- **THEN** the page shows an error message describing the failure, and no data is lost

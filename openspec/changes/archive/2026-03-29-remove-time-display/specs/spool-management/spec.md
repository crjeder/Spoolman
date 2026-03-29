## ADDED Requirements

### Requirement: Date-only timestamp display
The frontend SHALL display all spool timestamps (`registered`, `first_used`, `last_used`) as date-only values in `YYYY-MM-DD` format. Time-of-day SHALL NOT be shown anywhere in the spool UI.

#### Scenario: Registered date shown without time
- **WHEN** a spool's detail panel is open
- **THEN** `registered` is displayed as `YYYY-MM-DD` with no time component

#### Scenario: First used shown without time
- **WHEN** a spool has a `first_used` value and the detail panel is open
- **THEN** `first_used` is displayed as `YYYY-MM-DD` with no time component

#### Scenario: Last used shown without time
- **WHEN** a spool has a `last_used` value and the detail panel is open
- **THEN** `last_used` is displayed as `YYYY-MM-DD` with no time component

### Requirement: Date-only edit inputs for spool timestamps
The spool edit form SHALL use date-only inputs (not datetime-local) for `first_used` and `last_used`. When those values are submitted to the API, the time component SHALL be fixed to `00:05:00 UTC`.

#### Scenario: Edit form shows date picker for first_used
- **WHEN** the spool edit form is open
- **THEN** the `first_used` field is a date input (not a datetime-local input)

#### Scenario: Edit form shows date picker for last_used
- **WHEN** the spool edit form is open
- **THEN** the `last_used` field is a date input (not a datetime-local input)

#### Scenario: Submitted date gets fixed time component
- **WHEN** the user sets `first_used` to `2024-03-15` and saves
- **THEN** the API receives `2024-03-15T00:05:00Z` for `first_used`

#### Scenario: Existing datetime populates date field
- **WHEN** a spool with `first_used = 2024-03-15T14:32:00Z` is loaded into the edit form
- **THEN** the date input shows `2024-03-15` (date only, time discarded)

## ADDED Requirements

### Requirement: Material filter selection persists for the session
The selected material in the Material column dropdown SHALL be stored in `sessionStorage` and restored when the spool list is re-created or reloaded within the same tab session. The "Clear filters" control SHALL reset the material filter to "All" and remove its stored value.

#### Scenario: Selected material restored after navigation
- **WHEN** the user selects "PETG" and navigates away from the spool list and back
- **THEN** the Material dropdown shows "PETG" and only PETG spools are displayed

#### Scenario: Clear filters resets the material dropdown
- **WHEN** a material filter is active and the user clicks "Clear filters"
- **THEN** the Material dropdown returns to "All" and spools of every material are displayed

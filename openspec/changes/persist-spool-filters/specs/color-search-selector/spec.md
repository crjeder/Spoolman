## ADDED Requirements

### Requirement: Color level and picked color persist for the session
The color level selector value and the picked color SHALL be stored in `sessionStorage` and restored when the spool list is re-created or reloaded within the same tab session. When a stored level other than "Off" is restored, the color filter and implicit color-delta sort SHALL apply on first render without user action. The "Clear filters" control SHALL reset the level to "Off" (and, per `color-search-selector`, hide the picker and revert to column sort) and remove the stored values.

#### Scenario: Active color level restored after navigation
- **WHEN** the user sets the level to "Medium" with a picked color, navigates away and back
- **THEN** the level selector shows "Medium", the picker shows the stored color, and the list is filtered and delta-sorted accordingly

#### Scenario: Clear filters resets the color level
- **WHEN** a color level other than "Off" is active and the user clicks "Clear filters"
- **THEN** the level selector returns to "Off", the picker is hidden, and the list reverts to column-based sort

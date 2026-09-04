## ADDED Requirements

### Requirement: Multi-color editor in spool forms
The spool create and edit forms SHALL allow entering between 1 and 4 spool colors. The color section SHALL render one row per color, each row containing a native color input, a hex text input, and an opacity slider. A "+" button at the end of the last color row SHALL append a new color row, defaulting to `#000000` at full opacity. The "+" button SHALL be hidden or disabled once 4 rows are present. Every row except the first SHALL have a "−" button that removes that row. On submit, the form SHALL send all rows as the `colors` array in top-to-bottom order. The single `color_name` field SHALL remain unchanged and describes the spool's overall color.

#### Scenario: Add a second color
- **WHEN** the user opens the spool edit form and clicks the "+" button on the color row
- **THEN** a second color row appears with its own color input, hex input, and opacity slider

#### Scenario: Cap at four colors
- **WHEN** the form already shows 4 color rows
- **THEN** no "+" button is available to add a fifth row

#### Scenario: Remove an added color
- **WHEN** the form shows 2 or more color rows and the user clicks the "−" button on the second row
- **THEN** that row is removed and the remaining rows keep their values and order

#### Scenario: First color row cannot be removed
- **WHEN** the form shows exactly one color row
- **THEN** that row has no "−" button

#### Scenario: All colors saved in order
- **WHEN** the user sets row 1 to `#ff0000`, row 2 to `#00ff00`, row 3 to `#0000ff` and saves
- **THEN** the spool's `colors` array is `[#ff0000, #00ff00, #0000ff]` in that order

#### Scenario: Existing multi-color spool populates all rows
- **WHEN** a spool with `colors = [#ff0000, #00ff00]` is loaded into the edit form
- **THEN** the form shows two color rows pre-filled with `#ff0000` and `#00ff00`

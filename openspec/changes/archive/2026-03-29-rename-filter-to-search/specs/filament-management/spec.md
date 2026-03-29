## ADDED Requirements

### Requirement: Filament list search input
The frontend SHALL provide a text search input on the filament list page labeled "Search" (placeholder text "Search…"). The input SHALL filter the displayed filament rows client-side. A clear ("×") button SHALL appear inside the input when it has a value; clicking it SHALL empty the input and reset the list.

#### Scenario: Search filters filaments
- **WHEN** the user types in the search input
- **THEN** only filaments whose display name contains the typed text (case-insensitive) are shown

#### Scenario: Clear button appears with input
- **WHEN** the search input contains at least one character
- **THEN** a "×" clear button is visible inside the input

#### Scenario: Clear button hidden when empty
- **WHEN** the search input is empty
- **THEN** no clear button is shown

#### Scenario: Clear button resets list
- **WHEN** the user clicks the "×" clear button
- **THEN** the search input is emptied and all filaments are shown

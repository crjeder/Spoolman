## ADDED Requirements

### Requirement: Light theme uses logo-derived colour tokens
The CSS stylesheet SHALL define `:root` custom properties that match the Spoolman Light logo colour palette: sky-blue/cyan accent (`#4DC8E8`), charcoal text (`#3D4555`), and off-white sidebar background (`#F0F2F5`).

#### Scenario: Accent colour matches logo cyan
- **WHEN** the page is rendered without the `body.dark` class
- **THEN** the `--accent` CSS custom property resolves to `#4DC8E8`

#### Scenario: Text colour matches logo charcoal
- **WHEN** the page is rendered without the `body.dark` class
- **THEN** the `--fg` CSS custom property resolves to `#3D4555`

#### Scenario: Sidebar background matches logo off-white
- **WHEN** the page is rendered without the `body.dark` class
- **THEN** the `--sidebar-bg` CSS custom property resolves to `#F0F2F5`

### Requirement: Button text on accent background is readable
The `--accent-fg` token SHALL provide sufficient contrast against the cyan `--accent` colour to meet WCAG AA (4.5:1 for normal text).

#### Scenario: Dark text on cyan buttons
- **WHEN** a `.btn-primary` element is rendered in light mode
- **THEN** the foreground colour `#1a2433` on background `#4DC8E8` achieves at least 4.5:1 contrast ratio

### Requirement: Dark mode tokens are unchanged
The `body.dark` override block SHALL NOT be modified by this change.

#### Scenario: Dark mode accent is unchanged
- **WHEN** the page is rendered with the `body.dark` class
- **THEN** the `--accent` CSS custom property resolves to the existing dark-mode value (`#4d9fff`)

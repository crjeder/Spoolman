## Why

The app's default light theme uses generic greys and a blue accent that don't reflect the project's visual identity. The logo uses a distinctive palette — charcoal text, sky-blue/cyan accent, and an off-white background — that should be carried through into the UI for brand coherence.

## What Changes

- Update the `:root` CSS custom properties in the light theme to derive from the logo's color palette:
  - Accent colour: logo cyan (`#4DC8E8`) instead of generic blue
  - Foreground / text: logo charcoal (`#3D4555`) instead of near-black
  - Sidebar background: logo off-white (`#F0F2F5`) instead of plain grey
- Dark mode tokens are untouched — only the default (light) `:root` block changes
- No Rust code, no API, no behaviour changes

## Capabilities

### New Capabilities
- `light-theme-palette`: CSS design-token palette for the light theme derived from the Spoolman Light logo colours

### Modified Capabilities
<!-- No existing spec-level behaviour changes -->

## Impact

- **Files changed:** `style/spoolman.css` (`:root` token values only)
- **Dependencies:** None
- **APIs:** None — purely presentational
- **Build:** CSS-only change; no impact on Rust compilation or cargo-leptos build

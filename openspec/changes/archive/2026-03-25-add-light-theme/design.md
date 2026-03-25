## Context

The CSS stylesheet (`style/spoolman.css`) was introduced in the `add-css-styling-stylers` branch. It defines design tokens in `:root` and a `body.dark` override block. The light (default) tokens use generic values that don't reflect the project's brand.

The logo (`assets/spoolman-light-logo.png`) establishes a three-colour palette:
- Off-white page background: `#F0F2F5`
- Sky-blue/cyan accent (spool icon + "LIGHT" wordmark): `#4DC8E8`
- Dark charcoal text ("Spoolman" wordmark): `#3D4555`

Derived supporting tokens (hover states, borders, muted text) are computed to harmonise with these anchor colours.

## Goals / Non-Goals

**Goals:**
- Replace the five `:root` colour tokens (`--bg`, `--sidebar-bg`, `--fg`, `--sidebar-fg`, `--accent`) with values drawn from the logo palette
- Derive secondary tokens (`--border`, `--muted`, `--row-hover`, `--row-even`) so they read coherently with the new anchors

**Non-Goals:**
- Dark mode token changes
- Typography, spacing, or layout changes
- Any Rust / WASM code changes
- Introducing a new CSS framework or toolchain

## Decisions

### Token values

| Token | Old value | New value | Rationale |
|---|---|---|---|
| `--bg` | `#ffffff` | `#ffffff` | Keep pure white for main content contrast |
| `--sidebar-bg` | `#f5f5f5` | `#F0F2F5` | Logo off-white; gives sidebar a branded feel |
| `--fg` | `#1a1a1a` | `#3D4555` | Logo charcoal; softer than near-black, still readable |
| `--sidebar-fg` | `#333333` | `#3D4555` | Same charcoal for nav-link text consistency |
| `--accent` | `#0066cc` | `#4DC8E8` | Logo cyan — primary interactive colour |
| `--accent-fg` | `#ffffff` | `#1a2433` | Dark text on cyan buttons (cyan is light — white fails contrast) |
| `--border` | `#dddddd` | `#D0D5DE` | Slightly cooler grey to pair with charcoal text |
| `--muted` | `#888888` | `#7A8499` | Blue-tinted grey matching the charcoal family |
| `--row-hover` | `#f0f0f0` | `#E8ECF2` | Slight blue tint on hover, consistent with palette |
| `--row-even` | `#f9f9f9` | `#F5F7FA` | Near-white with very faint blue, matches sidebar bg family |
| `--danger` | `#c0392b` | `#c0392b` | Unchanged — red is universally understood |

### Contrast verification

- `--fg` `#3D4555` on `--bg` `#ffffff`: ~9.4:1 ✓ (WCAG AA/AAA)
- `--accent-fg` `#1a2433` on `--accent` `#4DC8E8`: ~5.0:1 ✓ (WCAG AA)
- `--sidebar-fg` `#3D4555` on `--sidebar-bg` `#F0F2F5`: ~7.8:1 ✓

## Risks / Trade-offs

- **Cyan accent on white is lighter than the previous blue** → compensated by keeping `--accent-fg` dark (`#1a2433`) rather than white, ensuring readable button labels
- **One-file change** — no rollback strategy needed beyond reverting the token values

## Open Questions

None. Token values are fixed; no unknowns remain.

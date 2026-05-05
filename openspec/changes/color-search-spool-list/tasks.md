## 1. Color utility module

- [x] 1.1 Create `crates/spoolman-client/src/utils/color.rs` with perceptual color distance (`color_distance`) supporting CIEDE2000, OkLab, and DIN99d algorithms; `hex_to_rgba` parser; `ColorAlgorithm` enum; `default_threshold_for` helper
- [x] 1.2 Declare `pub mod color;` in `crates/spoolman-client/src/utils/mod.rs`; declare `pub mod utils;` in `lib.rs`

## 2. Color filter in SpoolList

- [x] 2.1 Add `color_pick: RwSignal<String>` (default `"#000000"`) and `color_level: RwSignal<String>` (default `"off"`) signals
- [x] 2.2 Color picker popup on the "Color" column header — `<input type="color">` shown in a floating panel, toggled by clicking the header label
- [x] 2.3 Threshold level `<select>` inline in the column header (Off / Same / Close / Ballpark), reads threshold from `ColorThresholds` context per algorithm
- [x] 2.4 `filtered` closure: when `color_level != "off"`, keep spools where any `spool.colors` entry has `color_distance ≤ threshold`
- [x] 2.5 When color filter is active, `sorted` overrides column sort with ascending minimum ΔE distance (closest match first)

## 3. Reactive contexts in state.rs

- [x] 3.1 `ColorDistanceAlgorithm(RwSignal<ColorAlgorithm>)` context — algorithm selector (default DIN99d)
- [x] 3.2 `ColorThresholds` context — nine `RwSignal<f32>` values (3 algorithms × 3 levels) seeded from `default_threshold_for`; `get(level, algo)` helper

## 4. Update CHANGELOG and TODO

- [x] 4.1 Color search feature documented in `CHANGELOG.md` under `[1.5.0]`
- [x] 4.2 "Color search on spool list" item removed from `TODO.md`

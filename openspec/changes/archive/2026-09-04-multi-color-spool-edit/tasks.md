## 1. Shared color-row editor

- [x] 1.1 In `crates/spoolman-client/src/pages/spool.rs`, add a helper that renders a list of color rows from a `RwSignal<Vec<(RwSignal<String>, RwSignal<u8>)>>`: per-row native color input, hex text input (validated via `hex_to_rgba`), opacity slider, and the `alpha-pct` span. Reuse `.color-alpha-row`.
- [x] 1.2 Add a "−" button to every row except index 0 that removes that row.
- [x] 1.3 Add a "+" button after the last row that pushes `(#000000, 255)`; hide it when `len() == 4`.
- [x] 1.4 Add a helper that maps the rows vec to `Vec<Rgba>` (apply alpha, drop rows failing `hex_to_rgba`).

## 2. Wire into SpoolNew

- [x] 2.1 Replace `color_hex` / `color_alpha` scalar signals with the rows vec signal (seed with one default row).
- [x] 2.2 Update the SpoolmanDB prefill effect to set row 0's hex instead of `color_hex`.
- [x] 2.3 Render the color-row editor in place of the single color block.
- [x] 2.4 Build `colors` from the rows helper on submit.

## 3. Wire into SpoolEdit

- [x] 3.1 Replace `color_hex` / `color_alpha` scalar signals with the rows vec signal.
- [x] 3.2 In the prefill effect, populate one row per `sr.spool.colors` entry; if empty, seed one default row.
- [x] 3.3 Render the color-row editor in place of the single color block.
- [x] 3.4 Build `colors` from the rows helper on submit (still wrapped in `Some(...)`).

## 4. CSS

- [x] 4.1 Add styles for stacked color rows and the "+" / "−" buttons (small, icon-only, consistent with existing form controls).

## 5. Optional server guard

- [x] 5.1 In `crates/spoolman-server/src/store.rs` create and update paths, reject `colors.len() > 4` with a 400.

## 6. Verification

- [x] 6.1 `cargo check -p spoolman-types -p spoolman-server` and `cargo clippy` clean.
- [~] 6.2 Build the client (WSL/Docker) and manually verify: add/remove rows, 4-row cap, prefill of an existing multi-color spool, save order. (cargo check on wasm32 clean; full leptos build + browser check pending)
- [x] 6.3 Add a Playwright test in `tests/e2e/tests/` that adds a second color in the edit form and asserts both swatches render on the spool detail page.
- [x] 6.4 Update `CHANGELOG.md` under a new version and mark items in `TODO.md`.

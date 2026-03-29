## 1. Types

- [x] 1.1 Remove `remaining_pct` field from `SpoolResponse` struct in `crates/spoolman-types/src/responses.rs`
- [x] 1.2 Remove `remaining_pct` calculation from `SpoolResponse::new` in the same file

## 2. Server

- [x] 2.1 Fix any compile errors in `crates/spoolman-server/` caused by removed `remaining_pct` field (run `cargo check -p spoolman-server`)

## 3. Client — Spool List

- [x] 3.1 Remove `"remaining_pct"` from the `_visible_cols` initialiser in `SpoolList`
- [x] 3.2 Remove the `<ColHeader label="Remaining%" ...>` column header from the list table
- [x] 3.3 Remove the `<td class="num">{pct}</td>` cell from each list row
- [x] 3.4 Remove the `let pct = ...` local variable that formatted the percentage
- [x] 3.5 Remove the `"remaining_pct"` sort arm from the `sorted()` comparator

## 4. Client — Spool Detail

- [x] 4.1 Remove the `<dt>"Remaining %"</dt><dd>…</dd>` row from the `SpoolShow` detail grid

## 5. E2E Tests

- [x] 5.1 Search `tests/e2e/` for any reference to `remaining_pct` or "Remaining%" and remove or update those assertions

## 6. Verification

- [x] 6.1 Run `cargo check -p spoolman-types -p spoolman-server` — no errors
- [x] 6.2 Run `cargo check -p spoolman-client --target wasm32-unknown-unknown` — no errors
- [x] 6.3 Run `cargo clippy -p spoolman-types -p spoolman-server` — no warnings about dead code

## 1. Filament page

- [x] 1.1 Change `placeholder="Filter…"` to `placeholder="Search…"` in the filament list input (`pages/filament.rs`)
- [x] 1.2 Wrap the filament search `<input>` in a flex container `<div class="search-input-wrapper">`
- [x] 1.3 Add a reactive "×" clear button sibling to the input that is shown only when `ts.filter` is non-empty
- [x] 1.4 Wire the clear button's `on:click` to `ts.filter.set(String::new())`

## 2. Spool page

- [x] 2.1 Change `placeholder="Filter…"` to `placeholder="Search…"` in the spool list input (`pages/spool.rs`)
- [x] 2.2 Wrap the spool search `<input>` in a flex container `<div class="search-input-wrapper">`
- [x] 2.3 Add a reactive "×" clear button sibling to the input that is shown only when `ts.filter` is non-empty
- [x] 2.4 Wire the clear button's `on:click` to `ts.filter.set(String::new())`

## 3. Styling

- [x] 3.1 Add CSS for `.search-input-wrapper` (position relative, flex layout) and the clear button (absolute/inline, no border, cursor pointer) so the "×" sits inside the input's right edge

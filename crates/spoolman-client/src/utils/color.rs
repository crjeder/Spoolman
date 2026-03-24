use spoolman_types::models::Rgba;

/// Euclidean RGB distance between two colours; alpha is ignored.
/// Maximum possible value: sqrt(3 × 255²) ≈ 441.67.
pub fn rgb_distance(a: &Rgba, b: &Rgba) -> f32 {
    let dr = (a.r as f32) - (b.r as f32);
    let dg = (a.g as f32) - (b.g as f32);
    let db = (a.b as f32) - (b.b as f32);
    (dr * dr + dg * dg + db * db).sqrt()
}

/// Parse a `#rrggbb` hex string (as produced by `<input type="color">`)
/// into an `Rgba` with alpha = 255. Returns `None` for any other format.
pub fn hex_to_rgba(hex: &str) -> Option<Rgba> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Rgba { r, g, b, a: 255 })
}

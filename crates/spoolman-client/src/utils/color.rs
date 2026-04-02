use deltae::{DEMethod::DE2000, DeltaE, LabValue};
use oklab::{LinearRgb, Oklab};
use spoolman_types::models::Rgba;

// ── Color Distance Algorithms ────────────────────────────────────────────────

/// Supported color distance algorithms for perceptual color matching.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorAlgorithm {
    Ciede2000,
    OkLab,
    Din99d,
}

// ── sRGB → CIE L*a*b* conversion ───────────────────────────────────────────

/// Linearise a single sRGB channel value (0–255) via the exact IEC 61966-2-1
/// piecewise EOTF inverse. This is more accurate than the γ ≈ 2.2 approximation,
/// especially in near-black tones.
fn srgb_channel_to_linear(v: u8) -> f32 {
    let c = v as f32 / 255.0;
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055_f32).powf(2.4)
    }
}

/// Convert linear-light sRGB to CIE XYZ (D65 illuminant) using the
/// ITU-R BT.709 / IEC 61966-2-1 matrix. Returns (X, Y, Z) normalised so
/// that D65 white point is (0.95047, 1.00000, 1.08883).
fn linear_rgb_to_xyz(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let x = r * 0.412_456_4 + g * 0.357_576_1 + b * 0.180_437_5;
    let y = r * 0.212_672_9 + g * 0.715_152_2 + b * 0.072_174_9;
    let z = r * 0.019_333_9 + g * 0.119_192_0 + b * 0.950_304_1;
    (x, y, z)
}

/// CIE L*a*b* cube-root transfer function (f(t)).
fn lab_f(t: f32) -> f32 {
    const DELTA: f32 = 6.0 / 29.0;
    const DELTA2: f32 = DELTA * DELTA;
    const DELTA3: f32 = DELTA2 * DELTA;
    if t > DELTA3 {
        t.cbrt()
    } else {
        t / (3.0 * DELTA2) + 4.0 / 29.0
    }
}

/// Convert an `Rgba` to CIE L*a*b* (D65 illuminant). Alpha is ignored.
fn rgba_to_lab(c: &Rgba) -> LabValue {
    let r = srgb_channel_to_linear(c.r);
    let g = srgb_channel_to_linear(c.g);
    let b = srgb_channel_to_linear(c.b);
    let (x, y, z) = linear_rgb_to_xyz(r, g, b);

    // D65 white point normalisation
    let fx = lab_f(x / 0.950_47);
    let fy = lab_f(y / 1.000_00);
    let fz = lab_f(z / 1.088_83);

    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b_val = 200.0 * (fy - fz);

    // deltae::LabValue::new clamps/validates; use unchecked struct init to
    // avoid the Result when we know our inputs are always valid sRGB.
    LabValue { l, a, b: b_val }
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Perceptual color difference between two sRGB colours using CIEDE2000 (ΔE*00).
/// Alpha is ignored. Returns a value in [0, ~100]:
///   < 1   → imperceptible difference
///   < 10  → similar colours
///   > 25  → clearly distinct colours
pub fn color_distance(a: &Rgba, b: &Rgba, algo: ColorAlgorithm) -> f32 {
    match algo {
        ColorAlgorithm::Ciede2000 => ciede2000_distance(a, b),
        ColorAlgorithm::OkLab => oklab_distance(a, b),
        ColorAlgorithm::Din99d => din99d_distance(a, b),
    }
}

/// CIEDE2000 distance using the deltae crate.
fn ciede2000_distance(a: &Rgba, b: &Rgba) -> f32 {
    let lab_a = rgba_to_lab(a);
    let lab_b = rgba_to_lab(b);
    *DeltaE::new(&lab_a, &lab_b, DE2000).value()
}

/// OKLab distance using the oklab crate.
fn oklab_distance(a: &Rgba, b: &Rgba) -> f32 {
    let oklab_a = Oklab::from_linear_rgb(LinearRgb {
        r: srgb_channel_to_linear(a.r),
        g: srgb_channel_to_linear(a.g),
        b: srgb_channel_to_linear(a.b),
    });
    let oklab_b = Oklab::from_linear_rgb(LinearRgb {
        r: srgb_channel_to_linear(b.r),
        g: srgb_channel_to_linear(b.g),
        b: srgb_channel_to_linear(b.b),
    });
    let dl = oklab_a.l - oklab_b.l;
    let da = oklab_a.a - oklab_b.a;
    let db = oklab_a.b - oklab_b.b;
    (dl * dl + da * da + db * db).sqrt()
}

/// DIN99d distance using the DIN 6176:2001 transform on CIE L*a*b*.
fn din99d_distance(a: &Rgba, b: &Rgba) -> f32 {
    let lab_a = rgba_to_lab(a);
    let lab_b = rgba_to_lab(b);
    let din99d_a = lab_to_din99d(&lab_a);
    let din99d_b = lab_to_din99d(&lab_b);
    ((din99d_a.0 - din99d_b.0).powi(2) + (din99d_a.1 - din99d_b.1).powi(2) + (din99d_a.2 - din99d_b.2).powi(2)).sqrt()
}

/// Convert CIE L*a*b* to DIN99d coordinates.
fn lab_to_din99d(lab: &LabValue) -> (f32, f32, f32) {
    let l99 = 105.51 * (lab.l / 100.0 + 0.0158).ln();
    let a99 = 0.7 * (lab.a / 100.0) + 0.048 * (lab.b / 100.0);
    let b99 = 0.045 * (lab.a / 100.0) + 1.05 * (lab.b / 100.0);
    (l99, a99, b99)
}

/// Per-algorithm threshold constants for color similarity levels.
/// Returns the maximum ΔE value for the given level.
pub fn threshold_for(level: &str, algo: ColorAlgorithm) -> Option<f32> {
    match (level, algo) {
        ("off", _) => None,
        ("same", ColorAlgorithm::Ciede2000) => Some(2.3),
        ("close", ColorAlgorithm::Ciede2000) => Some(10.0),
        ("ballpark", ColorAlgorithm::Ciede2000) => Some(25.0),
        ("same", ColorAlgorithm::OkLab) => Some(0.02),
        ("close", ColorAlgorithm::OkLab) => Some(0.08),
        ("ballpark", ColorAlgorithm::OkLab) => Some(0.2),
        ("same", ColorAlgorithm::Din99d) => Some(2.0),
        ("close", ColorAlgorithm::Din99d) => Some(8.0),
        ("ballpark", ColorAlgorithm::Din99d) => Some(20.0),
        _ => None,
    }
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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RGBA color in sRGB color space, compatible with OpenTag3D / OpenPrintTag NFC tag standards.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// A filament material specification. No color fields — colors belong to Spool.
/// Fields align with OpenTag3D / OpenPrintTag NFC tag core and extended spec.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Filament {
    pub id: u32,
    pub manufacturer: Option<String>,
    pub material: Option<String>,
    pub material_modifier: Option<String>,
    /// Filament diameter in mm. Default 1.75.
    pub diameter: f32,
    /// Net weight of filament only (no spool tare), in grams.
    pub net_weight: Option<f32>,
    /// Density in g/cm³.
    pub density: f32,
    /// Nominal print temperature in °C.
    pub print_temp: Option<i32>,
    /// Nominal bed temperature in °C.
    pub bed_temp: Option<i32>,
    /// Empty spool weight in grams (informational, not used for tracking).
    pub spool_weight: Option<f32>,
    pub min_print_temp: Option<i32>,
    pub max_print_temp: Option<i32>,
    pub min_bed_temp: Option<i32>,
    pub max_bed_temp: Option<i32>,
    pub registered: DateTime<Utc>,
    pub comment: Option<String>,
}

/// A physical spool of filament. Carries color information and weight tracking.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Spool {
    pub id: u32,
    pub filament_id: u32,
    pub location_id: Option<u32>,
    /// Colors of this spool (1–4 RGBA values). Physically attached to this spool instance.
    pub colors: Vec<Rgba>,
    /// Human-readable color name (e.g. "Galaxy Black").
    pub color_name: Option<String>,
    /// Total weight at creation time (spool + filament), in grams (scale reading).
    pub initial_weight: f32,
    /// Latest scale reading (spool + filament), in grams.
    pub current_weight: f32,
    pub registered: DateTime<Utc>,
    pub first_used: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
    pub comment: Option<String>,
    pub archived: bool,
}

/// A storage location for spools.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    pub id: u32,
    /// Non-empty display name.
    pub name: String,
}

/// DataStore schema version, stored in the JSON file meta block.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreMeta {
    pub schema_version: u32,
}

/// Top-level JSON storage format.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DataStore {
    pub meta: StoreMeta,
    pub filaments: Vec<Filament>,
    pub spools: Vec<Spool>,
    pub locations: Vec<Location>,
    /// Key-value settings (e.g. "currency_symbol" → "€").
    pub settings: HashMap<String, String>,
}

impl Default for StoreMeta {
    fn default() -> Self {
        Self { schema_version: 1 }
    }
}

impl Filament {
    /// Derived display name: "{manufacturer} {material} {material_modifier}" with absent fields omitted.
    pub fn display_name(&self) -> String {
        [
            self.manufacturer.as_deref(),
            self.material.as_deref(),
            self.material_modifier.as_deref(),
        ]
        .iter()
        .filter_map(|s| *s)
        .collect::<Vec<_>>()
        .join(" ")
    }
}

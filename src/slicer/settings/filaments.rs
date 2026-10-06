use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use crate::settings::ParserError;
use crate::slicer::settings::{PrusaSection};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tuple<T: FromStr> {
    left: T,
    right: Option<T>,
}

impl<T> FromStr for Tuple<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');

        let left = parts
            .next()
            .ok_or_else(|| "missing left value".to_string())?
            .trim()
            .parse::<T>()
            .map_err(|e| format!("invalid left value: {e}"))?;

        let right = match parts.next() {
            Some(value) => Some(
                value
                    .trim()
                    .parse::<T>()
                    .map_err(|e| format!("invalid right value: {e}"))?,
            ),
            None => None,
        };

        if parts.next().is_some() {
            return Err("expected one or two comma-separated values".to_string());
        }

        Ok(Self { left, right })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilamentType {
    /// Acrylonitrile Butadiene Styrene (ABS).
    ///
    /// A common engineering thermoplastic with good toughness and
    /// temperature resistance. Prusa recommends an enclosure for ABS.
    /// See: https://help.prusa3d.com/filament-material-guide
    ABS,

    /// Acrylonitrile Styrene Acrylate (ASA).
    ///
    /// UV-resistant engineering thermoplastic commonly used for outdoor
    /// and mechanically exposed parts.
    /// See: https://help.prusa3d.com/filament-material-guide
    ASA,

    /// CPE — Co-polyester / Chlorinated Polyethylene-based filament.
    CPE,

    /// EDGE — material/profile family used by PrusaSlicer.
    EDGE,

    /// Flexible filament.
    FLEX,

    /// High Impact Polystyrene (HIPS).
    HIPS,

    /// nGen — Eastman Amphora AM3300-based copolyester filament.
    NGEN,

    /// Nylon / Polyamide (PA).
    Nylon,

    /// Polyamide (PA).
    PA,

    /// Polyether Block Amide (PEBA).
    PEBA,

    /// Polycarbonate (PC).
    PC,

    /// Polyethylene Terephthalate (PET).
    PET,

    /// Polyethylene Terephthalate Glycol-modified (PETG).
    PETG,

    /// Polycaprolactone / specialty PLA Tough formulation.
    PLATough,

    /// Polycyclohexylenedimethylene Terephthalate Glycol-modified (PCTG).
    PCTG,

    /// Polypropylene (PP).
    PP,

    /// Polyvinyl Alcohol (PVA).
    PVA,

    /// Polyvinyl Butyral (PVB).
    PVB,

    /// Polylactic Acid (PLA).
    PLA,

    /// Thermoplastic Polyurethane (TPU).
    TPU,

    /// Polyetherimide (PEI).
    PEI,

    /// Metal-filled filament.
    Metal,
    Glaze,
    /// Other or vendor-specific filament material.
    Other(String),
}

impl FromStr for FilamentType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = value.trim();

        let filament_type = match normalized.to_ascii_uppercase().as_str() {
            "ABS" => Self::ABS,
            "ASA" => Self::ASA,
            "CPE" => Self::CPE,
            "EDGE" => Self::EDGE,
            "FLEX" | "FLEXIBLE" => Self::FLEX,
            "HIPS" => Self::HIPS,
            "NGEN" => Self::NGEN,
            "NYLON" => Self::Nylon,
            "PA" | "POLYAMIDE" => Self::PA,
            "PEBA" => Self::PEBA,
            "PC" | "POLYCARBONATE" => Self::PC,
            "PET" => Self::PET,
            "PETG" => Self::PETG,
            "PLATOUGH" | "PLA TOUGH" => Self::PLATough,
            "PCTG" => Self::PCTG,
            "PP" | "POLYPROPYLENE" => Self::PP,
            "PVA" => Self::PVA,
            "PVB" => Self::PVB,
            "PLA" => Self::PLA,
            "TPU" => Self::TPU,
            "PEI" => Self::PEI,
            "METAL" | "METAL FILLED" => Self::Metal,

            // Keep vendor/custom material names rather than throwing
            // information away.
            _ => Self::Other(normalized.to_owned()),
        };

        Ok(filament_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalSettings {
    pub temperature: Option<Tuple<f32>>,
    pub bed_temperature: Option<Tuple<f32>>,
    pub first_layer_bed_temperature: Option<Tuple<f32>>,
    pub first_layer_temperature: Option<Tuple<f32>>,
    pub idle_temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowSettings {
    pub max_volumetric_speed: Option<Tuple<f32>>,
    pub max_volumetric_extrusion_rate: Option<f32>,
    // need to handle 1,1
    pub extrusion_multiplier: Option<Tuple<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanSettings {
    pub fan_always_on: Option<Tuple<u8>>,
    pub min_fan_speed: Option<Tuple<u8>>,
    pub max_fan_speed: Option<Tuple<u8>>,
    pub bridge_fan_speed: Option<String>,
    pub disable_first_layers: Option<Tuple<u32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilamentIdentity {
    pub name: String,
    pub vendor: Option<String>,
    pub cost: Option<f32>,
    pub colour: String,
    // need to handle 0,0
    pub density: Option<Tuple<f32>>,
    pub spool_weight: Option<String>,
    pub filament_type: Option<FilamentType>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filament {
    pub inherits: Vec<String>,
    pub identity: FilamentIdentity,
    pub thermal: ThermalSettings,
    pub fan: FanSettings,
    pub flow: FlowSettings,
    pub extra: HashMap<String, String>,
}

impl Filament {
    pub fn from_ini_section(section: PrusaSection) -> Result<Self, ParserError> {
        Self::from_properties(section.id.unwrap(), &section.properties)
    }
    /// Parse a PrusaSlicer filament profile from its key/value properties.
    ///
    /// Recognized properties are converted into their strongly typed fields.
    /// Properties which are not recognized are retained in [`Self::extra`]
    /// so that newer PrusaSlicer settings are not lost.
    pub fn from_properties(
        name: String,
        properties: &HashMap<String, String>,
    ) -> Result<Self, ParserError> {
        let mut extra = properties.clone();

        // -----------------------------------------------------------------
        // Helpers
        // -----------------------------------------------------------------

        fn take(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
        ) -> Option<String> {
            for key in keys {
                if let Some(value) = properties.get(*key) {
                    extra.remove(*key);
                    return Some(value.clone());
                }
            }

            None
        }

        fn required_string(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
            field: &str,
        ) -> Result<String, String> {
            take(properties, extra, keys)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(|| {
                    format!(
                        "missing required filament property `{field}` \
                         (accepted keys: {})",
                        keys.join(", ")
                    )
                })
        }

        fn parse_tuple<T: FromStr>(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
            field: &str,
        ) -> Result<Option<Tuple<T>>, ParserError>
        where <T as FromStr>::Err: std::fmt::Display
        {
            let Some(value) = take(properties, extra, keys) else {
                return Ok(None);
            };
            if value.is_empty() || value == "nil" {
                return Ok(None);
            }

            Ok(value
                .trim()
                .parse::<Tuple<T>>()
                .map(Some)
                .map_err(|error| {
                    format!(
                        "invalid value `{value}` for filament field \
                         `{field}`: {error}"
                    )
                })?
            )
        }

        fn optional_string(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
        ) -> Option<String> {
            take(properties, extra, keys)
                .filter(|value| !value.trim().is_empty())
        }

        fn parse_f32(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
            field: &str,
        ) -> Result<Option<f32>, String> {
            let Some(value) = take(properties, extra, keys) else {
                return Ok(None);
            };
            match value.as_str() {
                "" => return Ok(None),
                "nil" => return Ok(None),
                _ => {},
            }
            if value.is_empty() {
                return Ok(None);
            }
            value
                .trim()
                .parse::<f32>()
                .map(Some)
                .map_err(|error| {
                    format!(
                        "invalid value `{value}` for filament field \
                         `{field}`: {error}"
                    )
                })
        }

        fn parse_u8(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
            field: &str,
        ) -> Result<Option<u8>, String> {
            let Some(value) = take(properties, extra, keys) else {
                return Ok(None);
            };
            if value.is_empty() || value == "nil" {
                return Ok(None);
            }

            value
                .trim()
                .parse::<u8>()
                .map(Some)
                .map_err(|error| {
                    format!(
                        "invalid value `{value}` for filament field \
                         `{field}`: {error}"
                    )
                })
        }

        fn parse_u32(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
            field: &str,
        ) -> Result<Option<u32>, String> {
            let Some(value) = take(properties, extra, keys) else {
                return Ok(None);
            };

            if value.is_empty() || value == "nil" {
                return Ok(None);
            }

            value
                .trim()
                .parse::<u32>()
                .map(Some)
                .map_err(|error| {
                    format!(
                        "invalid value `{value}` for filament field \
                         `{field}`: {error}"
                    )
                })
        }

        fn parse_bool(
            properties: &HashMap<String, String>,
            extra: &mut HashMap<String, String>,
            keys: &[&str],
            default: bool,
            field: &str,
        ) -> Result<bool, String> {
            let Some(value) = take(properties, extra, keys) else {
                return Ok(default);
            };

            match value.trim().to_ascii_lowercase().as_str() {
                "1" | "true" | "yes" | "on" => Ok(true),
                "0" | "false" | "no" | "off" => Ok(false),
                _ => Err(format!(
                    "invalid boolean value `{value}` for filament \
                     field `{field}`"
                )),
            }
        }

        // -----------------------------------------------------------------
        // Identity
        // -----------------------------------------------------------------

        

        let vendor = optional_string(
            properties,
            &mut extra,
            &["filament_vendor", "vendor"],
        );

        let cost = parse_f32(
            properties,
            &mut extra,
            &["filament_cost", "cost"],
            "cost",
        ).unwrap_or(None);

        let colour = optional_string(
            properties,
            &mut extra,
            &[
                "filament_colour",
                "filament_color",
                "colour",
                "color",
            ],
        )
        .unwrap_or_default();

        let density = parse_tuple(
            properties,
            &mut extra,
            &["filament_density", "density"],
            "filament_density"
        )?;

        let spool_weight = optional_string(
            properties,
            &mut extra,
            &[
                "filament_spool_weight",
                "spool_weight",
            ],
        );

        let filament_type_string: Option<String> = optional_string(
            properties,
            &mut extra,
            &[
                "filament_type",
                "filament_material",
                "type",
                "material",
            ]
        );

        let filament_type = filament_type_string.clone()
            .map(|val|
            val.parse::<FilamentType>()
            .unwrap_or(FilamentType::Other(filament_type_string.expect("failed to get val")))
        );
        let notes = optional_string(
            properties,
            &mut extra,
            &["filament_notes", "notes"],
        );

        // -----------------------------------------------------------------
        // Thermal settings
        // -----------------------------------------------------------------

        let temperature = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_temperature",
                "temperature",
            ],
            "temperature"
        )?;

        let bed_temperature = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_bed_temperature",
                "bed_temperature",
            ],
            "bed_temperature",
        )?;

        let first_layer_bed_temperature = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_first_layer_bed_temperature",
                "first_layer_bed_temperature",
            ],
            "first_layer_bed_temperature",
        )?;

        let first_layer_temperature = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_first_layer_temperature",
                "first_layer_temperature",
            ],
            "first_layer_temperature",
        )?;

        let idle_temperature = parse_f32(
            properties,
            &mut extra,
            &[
                "filament_idle_temperature",
                "idle_temperature",
            ],
            "idle_temperature",
        )?;

        // -----------------------------------------------------------------
        // Fan settings
        // -----------------------------------------------------------------

        let fan_always_on = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_fan_always_on",
                "fan_always_on",
            ],
            "fan_always_on",
        )?;

        let min_fan_speed = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_min_fan_speed",
                "min_fan_speed",
            ],
            "min_fan_speed",
        )?;

        let max_fan_speed = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_max_fan_speed",
                "max_fan_speed",
            ],
            "max_fan_speed",
        )?;

        let bridge_fan_speed = optional_string(
            properties,
            &mut extra,
            &[
                "filament_bridge_fan_speed",
                "bridge_fan_speed",
            ]
        );

        let disable_first_layers = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_disable_fan_first_layers",
                "disable_fan_first_layers",
                "disable_first_layers",
            ],
            "disable_first_layers",
        )?;

        // -----------------------------------------------------------------
        // Flow settings
        // -----------------------------------------------------------------

        let max_volumetric_speed = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_max_volumetric_speed",
                "max_volumetric_speed",
            ],
            "max_volumetric_speed",
        )?;

        let max_volumetric_extrusion_rate = parse_f32(
            properties,
            &mut extra,
            &[
                "filament_max_volumetric_extrusion_rate",
                "max_volumetric_extrusion_rate",
            ],
            "max_volumetric_extrusion_rate",
        )?;

        let extrusion_multiplier = parse_tuple(
            properties,
            &mut extra,
            &[
                "filament_extrusion_multiplier",
                "extrusion_multiplier",
            ],
            "extrusion_multiplier"
        )?;

        // -----------------------------------------------------------------
        // Inheritance
        // -----------------------------------------------------------------

        let inherits = optional_string(
            properties,
            &mut extra,
            &["filament_inherits", "inherits"],
        )
        .map(|value| {
            value
                .split(';')
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default();

        Ok(Self {
            inherits,

            identity: FilamentIdentity {
                name,
                vendor,
                cost,
                colour,
                density,
                spool_weight,
                filament_type,
                notes,
            },

            thermal: ThermalSettings {
                temperature,
                bed_temperature,
                first_layer_bed_temperature,
                first_layer_temperature,
                idle_temperature,
            },

            fan: FanSettings {
                fan_always_on,
                min_fan_speed,
                max_fan_speed,
                bridge_fan_speed,
                disable_first_layers,
            },

            flow: FlowSettings {
                max_volumetric_speed,
                max_volumetric_extrusion_rate,
                extrusion_multiplier,
            },

            extra,
        })
    }
}
use crate::settings::ParserError;
use crate::settings::PrusaSection;
use serde::{Serialize, Deserialize};
pub enum Technology {
    FFF,
    SLA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrinterModel {
    name: String,
    variants: Vec<String>,
    /// probably should be an enum appears as FFF
    technology: Option<String>,
    /// examples MK4
    family: Option<String>,
    /// may alternatively be local path buf
    bed_model: Option<String>,
    /// may alternatively be best as a PathBuf
    bed_texture: Option<String>,
    /// used when selecting a printer
    thumbnail: Option<String>,
    /// will correct later for proper formating.
    default_materials: Vec<String>,
}

impl PrinterModel {
    pub fn from_ini_section(section: PrusaSection) -> Result<PrinterModel, ParserError> {
        if section.kind() != "printer_model" {
            return Err(ParserError::InvalidSectionKind(
                section.kind().clone(),
                "printer_model".into(),
            ));
        }

        let get = |name: &str| {
            section
                .properties
                .get(name)
                .cloned()
                .ok_or_else(|| ParserError::MissingField(name.into()))
        };

        let variants = section
            .properties
            .get("variants")
            .map(|value| {
                value
                    .split(';')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(|value| {
                        value.parse::<String>().map_err(|_| {
                            ParserError::InvalidField(
                                "variants".into(),
                                value.into(),
                            )
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();

        let default_materials = section
            .properties
            .get("default_materials")
            .map(|value| {
                value
                    .split(';')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_owned)
                    .collect()
            })
            .unwrap_or_default();

        Ok(Self {
            name: get("name")?,
            variants,
            technology: section.properties.get("technology").cloned(),
            family: section.properties.get("family").cloned(),
            bed_model: section.properties.get("bed_model").cloned(),
            bed_texture: section.properties.get("bed_texture").cloned(),
            thumbnail: section.properties.get("thumbnail").cloned(),
            default_materials,
        })
    }
}
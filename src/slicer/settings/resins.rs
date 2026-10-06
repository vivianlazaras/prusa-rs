use serde::{Serialize, Deserialize};
use std::collections::HashMap;
pub enum ResinType {
    Tough,
    Casting,
    Medical,
    Flexible,
    Other(String)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resin {
    name: String,
    inherits: Vec<String>,
    exposure_time: f32,
    initial_exposure_time: f32,
    vendor: Option<String>,
    material_type: ResinType,
    colour: Option<String>,
    
}
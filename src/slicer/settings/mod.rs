pub mod printers;
pub mod filaments;
use ini::Ini;
use std::{
    collections::HashMap,
    path::Path,
};
use thiserror::Error;
use walkdir::WalkDir;


pub type LiveSettings = HashMap<String, HashMap<String, PrusaIni>>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("invalid section kind expected: {0} found: {1}")]
    InvalidSectionKind(String, String),
    #[error("missing required field: {0}")]
    MissingField(String),
    #[error("invalid value for field: {0} value: {1}")]
    InvalidField(String, String),
    #[error("some other error: {0}")]
    Other(String),
}

impl From<std::string::String> for ParserError {
    fn from(val: String) -> ParserError {
        ParserError::Other(val)
    }
}

#[derive(Debug)]
pub struct PrusaIni {
    pub sections: Vec<PrusaSection>,
}

#[derive(Debug)]
pub struct PrusaSection {
    pub name: String,
    pub kind: String,
    pub id: Option<String>,
    pub properties: HashMap<String, String>,
}

impl PrusaSection {
    pub fn kind(&self) -> &String {
        &self.kind
    }
}

impl PrusaIni {
    pub fn parse(input: &str) -> Result<Self, ini::ParseError> {
        let ini = Ini::load_from_str(input)?;

        let sections = ini
            .iter()
            .filter_map(|(name, properties)| {
                let name = name.as_deref()?;

                let (kind, id) = match name.split_once(':') {
                    Some((kind, id)) => (kind.to_owned(), Some(id.to_owned())),
                    None => (name.to_owned(), None),
                };

                let properties = properties
                    .iter()
                    .map(|(key, value)| (key.to_owned(), value.to_owned()))
                    .collect();

                Some(PrusaSection {
                    name: name.to_owned(),
                    kind,
                    id,
                    properties,
                })
            })
            .collect();

        Ok(Self { sections })
    }

    /// Open path to the Prusa settings git repo.
    ///
    /// Expected layout:
    ///
    /// ```text
    /// base/
    /// └── live/
    ///     ├── PrusaResearch/
    ///     │   ├── 2.0.0.ini
    ///     │   └── 2.1.0.ini
    ///     └── OtherCompany/
    ///         └── 1.0.0.ini
    /// ```
    pub async fn import_live<P: AsRef<Path>>(
        base: P,
    ) -> Result<LiveSettings, Box<dyn std::error::Error>> {
        let live = base.as_ref().join("live");
        let mut result = LiveSettings::new();

        for entry in WalkDir::new(&live)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) != Some("ini") {
                continue;
            }

            let relative = path.strip_prefix(&live)?;

            let Some(company) = relative
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|name| name.to_str())
            else {
                continue;
            };

            let Some(version) = path
                .file_stem()
                .and_then(|name| name.to_str())
            else {
                continue;
            };

            let contents = tokio::fs::read_to_string(path).await?;
            let ini = Self::parse(&contents)?;

            result
                .entry(company.to_owned())
                .or_default()
                .insert(version.to_owned(), ini);
        }

        Ok(result)
    }
}
use crate::citation::{Citation, Citations};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PandocDefaults {
    pub input_file: Option<String>,
    pub input_files: Option<Vec<String>>,
    pub metadata: Document,
}

impl PandocDefaults {
    pub fn asset_paths(&self, base_dir: &str) -> Vec<String> {
        let mut paths = vec![];
        // TODO: Refactor to avoid duplication
        for path in self.input_file.iter() {
            let path = format!("{}/{}", base_dir, path);
            let path = fs::canonicalize(path).unwrap();
            let path = path.parent().unwrap();
            paths.push(path.to_string_lossy().into())
        }
        for path in self.input_files.iter().flatten() {
            let path = format!("{}/{}", base_dir, path);
            let path = fs::canonicalize(path).unwrap();
            let path = path.parent().unwrap();
            paths.push(path.to_string_lossy().into())
        }
        paths
    }
}

/// Documents are exposed via the API
/// All fields other than a unique "id" are optional
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    /// Each document must have a unique ID
    pub id: String,

    #[serde(default)]
    pub collections: Vec<String>,

    #[serde(default)]
    pub title: Option<String>,

    #[serde(default)]
    pub author: Vec<String>,

    #[serde(default)]
    pub date: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub bibliography: Option<String>,

    #[serde(skip)]
    pub pandoc_defaults_str: String,

    #[serde(skip)]
    pub asset_paths: Vec<String>,

    #[serde(skip)]
    pub base_dir: String,
}

impl Document {
    pub fn convert(&self, args: Vec<&str>) -> anyhow::Result<Vec<u8>> {
        let data =
            crate::pandoc::run_pandoc_with_defaults(&self.base_dir, args, &self.pandoc_defaults_str)?;
        Ok(data)
    }

    pub fn citations(&self) -> Result<Option<Vec<Citation>>> {
        match &self.bibliography {
            Some(bibfile) => {
                let bibstr = crate::pandoc::run_pandoc_citeproc(&self.base_dir, &bibfile)?;
                let citations: Citations = serde_yaml::from_str(&bibstr)?;
                Ok(Some(citations.references))
            }
            None => Ok(None),
        }
    }
}

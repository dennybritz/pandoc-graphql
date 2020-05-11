use anyhow::{anyhow, Context, Result};
use chrono::prelude::*;
use heck::KebabCase;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs::File;

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FormatKind {
    CommonMark,
    Pandoc,
}

impl FormatKind {
    // This is only necessary because we need to call a functio
    // for serde's default below
    pub fn pandoc() -> Self {
        FormatKind::Pandoc
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    #[serde(default = "FormatKind::pandoc")]
    pub format: FormatKind,

    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub authors: Option<Vec<Author>>,
    pub bibtex: Option<String>,
    pub bibliography: Option<String>,
    pub commonmark: Option<CommonMarkConfig>,
    pub pandoc: Option<BTreeMap<String, serde_yaml::Value>>,

    #[serde(skip)]
    pub base_dir: String,
    #[serde(skip)]
    pub assets: Vec<Asset>,
}

impl Post {
    pub fn slug(&self) -> String {
        self.slug.clone().unwrap_or(self.title.to_kebab_case())
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

    fn make_pandoc_config(&self) -> Result<BTreeMap<String, serde_yaml::Value>> {
        let mut config = self.pandoc.clone().unwrap_or(BTreeMap::new());

        let mut metadata: BTreeMap<String, serde_yaml::Value> = config
            .get("metadata")
            .cloned()
            .map(|e| serde_yaml::from_value(e).unwrap())
            .unwrap_or(BTreeMap::new());

        metadata
            .entry("title".into())
            .or_insert(serde_yaml::Value::String(self.title.clone()));

        metadata
            .entry("date".into())
            .or_insert(serde_yaml::Value::String(self.date.clone()));

        if let Some(desc) = &self.description {
            metadata
                .entry("description".into())
                .or_insert(serde_yaml::Value::String(desc.clone()));
            metadata
                .entry("abstract".into())
                .or_insert(serde_yaml::Value::String(desc.clone()));
        }

        if let Some(authors) = &self.authors {
            let authors: Vec<String> = authors.iter().map(|a| format!("{}", a.name)).collect();
            let authors = serde_yaml::to_value(authors).unwrap();
            metadata.entry("author".into()).or_insert(authors);
        }

        config.insert("metadata".into(), serde_yaml::to_value(metadata)?);

        // log::info!("{:?}", config);

        Ok(config)
    }

    pub fn html(&self) -> Result<String> {
        match &self.format {
            FormatKind::Pandoc => {
                let config = self.make_pandoc_config()?;
                let buf = crate::pandoc::run_pandoc(&self.base_dir, &config, "html")?;
                let html = String::from_utf8(buf)?;
                Ok(html)
            }
            FormatKind::CommonMark => {
                let md_config = self
                    .commonmark
                    .as_ref()
                    .ok_or(anyhow!("no markdown config"))?;
                Ok(crate::pandoc::markdown_to_html(&self.base_dir, md_config)?)
            }
        }
    }

    pub fn convert(&self, format: &str) -> Result<String> {
        match self.format {
            FormatKind::Pandoc => {
                let config = self.make_pandoc_config()?;
                let buf = crate::pandoc::run_pandoc(&self.base_dir, &config, &format)?;
                Ok(base64::encode(buf))
            }
            _ => {
                let html = self.html()?;
                let config = self.make_pandoc_config()?;
                crate::pandoc::convert_from_html(html.as_ref(), &config, format)
            }
        }
    }
}

/// A citation in CSL (https://citationstyles.org/) format
/// Also see
/// - https://citeproc-js.readthedocs.io/en/latest/csl-json/markup.html
/// - https://github.com/jgm/pandoc-citeproc/blob/master/man/pandoc-citeproc.1.md
#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
#[serde(rename_all = "kebab-case")]
pub struct Citation {
    pub id: String,
    pub title: Option<String>,
    pub author: Option<Vec<CitationAuthor>>,
    pub container_title: Option<String>,
    pub publisher: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub issued: Option<Vec<CitationIssued>>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
}

#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
pub struct CitationAuthor {
    pub family: Option<String>,
    pub given: Option<String>,
}

#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
#[serde(rename_all = "kebab-case")]
pub struct CitationIssued {
    pub year: Option<i32>,
    pub month: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Citations {
    pub references: Vec<Citation>,
}

#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
pub struct Author {
    pub name: String,
    pub email: Option<String>,
    pub url: Option<String>,
    pub affiliation: Option<String>,
    pub affiliation_url: Option<String>,
}

#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
pub struct Asset {
    pub path: String,
    pub absolute_path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommonMarkConfig {
    pub path: String,
}

pub fn source_from_directory(base: &str) -> Result<Vec<Post>> {
    let build_file_glob = format!("{}/**/build.yaml", base);
    log::info!("sourcing files from directory: {}", base);
    let build_file = glob::glob(build_file_glob.as_str())?;
    build_file
        .filter_map(|x| x.ok())
        .map(|path| {
            let base_dir = format!("{}", path.parent().unwrap().display());
            log::info!("sourcing: {}", base_dir);
            let mut post_data: Post = serde_yaml::from_reader(File::open(&path)?)?;

            // Validate date
            NaiveDate::parse_from_str(&post_data.date, "%Y-%m-%d")
                .with_context(|| format!("failed parse date in: {}", base_dir))?;

            let asset_path = format!("{}/assets", &base_dir);
            if std::path::Path::new(&asset_path).exists() {
                post_data.assets = source_assets(&base_dir)?;
            }
            post_data.base_dir = base_dir;
            return Ok(post_data);
        })
        .filter_map(|post: Result<Post>| match post {
            Ok(post) => Some(Ok(post)),
            Err(e) => {
                log::error!("{}", e);
                None
            }
        })
        .collect()
}

pub fn source_assets(base: &str) -> Result<Vec<Asset>> {
    let files = glob::glob(format!("{}/assets/**/*", base).as_str())?;
    files
        .filter_map(|f| f.ok())
        .map(|file| {
            let absolute_path = format!("{}", file.canonicalize()?.display());
            let file_name = format!("{}", file.display());
            let asset = Asset {
                path: file_name.replace(format!("{}/assets/", base).as_str(), ""),
                absolute_path,
            };
            Ok(asset)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_deserialize_citation() {
        init();
        let citation_raw = r###"
            id: Ruiz_2015
            type: article-journal
            author:
            - family: Ruiz
              given: Eduardo J.
            - family: Huterer
              given: Dragan
            issued:
            - year: 2015
              month: 3
            title: Testing the dark energy consistency with geometry and growth
            container-title: Physical Review D
            publisher: American Physical Society (APS)
            volume: '91'
            issue: '6'
            URL: http://dx.doi.org/10.1103/PhysRevD.91.063009
            DOI: 10.1103/physrevd.91.063009
            ISSN: 1550-2368      
        "###;
        let c: Citation = serde_yaml::from_str(citation_raw).expect("failed to parse citation");

        assert_eq!(c.id, "Ruiz_2015");
        let author = c.author.unwrap();
        assert_eq!(author.len(), 2);
        assert_eq!(
            author.get(0).and_then(|a| a.family.as_ref()),
            Some(&String::from("Ruiz"))
        );
        assert_eq!(
            author.get(0).and_then(|a| a.given.as_ref()),
            Some(&String::from("Eduardo J."))
        );
        assert_eq!(
            author.get(1).and_then(|a| a.family.as_ref()),
            Some(&String::from("Huterer"))
        );
        assert_eq!(
            author.get(1).and_then(|a| a.given.as_ref()),
            Some(&String::from("Dragan"))
        );
        assert_eq!(
            c.issued.as_ref().unwrap().get(0).and_then(|x| x.year),
            Some(2015)
        );
        assert_eq!(
            c.issued.as_ref().unwrap().get(0).and_then(|x| x.month),
            Some(3)
        );

        assert_eq!(
            c.title.unwrap(),
            "Testing the dark energy consistency with geometry and growth"
        );
        assert_eq!(c.container_title.unwrap(), "Physical Review D");
        assert_eq!(c.publisher.unwrap(), "American Physical Society (APS)");
        assert_eq!(c.volume.unwrap(), "91");
        assert_eq!(c.issue.unwrap(), "6");
        assert_eq!(
            c.url.unwrap(),
            "http://dx.doi.org/10.1103/PhysRevD.91.063009"
        );
        assert_eq!(c.doi.unwrap(), "10.1103/physrevd.91.063009");
    }

    #[test]
    pub fn test_source_assets() {
        init();
        let assets = source_assets("content/commonmark").expect("failed to get assets");
        assert_eq!(assets.len(), 1);

        assets
            .iter()
            .find(|a| a.path == "hello-md.png")
            .expect("expected an asset with hello-md.png file name");
    }

    #[test]
    pub fn test_source_from_directory() {
        init();
        let posts = source_from_directory("content/").expect("failed to get posts");
        assert_eq!(posts.len(), 4);

        let md_post = posts
            .iter()
            .find(|p| p.base_dir == "content/commonmark")
            .unwrap();
        assert_eq!(md_post.title, "Writing in CommonMark");
        assert_eq!(md_post.format, FormatKind::CommonMark);

        let pandoc_md_post = posts
            .iter()
            .find(|p| p.base_dir == "content/markdown")
            .unwrap();
        assert_eq!(pandoc_md_post.title, "Writing in Markdown");
        assert_eq!(pandoc_md_post.format, FormatKind::Pandoc);
    }
}

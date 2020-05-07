use anyhow::Result;
use heck::KebabCase;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FormatKind {
    Markdown,
    Pandoc,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub format: FormatKind,

    pub title: String,
    pub date: String,
    pub description: Option<String>,
    pub slug: Option<String>,
    pub url: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub authors: Option<Vec<Author>>,
    pub bibtex: Option<String>,
    pub markdown: Option<MarkdownConfig>,
    pub pandoc: Option<serde_yaml::Value>,

    #[serde(skip)]
    pub base_dir: String,
    #[serde(skip)]
    pub assets: Vec<Asset>,
}

impl Post {
    pub fn slug(&self) -> String {
        self.slug.clone().unwrap_or(self.title.to_kebab_case())
    }
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
pub struct MarkdownConfig {
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
            let mut post_data: Post = serde_yaml::from_reader(File::open(&path)?)?;
            let asset_path = format!("{}/assets", &base_dir);
            if std::path::Path::new(&asset_path).exists() {
                post_data.assets = source_assets(&base_dir)?;
            }
            post_data.base_dir = base_dir;

            return Ok(post_data);
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
    pub fn test_source_assets() {
        init();
        let assets = source_assets("test/content/markdown").expect("failed to get assets");
        assert_eq!(assets.len(), 1);

        assets
            .iter()
            .find(|a| a.path == "hello-md.png")
            .expect("expected an asset with hello-md.png file name");
    }

    #[test]
    pub fn test_source_from_directory() {
        init();
        let posts = source_from_directory("test/content").expect("failed to get posts");
        assert_eq!(posts.len(), 3);

        let md_post = posts
            .iter()
            .find(|p| p.base_dir == "test/content/markdown")
            .unwrap();
        assert_eq!(md_post.title, "Writing in Markdown");
        assert_eq!(md_post.format, FormatKind::Markdown);

        let pandoc_md_post = posts
            .iter()
            .find(|p| p.base_dir == "test/content/markdown-pandoc")
            .unwrap();
        assert_eq!(pandoc_md_post.title, "Writing in Pandoc Markdown");
        assert_eq!(pandoc_md_post.format, FormatKind::Pandoc);
    }
}

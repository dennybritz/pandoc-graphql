use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use heck::KebabCase;

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
            post_data.base_dir = base_dir;
            return Ok(post_data);
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_source_from_directory() {
        env_logger::init();

        let posts = source_from_directory("test/content").expect("failed to get posts");
        assert_eq!(posts.len(), 2);

        let md_post = posts
            .iter()
            .find(|p| p.base_dir == "test/content/markdown")
            .unwrap();
        assert_eq!(md_post.title, "A post in markdown!");
        assert_eq!(md_post.format, FormatKind::Markdown);

        let pandoc_md_post = posts
            .iter()
            .find(|p| p.base_dir == "test/content/markdown-pandoc")
            .unwrap();
        assert_eq!(pandoc_md_post.title, "A post in pandoc markdown!");
        assert_eq!(pandoc_md_post.format, FormatKind::Pandoc);
    }
}

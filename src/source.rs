use anyhow::Result;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FormatKind {
    Markdown,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Post {
    pub format: FormatKind,
    pub title: String,
    pub date: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub markdown: Option<MarkdownConfig>,

    #[serde(skip)]
    pub base_dir: String,
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
        assert_eq!(posts.len(), 1);

        let md_post = posts.get(0).unwrap();
        assert_eq!(md_post.base_dir, "test/content/markdown");
        assert_eq!(md_post.title, "A post in markdown!");
        assert_eq!(md_post.format, FormatKind::Markdown);
    }
}

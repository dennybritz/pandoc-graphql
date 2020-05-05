use serde::Deserialize;
use std::fs::File;

/// A post on the file system
pub struct Post {
    /// Path to the base folder of the past
    pub path: String,

    /// The build config, that is, the parsed YAML from build.yaml
    pub build_config: BuildConfig,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FormatKind {
    Markdown,
}

#[derive(Deserialize, Debug)]
pub struct BuildConfig {
    pub format: FormatKind,
    pub title: String,
    pub date: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub draft: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub markdown: Option<MarkdownConfig>,
}

#[derive(Deserialize, Debug)]
pub struct MarkdownConfig {
    pub path: String,
}

pub fn source_from_directory(base: &str) -> Vec<Post> {
    let build_file_glob = format!("{}/**/build.yaml", base);
    log::info!("sourcing files from directory: {}", base);
    let build_file = glob::glob(build_file_glob.as_str()).expect("failed to read glob pattern");
    build_file
        .filter_map(|x| x.ok())
        .filter_map(|path| {
            let build_config = serde_yaml::from_reader(File::open(&path).unwrap());
            match build_config {
                Ok(build_config) => Some(Post {
                    path: format!("{}", path.parent().unwrap().display()),
                    build_config,
                }),
                Err(e) => {
                    log::warn!("{}", e);
                    log::warn!("failed to parse build config, ignoring: {}", path.display());
                    None
                }
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_source_from_directory() {
        env_logger::init();

        let posts = source_from_directory("test/content");
        assert_eq!(posts.len(), 1);

        let md_post = posts.get(0).unwrap();
        assert_eq!(md_post.path, "test/content/markdown");
        assert_eq!(md_post.build_config.title, "A post in markdown!");
        assert_eq!(md_post.build_config.format, FormatKind::Markdown);
    }
}

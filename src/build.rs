use crate::source;
use anyhow::Result;
use pulldown_cmark::{html, Parser};
use std::io::Cursor;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn markdown_to_html(base_dir: &str, config: &source::MarkdownConfig) -> Result<String> {
    let md_file_path = format!("{}/{}", base_dir, config.path);
    log::info!("converting markdown to html: {}", md_file_path);
    let markdown_str = std::fs::read_to_string(md_file_path)?;
    let mut bytes = Vec::new();
    let parser = Parser::new(markdown_str.as_str());
    html::write_html(Cursor::new(&mut bytes), parser)?;
    Ok(String::from_utf8(bytes)?)
}

pub fn run_pandoc(
    base_dir: &str,
    config: &serde_yaml::Value,
    output_format: &str,
) -> Result<Vec<u8>> {
    // Because we need to pass the config to pandoc as a command-line argument,
    // we write it into a temporary file to disk
    let (file, config_path) = NamedTempFile::new()?.keep()?;
    log::info!("writing pandoc config tempfile: {}", config_path.display(),);
    serde_yaml::to_writer(&file, config)?;

    let output = Command::new("pandoc")
        .current_dir(base_dir)
        .arg("-d")
        .arg(format!("{}", &config_path.display()))
        .arg("-t")
        .arg(output_format)
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;
    for line in stderr.lines() {
        log::warn!("[pandoc] {}", line);
    }

    if !output.status.success() {
        return Err(anyhow::anyhow!("pandoc failure: {}", stderr));
    }

    Ok(output.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_run_pandoc() {
        env_logger::init();
        let base_dir = "test/content/markdown-pandoc";
        let config: serde_yaml::Value = serde_yaml::from_str(
            r###"
        input-file: hello-world.md
        "###,
        )
        .unwrap();

        let result = run_pandoc(base_dir, &config, "html").expect("failed to call pandoc");
        let result = String::from_utf8(result).expect("invalid utf-8 data");

        assert_eq!(
            result,
            "<h2 id=\"a-post-in-pandoc-markdown\">A post in pandoc markdown!</h2>\n<p>Hello World!</p>\n"
        );
    }
}

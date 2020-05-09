use crate::source;
use anyhow::Result;
use pulldown_cmark::{html, Parser};
use std::io::Cursor;
use std::io::Write;
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

pub fn convert_from_html(html: &str, format: &str) -> Result<String> {
    let (mut file, path) = NamedTempFile::new()?.keep()?;
    // let parent_dir = format!("{}", path.parent().unwrap().display());
    file.write_all(html.as_bytes())?;
    log::info!("writing temporary html file: {}", path.display());
    let config = format!(
        r###"
        from: html
        input-file: '{}'
    "###,
        path.display()
    );
    let config = serde_yaml::from_str(&config)?;
    let buf = crate::pandoc::run_pandoc(".", &config, &format)?;
    Ok(base64::encode(buf))
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

pub fn run_pandoc_citeproc(base_dir: &str, bibfile_path: &str) -> Result<String> {
    log::info!("running: pandoc-citeproc -y {}", bibfile_path);
    let output = Command::new("pandoc-citeproc")
        .current_dir(base_dir)
        .arg("-y")
        .arg(bibfile_path)
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;
    for line in stderr.lines() {
        log::warn!("[pandoc] {}", line);
    }

    if !output.status.success() {
        return Err(anyhow::anyhow!("pandoc failure: {}", stderr));
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    pub fn test_run_pandoc() {
        init();
        let base_dir = "test/content/markdown-pandoc";
        let config: serde_yaml::Value = serde_yaml::from_str(
            r###"
        input-file: content.md
        "###,
        )
        .unwrap();

        let result = run_pandoc(base_dir, &config, "html").expect("failed to call pandoc");
        let result = String::from_utf8(result).expect("invalid utf-8 data");
        assert!(result.contains("Cupcake ipsum dolor sit amet"));
    }

    #[test]
    pub fn test_run_citeproc() {
        init();
        let result = run_pandoc_citeproc("test/content/citations", "references.bib")
            .expect("failed to call pandoc-citeproc");
        assert!(result.contains("Impartial triangular chocolate bar games"));
    }

    #[test]
    pub fn test_convert_from_html() {
        init();
        let html = "<h1>Hello HTML!</h1>";
        let result = convert_from_html(html, "markdown").unwrap();
        let decoded = base64::decode(result).expect("base64 decoding failed");
        let result = String::from_utf8(decoded).expect("utf-8 decodding failed");
        assert_eq!(result, "Hello HTML!\n===========\n")
    }
}

use crate::source;
use anyhow::Result;
use pulldown_cmark::{html, Parser};
use std::io::Cursor;

pub fn markdown_to_html(base_dir: &str, config: &source::MarkdownConfig) -> Result<String> {
    let md_file_path = format!("{}/{}", base_dir, config.path);
    log::info!("converting markdown to html: {}", md_file_path);
    let markdown_str = std::fs::read_to_string(md_file_path)?;
    let mut bytes = Vec::new();
    let parser = Parser::new(markdown_str.as_str());
    html::write_html(Cursor::new(&mut bytes), parser)?;
    Ok(String::from_utf8(bytes)?)
}

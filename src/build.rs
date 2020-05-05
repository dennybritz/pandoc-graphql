use crate::source;
use source::FormatKind;
use anyhow::{Result, anyhow};
use heck::KebabCase;
use pulldown_cmark::{html, Parser};
use std::io::Cursor;

pub fn build_post(input: &source::Post) -> Result<crate::schema::Post> {
    // TODO: Right now we assume the post is Markdown

    let slug = input
        .build_config
        .slug
        .clone()
        .unwrap_or(input.build_config.title.to_kebab_case());

    let html = match input.build_config.format {
        FormatKind::Markdown => {
            let md_config = input.build_config.markdown.as_ref().ok_or(anyhow!("expected markdown config"))?;
            markdown_to_html(&input.path, md_config)?
        }
    };

    let post = crate::schema::Post {
        id: String::from("TODO"),
        slug: slug.clone(),
        title: input.build_config.title.clone(),
        date: input.build_config.date.clone(),
        html,
    };

    Ok(post)
}

pub fn markdown_to_html(base_dir: &str, config: &source::MarkdownConfig) -> Result<String> {
    let md_file_path = format!("{}/{}", base_dir, config.path);
    log::info!("converting markdown to html: {}", md_file_path);
    let markdown_str = std::fs::read_to_string(md_file_path)?;
    let mut bytes = Vec::new();
    let parser = Parser::new(markdown_str.as_str());
    html::write_html(Cursor::new(&mut bytes), parser)?;
    Ok(String::from_utf8(bytes)?)
}


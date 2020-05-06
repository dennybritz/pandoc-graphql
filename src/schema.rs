use crate::source;
use crate::source::FormatKind;
use anyhow::anyhow;
use juniper::{EmptyMutation, FieldResult};
use std::sync::{Arc, RwLock};

#[juniper::object]
#[graphql(description = "A blog post")]
impl crate::source::Post {
    fn id(&self) -> &str {
        "TODO"
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn date(&self) -> &str {
        &self.date
    }

    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    fn slug(&self) -> String {
        self.slug()
    }

    fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }

    fn draft(&self) -> Option<bool> {
        self.draft
    }

    fn tags(&self) -> Option<&Vec<String>> {
        self.tags.as_ref()
    }

    fn bibtex(&self) -> Option<&String> {
        self.bibtex.as_ref()
    }

    fn authors(&self) -> Option<&Vec<source::Author>> {
        self.authors.as_ref()
    }

    fn html(&self) -> FieldResult<String> {
        match &self.format {
            FormatKind::Pandoc => {
                let config = self.pandoc.as_ref().ok_or(anyhow!("no pandoc config"))?;
                let buf = crate::build::run_pandoc(&self.base_dir, config, "html")?;
                let html = String::from_utf8(buf)?;
                Ok(html)
            }
            FormatKind::Markdown => {
                let md_config = self
                    .markdown
                    .as_ref()
                    .ok_or(anyhow!("no markdown config"))?;
                Ok(crate::build::markdown_to_html(&self.base_dir, md_config)?)
            }
        }
    }

    fn convert(&self, format: String) -> FieldResult<String> {
        match &self.format {
            FormatKind::Pandoc => {
                // TODO: Support conversion from arbitrary html
                let config = self.pandoc.as_ref().ok_or(anyhow!("no pandoc config"))?;
                let buf = crate::build::run_pandoc(&self.base_dir, config, &format)?;
                Ok(base64::encode(buf))
            }
            FormatKind::Markdown => {
                Err(anyhow!("output conversion is currently only supported for pandoc posts").into())
            }
        }
    }
    
}

#[derive(Clone)]
pub struct SharedContext {
    pub context: Arc<RwLock<Context>>,
}

impl SharedContext {
    pub fn new() -> Self {
        SharedContext {
            context: Arc::new(RwLock::new(Context { posts: vec![] })),
        }
    }

    pub fn update(&self, path: &str) {
        let mut ctx = self.context.write().unwrap();
        match crate::source::source_from_directory(path) {
            Ok(posts) => ctx.posts = posts,
            Err(e) => log::warn!("failed to source posts from {}: {}", path, e),
        };
    }
}

pub struct Context {
    pub posts: Vec<crate::source::Post>,
}
impl juniper::Context for SharedContext {}

pub struct Query;

#[juniper::object(Context = SharedContext)]
impl Query {
    fn apiVersion() -> &str {
        "1.0"
    }

    fn posts(context: &SharedContext) -> FieldResult<Vec<source::Post>> {
        let context = context.context.as_ref().read().unwrap();
        // TODO: Can we get rid of this clone !?
        Ok(context.posts.clone())
    }

    fn post(context: &SharedContext, slug: String) -> FieldResult<source::Post> {
        let context = context.context.as_ref().read().unwrap();
        let post = context
            .posts
            .iter()
            .find(|p| p.slug() == slug)
            .ok_or(anyhow::anyhow!("no post for slug: {}", &slug))?
            .clone();
        Ok(post)
    }
}

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<SharedContext>>;

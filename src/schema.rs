use crate::source;
use crate::source::FormatKind;
use anyhow::anyhow;
use heck::KebabCase;
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

    fn slug(&self) -> String {
        self.slug.clone().unwrap_or(self.title.to_kebab_case())
    }

    fn html(&self) -> FieldResult<String> {
        match &self.format {
            FormatKind::Markdown => {
                let md_config = self
                    .markdown
                    .as_ref()
                    .ok_or(anyhow!("no markdown config"))?;
                Ok(crate::build::markdown_to_html(&self.base_dir, md_config)?)
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
}

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<SharedContext>>;

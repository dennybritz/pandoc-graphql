use crate::source;
use juniper::{EmptyMutation, FieldResult};
use std::sync::{Arc, RwLock};
use chrono::prelude::*;

#[juniper::object]
#[graphql(description = "A blog post")]
impl crate::source::Post {
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

    fn assets(&self) -> &Vec<source::Asset> {
        &self.assets
    }

    fn citations(&self) -> FieldResult<Option<Vec<source::Citation>>> {
        self.citations().map_err(|e| e.into())
    }

    fn html(&self) -> FieldResult<String> {
        self.html().map_err(|e| e.into())
    }

    fn pdf(&self) -> FieldResult<String> {
        self.convert("pdf").map_err(|e| e.into())
    }

    fn markdown(&self) -> FieldResult<String> {
        self.convert("markdown").map_err(|e| e.into())
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
        let mut posts = context.posts.clone();
        posts.sort_by(|a, b| {
            let a =  NaiveDate::parse_from_str(&a.date, "%Y-%m-%d").unwrap();
            let b =  NaiveDate::parse_from_str(&b.date, "%Y-%m-%d").unwrap();
            b.cmp(&a)
        });
        Ok(posts)
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

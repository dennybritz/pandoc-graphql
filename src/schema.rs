use juniper::{EmptyMutation, FieldResult};
use std::sync::{Arc, Mutex};

pub struct Post {
    pub id: String,
    pub title: String,
    pub date: String,
    pub slug: String,
    pub html: String,
}

#[juniper::object]
#[graphql(description = "A blog post")]
impl Post {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn date(&self) -> &str {
        &self.date
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    // TODO: HTML should be generated here...
    fn html(&self) -> &str {
        &self.html
    }    
}

#[derive(Clone)]
pub struct SharedContext {
    pub context: Arc<Mutex<Context>>,
}

impl SharedContext {
    pub fn new() -> Self {
        SharedContext {
            context: Arc::new(Mutex::new(Context { posts: vec![] })),
        }
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

    fn posts(context: &SharedContext) -> FieldResult<Vec<Post>> {
        let context = context.context.lock().unwrap();
        // TODO: Handle build error here
        let posts = context
            .posts
            .iter()
            .map(|p| crate::build::build_post(p).unwrap())
            .collect();
        Ok(posts)
    }
}

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<SharedContext>>;

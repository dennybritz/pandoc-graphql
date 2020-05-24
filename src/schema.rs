use juniper::{EmptyMutation, FieldResult};

#[juniper::object]
#[graphql(description = "A document")]
impl super::document::Document {
    fn id(&self) -> &str {
        self.id.as_ref()
    }

    fn collections(&self) -> &Vec<String> {
        self.collections.as_ref()
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn author(&self) -> &Vec<String> {
        self.author.as_ref()
    }

    fn date(&self) -> Option<&str> {
        self.date.as_deref()
    }

    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    fn tags(&self) -> &Vec<String> {
        self.tags.as_ref()
    }

    pub fn convert(&self, args: Option<Vec<String>>) -> FieldResult<String> {
        let args = args.unwrap_or(vec![]);
        let args = args.iter().map(|x| x.as_ref()).collect();
        let data = self.convert(args)?;
        Ok(base64::encode(data))
    }

    pub fn html(&self) -> FieldResult<String> {
        let data = self.convert(vec!["-t", "html"])?;
        let data = String::from_utf8(data)?;
        Ok(data)
    }

    fn citations(&self) -> FieldResult<Option<Vec<crate::citation::Citation>>> {
        self.citations().map_err(|e| e.into())
    }

    fn asset(&self, name: String) -> FieldResult<String> {
        for asset_path in self.asset_paths.iter() {
            let fpath = format!("{}/{}", asset_path, name);
            let fpath = std::path::Path::new(&fpath);
            if fpath.exists() && fpath.is_file() {
                let buf = std::fs::read(fpath)?;
                return Ok(base64::encode(buf));
            }
        }
        Err(anyhow::anyhow!("asset not found: {}", name).into())
    }
}

#[derive(Debug, Clone)]
pub struct Context {
    index_files: Vec<String>,
    base_dir: String,
}
impl juniper::Context for Context {}

impl Context {
    pub fn new(index_files: Vec<String>, base_dir: String) -> Self {
        Context {
            index_files,
            base_dir,
        }
    }

    pub fn load_documents_from_file(
        &self,
        path: &std::path::PathBuf,
    ) -> anyhow::Result<Vec<super::document::Document>> {
        log::info!("reading documents from: {}", path.display());
        let docs = std::fs::read_to_string(path)?;
        let docs: Vec<&str> = docs.split("\n---\n").collect();
        // TODO: Move this into Document implementation
        docs.into_iter()
            .map(|d| {
                let mut doc: super::document::PandocDefaults = serde_yaml::from_str(d)?;
                doc.metadata.pandoc_defaults_str = String::from(d);
                doc.metadata.asset_paths = doc.asset_paths(&self.base_dir);
                doc.metadata.base_dir = String::from(&self.base_dir);
                Ok(doc.metadata)
            })
            .collect()
    }

    pub fn load_documents(&self) -> anyhow::Result<Vec<super::document::Document>> {
        let mut docs: Vec<super::document::Document> = self
            .index_files
            .iter()
            .filter_map(|pattern| match glob::glob(pattern) {
                Ok(paths) => Some(paths.flatten()),
                Err(e) => {
                    log::warn!("invalid glob pattern, ignoring {}: {}", &pattern, e);
                    None
                }
            })
            .flatten()
            .filter_map(|pb| match self.load_documents_from_file(&pb) {
                Ok(docs) => Some(docs),
                Err(e) => {
                    log::warn!("failed to load documents, ignoring {}: {}", pb.display(), e);
                    None
                }
            })
            .flatten()
            .collect();
        docs.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(docs)
    }
}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn apiVersion() -> &str {
        "0.1"
    }

    fn documents(
        context: &Context,
        collection: Option<String>,
    ) -> FieldResult<Vec<super::document::Document>> {
        let docs = context.load_documents()?;
        let docs = match collection {
            Some(name) => docs
                .into_iter()
                .filter(|d| d.collections.contains(&name))
                .collect(),
            None => docs,
        };
        Ok(docs)
    }

    fn document(context: &Context, id: String) -> FieldResult<super::document::Document> {
        let docs = context.load_documents()?;
        let document = docs.into_iter().find(|d| d.id.as_str() == id.as_str());
        document.ok_or(anyhow::anyhow!("document not found: {}", &id).into())
    }
}

pub type Schema = juniper::RootNode<'static, Query, EmptyMutation<Context>>;

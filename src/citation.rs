use serde::Deserialize;

/// A citation in CSL (https://citationstyles.org/) format
/// Also see
/// - https://citeproc-js.readthedocs.io/en/latest/csl-json/markup.html
/// - https://github.com/jgm/pandoc-citeproc/blob/master/man/pandoc-citeproc.1.md
#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
#[serde(rename_all = "kebab-case")]
pub struct Citation {
    pub id: String,
    pub title: Option<String>,
    pub author: Option<Vec<CitationAuthor>>,
    pub container_title: Option<String>,
    pub publisher: Option<String>,
    pub volume: Option<String>,
    pub issue: Option<String>,
    pub issued: Option<Vec<CitationIssued>>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
    #[serde(rename = "DOI")]
    pub doi: Option<String>,
}

#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
pub struct CitationAuthor {
    pub family: Option<String>,
    pub given: Option<String>,
}

#[derive(Deserialize, Debug, Clone, juniper::GraphQLObject)]
#[serde(rename_all = "kebab-case")]
pub struct CitationIssued {
    pub year: Option<i32>,
    pub month: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Citations {
    pub references: Vec<Citation>,
}

use std::collections::HashMap;
use axum::extract::Query;
use axum::Json;
pub use crate::data::{Repo, REPOS};
use crate::data::Language;

pub async fn root() -> Json<Vec<Repo>> {
    Json(REPOS.to_vec())
}

pub async fn query(Query(params): Query<HashMap<String, String>>) -> Json<Vec<Repo>> {
    let mut result: Vec<Repo> = vec![];
    for i in REPOS.to_vec() {
        let query = QueryModel {
            name: match params.get("name") {
                None => String::from("NULL"),
                Some(name) => {name.to_owned()}
            },
            about: match params.get("about") {
                None => String::from("NULL"),
                Some(about) => {about.to_owned()}
            },
            language: match params.get("language") {
                None => Language::NULL,
                Some(language) => {
                    match language.as_str() {
                        "rust" => Language::RUST,
                        "java" => Language::JAVA,
                        "csharp" => Language::CSHARP,
                        "c" => Language::C,
                        "cpp" => Language::CPP,
                        "python" => Language::PYTHON,
                        "css" => Language::CSS,
                        "html" => Language::HTML,
                        "markdown" => Language::MARKDOWN,
                        "ev" => Language::EV,
                        _ => Language::NULL
                    }
                }
            },
        };
        // If the query parameter is null it must mean it was not provided,
        // and that should be treated as a generalization and let it pass through
        if (query.name == "NULL" || i.name == query.name) &&
            (query.about == "NULL" || i.about == query.about) &&
            (query.language == Language::NULL || i.language == query.language) {
            result.push(i.clone());
        }
    }
    Json(result)
}

struct QueryModel {
    name: String,
    about: String,
    language: Language,
}
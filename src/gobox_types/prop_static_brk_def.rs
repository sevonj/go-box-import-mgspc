use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropStaticBrkDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub model_path: String,
    pub coll_path: String,
    pub model_destroy_path: String,
    pub coll_destroy_path: String,
}

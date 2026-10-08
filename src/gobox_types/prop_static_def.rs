use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct PropStaticDef {
    pub id: String,
    pub name: String,
    pub description: String,
    pub model_path: String,
    pub coll_path: String,
}

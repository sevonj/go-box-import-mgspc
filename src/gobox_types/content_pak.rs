use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct ContentPak {
    pub stages: Vec<String>,
    pub props_static: Vec<String>,
    pub props_static_brk: Vec<String>,
}

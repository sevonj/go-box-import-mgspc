use crate::gobox_types::PropSpawnDef;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct StageDef {
    pub id: String,
    pub version: i64,
    pub external: bool,
    pub name: String,
    pub description: String,
    pub thumbnail_path: String,
    pub model_path: String,
    pub coll_path: String,
    pub player_start_position: [f32; 3],
    pub player_start_rotation: [f32; 3],
    pub props_static: Vec<PropSpawnDef>,
}

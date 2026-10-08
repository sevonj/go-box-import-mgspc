use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct PropSpawnDef {
    pub id: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

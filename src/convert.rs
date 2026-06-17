use glam::Vec3;
use gltf::json::extensions::mesh;
use gltf::{Gltf, Scene};

use crate::types::Kmd;
use crate::types::KmdMesh;

mod stage_info;

pub fn kmd_to_gltf(kmd: &Kmd) {}

pub fn kmd_mesh_to_gltf(meshes: &[KmdMesh]) {
    let num_vertices = meshes.iter().map(|m| m.num_vertices()).sum();
    let mut positions: Vec<Vec3> = Vec::with_capacity(num_vertices);

    let mut base_vertex = 0;
    for mesh in meshes {
        for v in mesh.vertices() {
            positions.push(v.into());
        }
    }
}

use std::collections::HashMap;

use glam::Vec2;

use super::Kmd;

impl Kmd {
    pub fn to_wavefront(&self, materials: &HashMap<u16, String>) -> String {
        let mut out = String::new();
        let mut base_v = 1;
        let mut base_vt = 1;
        out.push_str("mtllib extracted_materials.mtl\n");

        for (i, mesh) in self.meshes.iter().enumerate() {
            out.push_str(&format!("o mesh_{i}\n"));

            let num_vertices = mesh.vertices.len();
            for v in &mesh.vertices {
                let x = v.x as f32 / 1024.0;
                let y = v.y as f32 / 1024.0;
                let z = v.z as f32 / 1024.0;
                out.push_str(&format!("v {x} {y} {z}\n"));
            }

            for uv in &mesh.uvs {
                let uv: Vec2 = uv.into();
                out.push_str(&format!("vt {} {}\n", uv.x, uv.y));
            }

            for (i, (face, tex_hash)) in
                mesh.vertex_faces.iter().zip(&mesh.material_ids).enumerate()
            {
                if let Some(tex_name) = materials.get(tex_hash) {
                    out.push_str(&format!("usemtl {tex_name}\n"));
                } else {
                    out.push_str(&format!("usemtl notfound_{tex_hash:04X}\n"));
                }

                let a = face[3] as usize + base_v;
                let b = face[2] as usize + base_v;
                let c = face[1] as usize + base_v;
                let d = face[0] as usize + base_v;
                let vt_a = i * 4 + 3 + base_vt;
                let vt_b = i * 4 + 2 + base_vt;
                let vt_c = i * 4 + 1 + base_vt;
                let vt_d = i * 4 + 0 + base_vt;
                out.push_str(&format!("f {a}/{vt_a} {b}/{vt_b} {c}/{vt_c} {d}/{vt_d}\n"));
            }

            base_v += num_vertices;
            base_vt += mesh.vertex_faces.len() * 4;
        }
        out
    }
}

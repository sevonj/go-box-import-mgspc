use super::Kmd;
use crate::mgs_types::KmdMesh;
use glam::DVec3;
use glam::Vec2;
use glam::Vec3;
use gltf::accessor::DataType;
use gltf::accessor::Dimensions;
use gltf::buffer::Target;
use gltf::json::Accessor;
use gltf::json::Buffer;
use gltf::json::Index;
use gltf::json::Mesh;
use gltf::json::Node;
use gltf::json::Root;
use gltf::json::Scene;
use gltf::json::Value;
use gltf::json::accessor::GenericComponentType;
use gltf::json::buffer::View;
use gltf::json::extras::Void;
use gltf::json::mesh::Primitive;
use gltf::json::mesh::Semantic;
use gltf::json::validation::Checked;
use gltf::mesh::Mode;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Debug)]
struct TempPrim {
    base_vertex: usize,
    num_vertices: usize,
    base_index: usize,
    num_indices: usize,
    material_id: u16,
}

impl Kmd {
    pub fn to_glb(meshes: &[KmdMesh]) -> Vec<u8> {
        {
            let mut positions: Vec<Vec3> = vec![];
            let mut normals: Vec<Vec3> = vec![];
            let mut uvs: Vec<Vec2> = vec![];
            let mut indices: Vec<u32> = vec![];

            let mut temp_meshes: Vec<Vec<TempPrim>> = vec![];

            for mesh in meshes {
                let pos: Vec3 = mesh.header.pos.into();
                let input_positions: Vec<Vec3> = mesh
                    .vertices()
                    .iter()
                    .map(Into::into)
                    .map(|p: Vec3| p + pos)
                    .collect();
                let input_normals: Vec<Vec3> = mesh.normals().iter().map(Into::into).collect();
                let input_uvs: Vec<Vec2> = mesh.uvs().iter().map(Into::into).collect();

                let mut temp_prims: Vec<TempPrim> = vec![];

                // Faces grouped by material
                let mut faces_grouped: HashMap<u16, Vec<usize>> = HashMap::new();
                for (face_idx, &mat_id) in mesh.material_ids.iter().enumerate() {
                    faces_grouped.entry(mat_id).or_default().push(face_idx);
                }

                for (mat_id, f_indices) in &faces_grouped {
                    let mut prim_positions: Vec<Vec3> = vec![];
                    let mut prim_normals: Vec<Vec3> = vec![];
                    let mut prim_uvs: Vec<Vec2> = vec![];
                    let mut prim_indices: Vec<u32> = vec![];

                    for &f_idx in f_indices {
                        let pos_indices = mesh.vertex_faces[f_idx];
                        let nor_indices = mesh.normal_faces[f_idx];
                        for corner in [3, 2, 0, 2, 1, 0] {
                            prim_indices.push(prim_positions.len() as u32);

                            let idx_p = pos_indices[corner] as usize;
                            prim_positions.push(input_positions[idx_p]);

                            let idx_n = nor_indices[corner] as usize;
                            prim_normals.push(input_normals[idx_n]);

                            let base_uv = f_idx * 4;
                            prim_uvs.push(input_uvs[base_uv + corner]);
                        }
                    }

                    temp_prims.push(TempPrim {
                        base_vertex: positions.len(),
                        num_vertices: prim_positions.len(),
                        base_index: indices.len(),
                        num_indices: prim_indices.len(),
                        material_id: *mat_id,
                    });

                    positions.extend(prim_positions);
                    normals.extend(prim_normals);
                    uvs.extend(prim_uvs);
                    indices.extend(prim_indices);
                }

                temp_meshes.push(temp_prims);
            }

            let mut bin: Vec<u8> = vec![];
            let buffer_views: Vec<View> = {
                let off_positions = 0;
                for p in &positions {
                    for i in 0..3 {
                        bin.extend_from_slice(&p[i].to_le_bytes());
                    }
                }
                let len_positions = bin.len() - off_positions;
                let off_normals = bin.len();
                for n in &normals {
                    for i in 0..3 {
                        bin.extend_from_slice(&n[i].to_le_bytes());
                    }
                }
                let len_normals = bin.len() - off_normals;
                let off_uvs = bin.len();
                for uv in &uvs {
                    for i in 0..2 {
                        bin.extend_from_slice(&uv[i].to_le_bytes());
                    }
                }
                let len_uvs = bin.len() - off_uvs;
                let off_indices = bin.len();
                for i in &indices {
                    bin.extend_from_slice(&i.to_le_bytes());
                }
                let len_indices = bin.len() - off_indices;

                let buf_idx = Index::new(0);

                vec![
                    create_buffer_view(
                        buf_idx,
                        off_positions,
                        len_positions,
                        Some(Target::ArrayBuffer),
                    ),
                    create_buffer_view(
                        buf_idx,
                        off_normals,
                        len_normals,
                        Some(Target::ArrayBuffer),
                    ),
                    create_buffer_view(buf_idx, off_uvs, len_uvs, Some(Target::ArrayBuffer)),
                    create_buffer_view(
                        buf_idx,
                        off_indices,
                        len_indices,
                        Some(Target::ElementArrayBuffer),
                    ),
                ]
            };

            let mut accessors: Vec<Accessor> = vec![];
            let mut meshes: Vec<Mesh> = vec![];
            let mut nodes: Vec<Node> = vec![];

            for (mesh_idx, temp_prims) in temp_meshes.iter().enumerate() {
                let mut primitives = vec![];

                for temp_prim in temp_prims {
                    let base_v = temp_prim.base_vertex;
                    let num_v = temp_prim.num_vertices;
                    let base_i = temp_prim.base_index;
                    let num_i = temp_prim.num_indices;

                    let (bb_min, bb_max) = calc_bbox(&positions[base_v..base_v + num_v]);

                    let base_accessor = accessors.len() as u32;
                    accessors.push(create_accessor_vec3(
                        Index::new(0),
                        base_v * size_of::<Vec3>(),
                        num_v,
                        bb_min.into(),
                        bb_max.into(),
                    ));
                    accessors.push(create_accessor_vec3(
                        Index::new(1),
                        base_v * size_of::<Vec3>(),
                        num_v,
                        DVec3::NEG_ONE,
                        DVec3::ONE,
                    ));
                    accessors.push(create_accessor_vec2(
                        Index::new(2),
                        base_v * size_of::<Vec2>(),
                        num_v,
                    ));
                    accessors.push(make_accessor_indices(Index::new(3), base_i * 4, num_i));

                    let mut attributes = BTreeMap::new();
                    attributes.insert(
                        Checked::Valid(Semantic::Positions),
                        Index::new(base_accessor),
                    );
                    attributes.insert(
                        Checked::Valid(Semantic::Normals),
                        Index::new(base_accessor + 1),
                    );
                    attributes.insert(
                        Checked::Valid(Semantic::TexCoords(0)),
                        Index::new(base_accessor + 2),
                    );

                    primitives.push(Primitive {
                        attributes,
                        indices: Some(Index::new(base_accessor + 3)),
                        material: None, // TODO: materials
                        mode: Checked::Valid(Mode::Triangles),
                        extensions: None,
                        extras: Void::default(),
                        targets: None,
                    });
                }

                let mesh_json_idx = meshes.len() as u32;
                meshes.push(Mesh {
                    primitives,
                    name: Some(format!("mesh_{}", mesh_idx)),
                    weights: None,
                    extensions: None,
                    extras: Void::default(),
                });
                nodes.push(Node {
                    mesh: Some(Index::new(mesh_json_idx)),
                    name: Some(format!("node_{}", mesh_idx)),
                    ..Default::default()
                });
            }

            let scene_nodes: Vec<Index<Node>> = (0..nodes.len() as u32).map(Index::new).collect();
            let json_str = Root {
                accessors,
                buffers: vec![Buffer {
                    byte_length: bin.len().into(),
                    uri: None,
                    name: None,
                    extensions: None,
                    extras: Void::default(),
                }],
                buffer_views,
                meshes,
                nodes,
                scenes: vec![Scene {
                    nodes: scene_nodes,
                    name: None,
                    extensions: None,
                    extras: Void::default(),
                }],
                scene: Some(Index::new(0)),
                ..Default::default()
            }
            .to_string()
            .unwrap();
            let json_bytes = json_str.as_bytes();

            let len_json = json_bytes.len();
            let len_bin = bin.len();
            let len_total = 12 + 8 + len_json + 8 + len_bin;
            let mut glb = Vec::with_capacity(len_total);

            glb.extend_from_slice(b"glTF");
            glb.extend_from_slice(&2_u32.to_le_bytes());
            glb.extend_from_slice(&(len_total as u32).to_le_bytes());

            glb.extend_from_slice(&(len_json as u32).to_le_bytes());
            glb.extend_from_slice(b"JSON");
            glb.extend_from_slice(json_bytes);

            glb.extend_from_slice(&(len_bin as u32).to_le_bytes());
            glb.extend_from_slice(b"BIN\0");
            glb.extend_from_slice(&bin);

            glb
        }
    }
}

fn create_buffer_view(
    buffer: Index<Buffer>,
    byte_offset: usize,
    byte_length: usize,
    target: Option<Target>,
) -> View {
    View {
        buffer,
        byte_offset: Some(byte_offset.into()),
        byte_length: byte_length.into(),
        byte_stride: None,
        target: target.map(Checked::Valid),
        name: None,
        extensions: None,
        extras: Void::default(),
    }
}

fn calc_bbox(verts: &[Vec3]) -> (Vec3, Vec3) {
    let mut min = Vec3::MAX;
    let mut max = Vec3::MIN;
    for v in verts {
        for i in 0..3 {
            min[i] = min[i].min(v[i]);
            max[i] = max[i].max(v[i]);
        }
    }
    (min, max)
}

fn create_accessor_vec3(
    buffer_view: Index<View>,
    byte_offset: usize,
    count: usize,
    min: DVec3,
    max: DVec3,
) -> Accessor {
    Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(byte_offset.into()),
        count: count.into(),
        component_type: Checked::Valid(GenericComponentType(DataType::F32)),
        type_: Checked::Valid(Dimensions::Vec3),
        min: Some(Value::from(min.to_array().to_vec())),
        max: Some(Value::from(max.to_array().to_vec())),
        normalized: false,
        sparse: None,
        name: None,
        extensions: None,
        extras: Void::default(),
    }
}

fn create_accessor_vec2(buffer_view: Index<View>, byte_offset: usize, count: usize) -> Accessor {
    Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(byte_offset.into()),
        count: count.into(),
        component_type: Checked::Valid(GenericComponentType(DataType::F32)),
        type_: Checked::Valid(Dimensions::Vec2),
        min: None,
        max: None,
        normalized: false,
        sparse: None,
        name: None,
        extensions: None,
        extras: Void::default(),
    }
}

fn make_accessor_indices(buffer_view: Index<View>, byte_offset: usize, count: usize) -> Accessor {
    Accessor {
        buffer_view: Some(buffer_view),
        byte_offset: Some(byte_offset.into()),
        count: count.into(),
        component_type: Checked::Valid(GenericComponentType(DataType::U32)),
        type_: Checked::Valid(Dimensions::Scalar),
        min: None,
        max: None,
        normalized: false,
        sparse: None,
        name: None,
        extensions: None,
        extras: Void::default(),
    }
}

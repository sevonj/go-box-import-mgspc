use crate::gobox_types::Coll;
use crate::intermediary_mesh::MatTranspMode::Half;
use crate::mgs_types::KmdMesh;
use glam::Vec3;
use glam::{EulerRot, Quat, Vec2};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct IntermediaryMesh {
    pub origin: Vec3,
    pub surfaces: HashMap<u16, IntermediarySurf>,
}

impl From<&KmdMesh> for IntermediaryMesh {
    fn from(mesh: &KmdMesh) -> Self {
        let num_faces = mesh.num_faces();

        assert_eq!(num_faces, mesh.vertex_faces.len());
        assert_eq!(num_faces, mesh.normal_faces.len());
        assert_eq!(num_faces * 4, mesh.uvs.len());
        assert_eq!(num_faces, mesh.material_ids.len());

        let mut surfaces = HashMap::new();

        for i in 0..num_faces {
            let mat_id = mesh.material_ids[i];
            if !surfaces.contains_key(&mat_id) {
                surfaces.insert(mat_id, IntermediarySurf::new(mat_id));
            }
            let surf = surfaces.get_mut(&mat_id).unwrap();

            let vface = &mesh.vertex_faces[i];
            let nface = &mesh.normal_faces[i];
            let uvface = &mesh.uvs[i * 4..];

            for corner in [3, 2, 0, 2, 1, 0] {
                surf.positions
                    .push(mesh.vertices[vface[corner] as usize].into());
                let normal: Vec3 = mesh.normals[nface[corner] as usize].into();
                surf.normals.push(normal.normalize());
                surf.uvs.push(uvface[corner].into());
                surf.indices.push(surf.indices.len() as u32);
            }
        }

        for surf in surfaces.values() {
            assert_eq!(surf.positions.len(), surf.normals.len());
            assert_eq!(surf.indices.len(), surf.positions.len());
        }

        Self {
            origin: mesh.header.pos.into(),
            surfaces,
        }
    }
}

impl IntermediaryMesh {
    pub fn empty() -> Self {
        Self {
            origin: Vec3::ZERO,
            surfaces: HashMap::new(),
        }
    }

    // temporary, until objects are implemented
    pub fn rotate(&mut self, euler: Vec3) {
        let rot = Quat::from_euler(EulerRot::XYZ, euler.x, euler.y, euler.z);

        for surf in self.surfaces.values_mut() {
            for p in &mut surf.positions {
                *p = rot * *p;
            }
            for n in &mut surf.normals {
                // Rotation preserves length, so unit normals stay unit.
                *n = rot * *n;
            }
        }
    }

    pub fn collapse_materials(&mut self) {
        let mut surfaces = self.surfaces.clone();
        let mut supersurf = IntermediarySurf::new(0);
        for surf in surfaces.values() {
            let base_v = supersurf.positions.len() as u32;
            supersurf.positions.extend(&surf.positions);
            supersurf.normals.extend(&surf.normals);
            supersurf
                .indices
                .extend(surf.indices.iter().map(|i| i + base_v));
            supersurf.uvs.extend(&surf.uvs);
        }
        supersurf.dedupe();
        surfaces.clear();
        surfaces.insert(0, supersurf);
    }

    pub fn join(&self, rhs: &Self) -> Self {
        let mut surfaces = self.surfaces.clone();
        for mat_id in rhs.surfaces.keys() {
            if !surfaces.contains_key(&mat_id) {
                surfaces.insert(*mat_id, IntermediarySurf::new(*mat_id));
            }

            let surf = surfaces.get_mut(&mat_id).unwrap();
            let rhsurf = rhs.surfaces.get(&mat_id).unwrap();
            surf.material = rhsurf.material.clone();

            let base_v = surf.positions.len() as u32;
            surf.positions.extend(
                rhsurf
                    .positions
                    .iter()
                    .map(|v| v + rhs.origin - self.origin),
            );
            surf.normals.extend(&rhsurf.normals);
            surf.indices
                .extend(rhsurf.indices.iter().map(|i| i + base_v));
            surf.uvs.extend(&rhsurf.uvs);
        }

        Self {
            origin: self.origin,
            surfaces,
        }
    }

    pub fn from_seal_glb(glb: &[u8], origin: Vec3) -> Self {
        let gltf = gltf::Gltf::from_slice(glb).unwrap();
        let blob = gltf.blob.as_deref();

        let mut positions: Vec<Vec3> = vec![];
        let mut indices: Vec<u32> = vec![];

        for mesh in gltf.document.meshes() {
            for primitive in mesh.primitives() {
                assert_eq!(primitive.mode(), gltf::mesh::Mode::Triangles);

                let reader = primitive.reader(|buffer| match buffer.source() {
                    gltf::buffer::Source::Bin => blob,
                    gltf::buffer::Source::Uri(_) => None,
                });

                let prim_positions: Vec<Vec3> =
                    reader.read_positions().unwrap().map(Vec3::from).collect();

                let base_v = positions.len() as u32;
                indices.extend(
                    reader
                        .read_indices()
                        .unwrap()
                        .into_u32()
                        .map(|i| i + base_v),
                );
                positions.extend(prim_positions);
            }
        }

        let num_vertices = positions.len();
        let mut surfaces = HashMap::new();
        surfaces.insert(
            0,
            IntermediarySurf {
                mat_id: 0,
                material: IntermediaryMatData::Sealant,
                positions,
                normals: vec![Vec3::ZERO; num_vertices],
                uvs: vec![Vec2::ZERO; num_vertices],
                indices,
            },
        );

        Self { origin, surfaces }
    }

    pub fn to_wavefront(&self, materials: Option<&HashMap<u16, String>>) -> String {
        let mut out = String::new();
        let mut base_v: usize = 1;

        if materials.is_some() {
            out.push_str(&format!("mtllib mats.mtl\n"));
        }

        for surf in self.surfaces.values() {
            let num_vertices = surf.positions.len();
            for v in &surf.positions {
                let v = v + self.origin;
                let x = v.x;
                let y = v.y;
                let z = v.z;
                out.push_str(&format!("v {x} {y} {z}\n"));
            }

            for n in &surf.normals {
                let x = n.x;
                let y = n.y;
                let z = n.z;
                out.push_str(&format!("vn {x} {y} {z}\n"));
            }

            for uv in &surf.uvs {
                out.push_str(&format!("vt {} {}\n", uv.x, uv.y));
            }

            let mat_name = materials
                .and_then(|m| m.get(&surf.mat_id))
                .cloned()
                .unwrap_or(surf.mat_id.to_string());
            out.push_str(&format!("usemtl {mat_name}\n"));

            for i in 0..surf.indices.len() / 3 {
                let a = base_v + surf.indices[i * 3] as usize;
                let b = base_v + surf.indices[i * 3 + 1] as usize;
                let c = base_v + surf.indices[i * 3 + 2] as usize;
                out.push_str(&format!("f {a}/{a}/{a} {b}/{b}/{b} {c}/{c}/{c}\n"));
            }

            base_v += num_vertices;
        }

        out
    }

    pub fn to_coll(&self) -> Coll {
        let mut coll = Coll::default();
        for surf in self.surfaces.values() {
            for i in 0..surf.indices.len() / 3 {
                let a = surf.indices[i * 3] as usize;
                let b = surf.indices[i * 3 + 2] as usize;
                let c = surf.indices[i * 3 + 1] as usize;
                coll.vbuf.extend(surf.positions[a].to_array());
                coll.vbuf.extend(surf.positions[b].to_array());
                coll.vbuf.extend(surf.positions[c].to_array());
                coll.num_tris += 1;
            }
        }
        coll
    }
}

#[derive(Debug, Clone)]
pub enum IntermediaryMatData {
    None,
    Texture(IntermediaryTexInfo),
    Sealant,
}

#[derive(Debug, Clone)]
pub struct IntermediarySurf {
    pub mat_id: u16,
    pub material: IntermediaryMatData,
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u32>,
}

impl IntermediarySurf {
    pub fn new(mat_id: u16) -> Self {
        Self {
            mat_id,
            material: IntermediaryMatData::None,
            positions: vec![],
            normals: vec![],
            uvs: vec![],
            indices: vec![],
        }
    }

    pub fn dedupe(&mut self) {
        let num_vertices = self.positions.len();

        assert_eq!(num_vertices, self.normals.len());
        assert_eq!(num_vertices, self.uvs.len());

        let mut seen: HashMap<[u32; 8], u32> = HashMap::with_capacity(num_vertices);
        let mut remap: Vec<u32> = Vec::with_capacity(self.indices.len());

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();

        for i in 0..num_vertices {
            let (p, n, uv) = (self.positions[i], self.normals[i], self.uvs[i]);
            let key = [
                (p.x + 0.0).to_bits(),
                (p.y + 0.0).to_bits(),
                (p.z + 0.0).to_bits(),
                (n.x + 0.0).to_bits(),
                (n.y + 0.0).to_bits(),
                (n.z + 0.0).to_bits(),
                (uv.x + 0.0).to_bits(),
                (uv.y + 0.0).to_bits(),
            ];

            let new_idx = *seen.entry(key).or_insert_with(|| {
                positions.push(p);
                normals.push(n);
                uvs.push(uv);
                (positions.len() - 1) as u32
            });
            remap.push(new_idx);
        }

        for idx in &mut self.indices {
            *idx = remap[*idx as usize];
        }

        self.positions = positions;
        self.normals = normals;
        self.uvs = uvs;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatTranspMode {
    Opaque,
    Mask,
    Half,
    Additive,
}

#[derive(Debug, Clone)]
pub struct IntermediaryTexInfo {
    pub name: String,
    pub path: PathBuf,
    pub transparency: MatTranspMode,
    pub double_sided: bool,
}

mod to_gltf {
    use crate::intermediary_mesh::IntermediaryMesh;
    use crate::intermediary_mesh::{IntermediaryMatData, MatTranspMode};
    use glam::DVec3;
    use glam::Vec2;
    use glam::Vec3;
    use gltf::Semantic;
    use gltf::accessor::DataType;
    use gltf::accessor::Dimensions;
    use gltf::buffer::Target;
    use gltf::json::Accessor;
    use gltf::json::Buffer;
    use gltf::json::Image;
    use gltf::json::Index;
    use gltf::json::Material;
    use gltf::json::Mesh;
    use gltf::json::Node;
    use gltf::json::Root;
    use gltf::json::Scene;
    use gltf::json::Texture;
    use gltf::json::Value;
    use gltf::json::accessor::GenericComponentType;
    use gltf::json::buffer::View;
    use gltf::json::extras::Void;
    use gltf::json::material::PbrBaseColorFactor;
    use gltf::json::material::PbrMetallicRoughness;
    use gltf::json::material::StrengthFactor;
    use gltf::json::mesh::Primitive;
    use gltf::json::texture::{Info, Sampler};
    use gltf::json::validation::Checked;
    use gltf::mesh::Mode;
    use gltf::texture::MagFilter;
    use gltf::texture::MinFilter;
    use gltf_json::extensions::material::Unlit;
    use gltf_json::material::AlphaCutoff;
    use gltf_json::material::AlphaMode;
    use std::collections::BTreeMap;

    #[derive(Debug)]
    struct TempPrim {
        base_vertex: usize,
        num_vertices: usize,
        base_index: usize,
        num_indices: usize,
        material_id: u16,
        material: IntermediaryMatData,
    }

    impl IntermediaryMesh {
        pub fn to_glb(&self) -> Vec<u8> {
            {
                let mut positions: Vec<Vec3> = vec![];
                let mut normals: Vec<Vec3> = vec![];
                let mut uvs: Vec<Vec2> = vec![];
                let mut indices: Vec<u32> = vec![];

                let mut temp_meshes: Vec<Vec<TempPrim>> = vec![];

                for surf in self.surfaces.values() {
                    let mut temp_prims: Vec<TempPrim> = vec![];

                    temp_prims.push(TempPrim {
                        base_vertex: positions.len(),
                        num_vertices: surf.positions.len(),
                        base_index: indices.len(),
                        num_indices: surf.indices.len(),
                        material_id: surf.mat_id,
                        material: surf.material.clone(),
                    });

                    positions.extend(&surf.positions);
                    normals.extend(&surf.normals);
                    uvs.extend(&surf.uvs);
                    indices.extend(&surf.indices);

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
                let mut materials: Vec<Material> = Vec::new();
                let mut images: Vec<Image> = Vec::new();
                let mut textures: Vec<Texture> = Vec::new();
                let samplers = vec![Sampler {
                    mag_filter: Some(Checked::Valid(MagFilter::Nearest)),
                    min_filter: Some(Checked::Valid(MinFilter::Nearest)),
                    ..Default::default()
                }];
                let mut nodes: Vec<Node> = vec![];

                let mat_sealant_idx = materials.len() as u32;
                materials.push(Material {
                    name: Some("room_sealant".to_string()),
                    pbr_metallic_roughness: PbrMetallicRoughness {
                        base_color_factor: PbrBaseColorFactor([0.0, 0.0, 0.0, 1.0]),
                        metallic_factor: StrengthFactor(0.0),
                        roughness_factor: StrengthFactor(0.5),
                        ..Default::default()
                    },
                    double_sided: false,
                    extensions: Some(gltf_json::extensions::material::Material {
                        unlit: Some(Unlit {}),
                        ..Default::default()
                    }),
                    ..Default::default()
                });

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

                        let mat_idx = match &temp_prim.material {
                            IntermediaryMatData::None => {
                                materials.push(Material {
                                    name: Some(format!(
                                        "MISSING_PLACEHOLDER_{}",
                                        temp_prim.material_id
                                    )),
                                    pbr_metallic_roughness: PbrMetallicRoughness {
                                        base_color_factor: PbrBaseColorFactor([1.0, 0.0, 1.0, 1.0]),
                                        metallic_factor: StrengthFactor(0.0),
                                        roughness_factor: StrengthFactor(0.5),
                                        ..Default::default()
                                    },
                                    double_sided: true,
                                    extensions: Some(gltf_json::extensions::material::Material {
                                        unlit: Some(Unlit {}),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                });
                                materials.len() as u32 - 1
                            }
                            IntermediaryMatData::Texture(tex_info) => {
                                let name = tex_info.name.clone();
                                images.push(Image {
                                    buffer_view: None,
                                    mime_type: None,
                                    name: Some(name.clone()),
                                    uri: Some(format!("textures/{name}.png")),
                                    extensions: None,
                                    extras: Void::default(),
                                });

                                textures.push(Texture {
                                    name: Some(name.clone()),
                                    sampler: Some(Index::new(0)),
                                    source: Index::new(images.len() as u32 - 1),
                                    extensions: None,
                                    extras: Void::default(),
                                });

                                // TODO: Figure out additive
                                let a = match tex_info.transparency {
                                    MatTranspMode::Opaque | MatTranspMode::Mask => 1.0,
                                    MatTranspMode::Half | MatTranspMode::Additive => 0.5,
                                };
                                materials.push(Material {
                                    name: Some(name),
                                    alpha_mode: match tex_info.transparency {
                                        MatTranspMode::Opaque => Checked::Valid(AlphaMode::Opaque),
                                        MatTranspMode::Mask => Checked::Valid(AlphaMode::Mask),
                                        MatTranspMode::Half => Checked::Valid(AlphaMode::Blend),
                                        MatTranspMode::Additive => Checked::Valid(AlphaMode::Blend),
                                    },
                                    alpha_cutoff: Some(AlphaCutoff(0.5)),
                                    pbr_metallic_roughness: PbrMetallicRoughness {
                                        base_color_factor: PbrBaseColorFactor([1.0, 1.0, 1.0, a]),
                                        base_color_texture: Some(Info {
                                            index: Index::new(textures.len() as u32 - 1),
                                            tex_coord: 0,
                                            extensions: None,
                                            extras: Void::default(),
                                        }),
                                        ..Default::default()
                                    },
                                    double_sided: tex_info.double_sided,
                                    extensions: Some(gltf_json::extensions::material::Material {
                                        unlit: Some(Unlit {}),
                                        ..Default::default()
                                    }),
                                    ..Default::default()
                                });
                                materials.len() as u32 - 1
                            }
                            _ => mat_sealant_idx,
                        };

                        primitives.push(Primitive {
                            attributes,
                            indices: Some(Index::new(base_accessor + 3)),
                            material: Some(Index::new(mat_idx)),
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
                        translation: Some(self.origin.to_array()),
                        ..Default::default()
                    });
                }

                let scene_nodes: Vec<Index<Node>> =
                    (0..nodes.len() as u32).map(Index::new).collect();
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
                    materials,
                    images,
                    textures,
                    samplers,
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

    pub fn create_buffer_view(
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

    pub fn calc_bbox(verts: &[Vec3]) -> (Vec3, Vec3) {
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

    pub fn create_accessor_vec3(
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

    pub fn create_accessor_vec2(
        buffer_view: Index<View>,
        byte_offset: usize,
        count: usize,
    ) -> Accessor {
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

    pub fn make_accessor_indices(
        buffer_view: Index<View>,
        byte_offset: usize,
        count: usize,
    ) -> Accessor {
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
}

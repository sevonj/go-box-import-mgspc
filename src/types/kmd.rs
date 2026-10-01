mod to_gltf;
mod to_wavefront;

use glam::Vec2;

use crate::error::MgsError;
use crate::types::Vector;
use crate::types::vector::ShortVector;
use crate::util::*;

/// Deserialized kmd file
#[derive(Debug, Clone)]
pub struct Kmd {
    header: KmdHeader,
    meshes: Vec<KmdMesh>,
}

impl Kmd {
    pub fn read(buf: &[u8], data_offset: &mut usize) -> Result<Self, MgsError> {
        let header = KmdHeader::from_le_unsized(buf)?;
        *data_offset += size_of::<KmdHeader>();

        let num_meshes = header.num_meshes as usize;

        let mut mesh_headers = Vec::with_capacity(num_meshes);
        for _ in 0..num_meshes {
            mesh_headers.push(KmdMeshHeader::from_le_unsized(&buf[*data_offset..])?);
            *data_offset += size_of::<KmdMeshHeader>();
        }

        let mut meshes = Vec::with_capacity(num_meshes);
        for mesh_header in mesh_headers {
            meshes.push(KmdMesh::read(mesh_header, buf)?);
        }

        Ok(Self { header, meshes })
    }

    pub fn header(&self) -> &KmdHeader {
        &self.header
    }

    pub fn meshes(&self) -> &[KmdMesh] {
        &self.meshes
    }
}

/// 1:1 from disk
/// Kmd file header
#[derive(Debug, Clone)]
pub struct KmdHeader {
    pub num_visible: u32,
    pub num_meshes: u32,
    pub bbox_min: Vector,
    pub bbox_max: Vector,
}

impl KmdHeader {
    pub fn from_le_unsized(buf: &[u8]) -> Result<Self, MgsError> {
        check_fits_buf::<Self>(buf)?;
        Self::from_le_bytes(buf[..size_of::<Self>()].try_into().unwrap())
    }

    pub fn from_le_bytes(buf: &[u8; size_of::<Self>()]) -> Result<Self, MgsError> {
        Ok(Self {
            num_visible: read_u32_le(buf, 0x00)?,
            num_meshes: read_u32_le(buf, 0x04)?,
            bbox_min: Vector::from_le_unsized(&buf[0x08..])?,
            bbox_max: Vector::from_le_unsized(&buf[0x14..])?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KmdUv([u8; 2]);

impl Into<Vec2> for KmdUv {
    fn into(self) -> Vec2 {
        Vec2 {
            x: self.0[0] as f32 / 256.0,
            y: 1.0 - self.0[1] as f32 / 256.0,
        }
    }
}

impl Into<Vec2> for &KmdUv {
    fn into(self) -> Vec2 {
        (*self).into()
    }
}

/// Deserialized kmd mesh entry
#[derive(Debug, Clone)]
pub struct KmdMesh {
    pub header: KmdMeshHeader,
    pub vertices: Vec<ShortVector>,
    /// Indices for each face (every face is a quad)
    pub vertex_faces: Vec<[u8; 4]>,
    pub normals: Vec<ShortVector>,
    /// Normal indices for each face
    pub normal_faces: Vec<[u8; 4]>,
    /// 4x per face
    pub uvs: Vec<KmdUv>,
    /// per face
    pub material_ids: Vec<u16>,
}

impl KmdMesh {
    /// Buf must start at kmd file start
    pub fn read(header: KmdMeshHeader, buf: &[u8]) -> Result<Self, MgsError> {
        let vertices = {
            let num = header.num_vertices as usize;
            let start = header.ptr_vertices as usize;
            let end = start + num * (size_of::<ShortVector>() + 2);
            if buf.len() < end {
                return Err(MgsError::BufferTooSmall {
                    for_what: "read_vec vertices",
                    need: end,
                    avail: buf.len(),
                });
            }
            let mut vertices = Vec::with_capacity(num);
            for i in 0..num {
                let off = start + i * (size_of::<ShortVector>() + 2);
                vertices.push(ShortVector::from_le_unsized(&buf[off..]).unwrap());
            }
            vertices
        };

        let vertex_faces = {
            let num = header.num_faces as usize;
            let start = header.ptr_indices as usize;
            let end = start + num * 4;
            if buf.len() < end {
                return Err(MgsError::BufferTooSmall {
                    for_what: "read_vec indices",
                    need: end,
                    avail: buf.len(),
                });
            }
            let mut faces = Vec::with_capacity(num);
            for i in 0..num {
                let off = start + i * 4;
                faces.push(read_array(buf, off));
            }
            faces
        };

        let normals = {
            let num = header.num_normals as usize;
            let start = header.ptr_normals as usize;
            let end = start + num * (size_of::<ShortVector>() + 2);
            if buf.len() < end {
                return Err(MgsError::BufferTooSmall {
                    for_what: "read_vec normals",
                    need: end,
                    avail: buf.len(),
                });
            }
            let mut normals = Vec::with_capacity(num);
            for i in 0..num {
                let off = start + i * (size_of::<ShortVector>() + 2);
                normals.push(ShortVector::from_le_unsized(&buf[off..]).unwrap());
            }
            normals
        };

        let normal_faces = {
            let num = header.num_faces as usize;
            let start = header.ptr_normal_indices as usize;
            let end = start + num * 4;
            if buf.len() < end {
                return Err(MgsError::BufferTooSmall {
                    for_what: "read_vec normal_faces",
                    need: end,
                    avail: buf.len(),
                });
            }
            let mut faces = Vec::with_capacity(num);
            for i in 0..num {
                let off = start + i * 4;
                let mut f = read_array(buf, off);
                for i in &mut f {
                    *i &= 0x7f;
                }
                faces.push(f);
            }
            faces
        };

        let uvs = {
            let num = header.num_faces as usize * 4;
            let start = header.ptr_uvs as usize;
            let end = start + num * 2;
            if buf.len() < end {
                return Err(MgsError::BufferTooSmall {
                    for_what: "read_vec uvs",
                    need: end,
                    avail: buf.len(),
                });
            }
            let mut uvs = Vec::with_capacity(num);
            for i in 0..num {
                let off = start + i * 2;
                uvs.push(KmdUv(read_array(buf, off)));
            }
            uvs
        };

        let material_ids = {
            let num = header.num_faces as usize;
            let start = header.ptr_materials as usize;
            let end = start + num * 2;
            if buf.len() < end {
                return Err(MgsError::BufferTooSmall {
                    for_what: "read_vec materials",
                    need: end,
                    avail: buf.len(),
                });
            }
            let mut materials = Vec::with_capacity(num);
            for i in 0..num {
                let off = start + i * 2;
                materials.push(read_u16_le(buf, off)?);
            }
            materials
        };

        assert_eq!(vertex_faces.len(), normal_faces.len());
        assert_eq!(vertex_faces.len(), uvs.len() / 4);
        assert_eq!(vertex_faces.len(), material_ids.len());
        for f in &vertex_faces {
            for i in f {
                assert!((*i as usize) < vertices.len());
            }
        }
        for f in &normal_faces {
            for i in f {
                assert!((*i as usize) < normals.len());
            }
        }

        Ok(Self {
            header,
            vertices,
            vertex_faces,
            normals,
            normal_faces,
            uvs,
            material_ids,
        })
    }

    pub fn header(&self) -> &KmdMeshHeader {
        &self.header
    }

    pub fn vertices(&self) -> &[ShortVector] {
        &self.vertices
    }

    pub fn vertex_faces(&self) -> &[[u8; 4]] {
        &self.vertex_faces
    }

    pub fn normals(&self) -> &[ShortVector] {
        &self.normals
    }

    pub fn normal_faces(&self) -> &[[u8; 4]] {
        &self.normal_faces
    }

    pub fn uvs(&self) -> &[KmdUv] {
        &self.uvs
    }

    pub fn material_ids(&self) -> &[u16] {
        &self.material_ids
    }

    pub fn num_faces(&self) -> usize {
        self.vertex_faces.len()
    }

    pub fn num_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn num_normals(&self) -> usize {
        self.normals.len()
    }

    pub fn merge(&self, others: &[Self]) -> Self {
        let mut base_vertex = self.num_vertices();
        let mut base_face = self.num_faces();
        // let mut base_normal = self.num_normals();

        let num_faces = base_face + others.iter().map(Self::num_faces).sum::<usize>();
        let num_vertices = base_vertex + others.iter().map(Self::num_vertices).sum::<usize>();
        // let num_normals = base_normal + others.iter().map(Self::num_normals).sum::<usize>();

        let mut vertices = Vec::with_capacity(num_vertices);
        // let mut faces = Vec::with_capacity(num_faces);
        // let mut normals = Vec::with_capacity(num_normals);

        vertices.extend_from_slice(&self.vertices);
        //        faces.extend_from_slice(&self.faces);
        // normals.extend_from_slice(&self.normals);

        for other in others {
            vertices.extend_from_slice(other.vertices());
            /*for face in other.faces(){
                let mut face = face.clone();
                for v in face{
                    v += base_vertex;
                }
                base_vertex += other.num_vertices();
            }*/
        }

        todo!()
    }
}

/// 1:1 from disk
/// Kmd mesh entry
/// ptrs are relative to kmd file header
#[derive(Debug, Clone)]
pub struct KmdMeshHeader {
    pub flags: i32,
    pub num_faces: u32,
    pub bbox_min: Vector,
    pub bbox_max: Vector,
    pub pos: Vector,
    pub parent: i32,
    pub extend: i32,
    pub num_vertices: u32,
    pub ptr_vertices: i32,
    pub ptr_indices: i32,
    pub num_normals: u32,
    pub ptr_normals: i32,
    pub ptr_normal_indices: i32,
    pub ptr_uvs: i32,
    pub ptr_materials: i32,
    pub pad: i32,
}

impl KmdMeshHeader {
    pub fn from_le_unsized(buf: &[u8]) -> Result<Self, MgsError> {
        check_fits_buf::<Self>(buf)?;
        Self::from_le_bytes(buf[..size_of::<Self>()].try_into().unwrap())
    }

    pub fn from_le_bytes(buf: &[u8; size_of::<Self>()]) -> Result<Self, MgsError> {
        Ok(Self {
            flags: read_i32_le(buf, 0x00)?,
            num_faces: read_u32_le(buf, 0x04)?,
            bbox_min: Vector::from_le_unsized(&buf[0x08..])?,
            bbox_max: Vector::from_le_unsized(&buf[0x14..])?,
            pos: Vector::from_le_unsized(&buf[0x20..])?,
            parent: read_i32_le(buf, 0x2c)?,
            extend: read_i32_le(buf, 0x30)?,
            num_vertices: read_u32_le(buf, 0x34)?,
            ptr_vertices: read_i32_le(buf, 0x38)?,
            ptr_indices: read_i32_le(buf, 0x3c)?,
            num_normals: read_u32_le(buf, 0x40)?,
            ptr_normals: read_i32_le(buf, 0x44)?,
            ptr_normal_indices: read_i32_le(buf, 0x48)?,
            ptr_uvs: read_i32_le(buf, 0x4c)?,
            ptr_materials: read_i32_le(buf, 0x50)?,
            pad: read_i32_le(buf, 0x54)?,
        })
    }
}

mod s08a;
mod s08b;
mod s08c;

use crate::gobox_types::PropSpawnDef;
use glam::Vec3;
use s08a::*;
use s08b::*;
use s08c::*;

pub const NUKE_BLDG_OFFSET: Vec3 = Vec3 {
    x: -6.0,
    y: -5.0,
    z: -184.0,
};

pub const ROOMS: &'static [MgsRoomInfo] = &[
    // Cargo dock
    MgsRoomInfo {
        name: "s00a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("00a.kmd", None, None),
            RoomStaticInfo::new("00a_o1.kmd", None, None),
            RoomStaticInfo::new("00a_o2.kmd", None, None),
            RoomStaticInfo::new("00a_o3.kmd", None, None),
            RoomStaticInfo::new("00a_o4.kmd", None, None),
            RoomStaticInfo::new("00a_r2.kmd", None, None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_00a.glb")),
        origin: Vec3 {
            x: -11.0,
            y: -25.0,
            z: -1.0,
        },
        prop_spawns: &[],
    },
    // helipad
    MgsRoomInfo {
        name: "s01a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("01a.kmd", None, None),
            RoomStaticInfo::new("01a_o1.kmd", None, None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -50.0,
        },
        prop_spawns: &[],
    },
    // tank hangar
    MgsRoomInfo {
        name: "s02a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("02a.kmd", None, None),
            RoomStaticInfo::new("02a_d1.kmd", None, None),
            RoomStaticInfo::new("02a_d2.kmd", None, None),
            RoomStaticInfo::new("02a_d3.kmd", None, None),
            RoomStaticInfo::new("02a_d6.kmd", None, None),
            RoomStaticInfo::new("02a_d7.kmd", None, None),
            RoomStaticInfo::new("02a_o1.kmd", None, None),
            RoomStaticInfo::new("02a_o2.kmd", None, None),
            RoomStaticInfo::new("02a_o3.kmd", None, None),
            RoomStaticInfo::new("02a_o4.kmd", None, None),
            RoomStaticInfo::new("02a_o5.kmd", None, None),
            RoomStaticInfo::new("02a_r1.kmd", None, None),
            RoomStaticInfo::new("02a_r3.kmd", None, None),
            RoomStaticInfo::new("02a_r4.kmd", None, None),
            RoomStaticInfo::new("02a_r5.kmd", None, None),
            RoomStaticInfo::new("02a_r6.kmd", None, None),
            RoomStaticInfo::new("02a_r7.kmd", None, None),
            RoomStaticInfo::new("02a_r9.kmd", None, None),
            RoomStaticInfo::new("02a_r10.kmd", None, None),
            RoomStaticInfo::new("02a_r11.kmd", None, None),
            RoomStaticInfo::new("02a_r12.kmd", None, None),
            RoomStaticInfo::new("02a_r13.kmd", None, None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_02a.glb")),
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -84.0,
        },
        prop_spawns: &[],
    },
    // brig
    MgsRoomInfo {
        name: "s03a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("03a.kmd", None, None),
            RoomStaticInfo::new("03a_d1.kmd", None, None),
            RoomStaticInfo::new("03a_d2.kmd", None, None),
            RoomStaticInfo::new("03a_d3.kmd", None, None),
            RoomStaticInfo::new("03a_d4.kmd", None, None),
            RoomStaticInfo::new("03a_o1a.kmd", None, None),
            RoomStaticInfo::new("03a_o1b.kmd", None, None),
            RoomStaticInfo::new("03a_o2.kmd", None, None),
            RoomStaticInfo::new("03a_r1.kmd", None, None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -4.0,
            y: -10.0,
            z: -86.0,
        },
        prop_spawns: &[],
    },
    // giant electric machine torture room that exists there for some reason ???
    MgsRoomInfo {
        name: "s03b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("03b.kmd", None, None),
            RoomStaticInfo::new("03b_d1.kmd", None, None),
            RoomStaticInfo::new("03b_d2.kmd", None, None),
            RoomStaticInfo::new("03b_d3.kmd", None, None),
            RoomStaticInfo::new("03b_d4.kmd", None, None),
            RoomStaticInfo::new("03b_o1.kmd", None, None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -9.8,
            y: -10.0,
            z: -86.0,
        },
        prop_spawns: &[],
    },
    // storage closets and trapdoors
    MgsRoomInfo {
        name: "s04a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("04a.kmd", None, None),
            RoomStaticInfo::new("04a_d1.kmd", None, None),
            RoomStaticInfo::new("04a_d2.kmd", None, None),
            RoomStaticInfo::new("04a_d3.kmd", None, None),
            RoomStaticInfo::new("04a_d4.kmd", None, None),
            RoomStaticInfo::new("04a_d6.kmd", None, None),
            RoomStaticInfo::new("04a_d7.kmd", None, None),
            RoomStaticInfo::new("04a_d8.kmd", None, None),
            RoomStaticInfo::new("04a_o1a.kmd", None, None),
            RoomStaticInfo::new("04a_o2a.kmd", None, None),
            RoomStaticInfo::new("04a_o3a.kmd", None, None),
            RoomStaticInfo::new("04a_o4a.kmd", None, None),
            RoomStaticInfo::new("04a_o5.kmd", None, None),
            RoomStaticInfo::new("04a_o6.kmd", None, None),
            RoomStaticInfo::new("04a_o7.kmd", None, None),
            RoomStaticInfo::new("04a_r1.kmd", None, None),
            RoomStaticInfo::new("04a_r2.kmd", None, None),
            RoomStaticInfo::new("04a_r3.kmd", None, None),
            RoomStaticInfo::new("04a_r4.kmd", None, None),
            RoomStaticInfo::new("04a_r5.kmd", None, None),
            RoomStaticInfo::new("04a_r6.kmd", None, None),
            RoomStaticInfo::new("04a_r7.kmd", None, None),
            RoomStaticInfo::new("04a_r8.kmd", None, None),
            RoomStaticInfo::new("04a_r9.kmd", None, None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -2.5,
            y: -20.0,
            z: -86.5,
        },
        prop_spawns: &[],
    },
    // AT prez / ocelot boss
    MgsRoomInfo {
        name: "s04b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("04b.kmd", None, None),
            RoomStaticInfo::new("04b_d1.kmd", None, None),
            RoomStaticInfo::new("04b_d2.kmd", None, None),
            RoomStaticInfo::new("04b_d3.kmd", None, None),
            RoomStaticInfo::new("04b_o1a.kmd", None, None),
            RoomStaticInfo::new("04b_o2a.kmd", None, None),
            RoomStaticInfo::new("04b_o3a.kmd", None, None),
            RoomStaticInfo::new("04b_r1.kmd", None, None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -4.0,
            y: -20.0,
            z: -70.0,
        },
        prop_spawns: &[],
    },
    // raven tank battle
    MgsRoomInfo {
        name: "s05a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("05a.kmd", None, None),
            RoomStaticInfo::new("05a_d1a.kmd", None, None),
            RoomStaticInfo::new("05a_o1.kmd", None, None),
            RoomStaticInfo::new("05a_o2.kmd", None, None),
            RoomStaticInfo::new("05a_o3.kmd", None, None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: 2.0,
            y: -0.0,
            z: -130.0,
        },
        prop_spawns: &[],
    },
    // nuke storage
    MgsRoomInfo {
        name: "s06a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("06a.kmd", None, None),
            RoomStaticInfo::new("06a_d1.kmd", None, None),
            RoomStaticInfo::new("06a_d2.kmd", None, None),
            RoomStaticInfo::new("06a_o1.kmd", None, None),
            RoomStaticInfo::new("06a_r1.kmd", None, None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_06a.glb")),
        origin: NUKE_BLDG_OFFSET,
        prop_spawns: &[],
    },
    // toilet floor
    MgsRoomInfo {
        name: "s07a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("07a.kmd", None, None),
            RoomStaticInfo::new("07a_d1.kmd", None, None),
            RoomStaticInfo::new("07a_d2.kmd", None, None),
            RoomStaticInfo::new("07a_d3.kmd", None, None),
            RoomStaticInfo::new("07a_d4.kmd", None, None),
            RoomStaticInfo::new("07a_d5.kmd", None, None),
            RoomStaticInfo::new("07a_d6.kmd", None, None),
            RoomStaticInfo::new("07a_d7.kmd", None, None),
            RoomStaticInfo::new("07a_d8.kmd", None, None),
            RoomStaticInfo::new("07a_d9.kmd", None, None),
            RoomStaticInfo::new("07a_d10.kmd", None, None),
            RoomStaticInfo::new("07a_d11.kmd", None, None),
            RoomStaticInfo::new("07a_d12.kmd", None, None),
            RoomStaticInfo::new("07a_d13.kmd", None, None),
            RoomStaticInfo::new("07a_d14.kmd", None, None),
            RoomStaticInfo::new("07a_d15.kmd", None, None),
            RoomStaticInfo::new("07a_d16.kmd", None, None),
            RoomStaticInfo::new("07a_d17.kmd", None, None),
            RoomStaticInfo::new("07a_d18.kmd", None, None),
            RoomStaticInfo::new("07a_d19.kmd", None, None),
            RoomStaticInfo::new("07a_d20.kmd", None, None),
            RoomStaticInfo::new("07a_o1.kmd", None, None),
            RoomStaticInfo::new("07a_r1.kmd", None, None),
            RoomStaticInfo::new("07a_r2.kmd", None, None),
            RoomStaticInfo::new("07a_r3.kmd", None, None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_07a.glb")),
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x - 3.41797,
            y: NUKE_BLDG_OFFSET.y - 10.0,
            z: NUKE_BLDG_OFFSET.z - 9.765625,
        },
        prop_spawns: &[],
    },
    // admin office
    MgsRoomInfo {
        name: "s07b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar", "stg_mdl2.dar"],
        static_geom: &[
            RoomStaticInfo::new("07b.kmd", None, None),
            RoomStaticInfo::new("07b_d1a.kmd", None, None),
            // KmdImportInfo::new("07b_d2.kmd",   None),
            // KmdImportInfo::new("07b_d3.kmd",   None),
            // KmdImportInfo::new("07b_d4.kmd",   None),
            RoomStaticInfo::new("07b_o1.kmd", None, None),
            RoomStaticInfo::new("07b_o2.kmd", None, None),
            RoomStaticInfo::new("07b_o3.kmd", None, None),
            RoomStaticInfo::new("07b_o4.kmd", None, None),
            RoomStaticInfo::new("07b_o5.kmd", None, None),
            RoomStaticInfo::new("07b_o6.kmd", None, None),
            RoomStaticInfo::new("07b_o7.kmd", None, None),
            RoomStaticInfo::new("07b_o8.kmd", None, None),
            RoomStaticInfo::new("07b_o9.kmd", None, None),
            RoomStaticInfo::new("07b_o10.kmd", None, None),
            RoomStaticInfo::new("07b_o11.kmd", None, None),
            RoomStaticInfo::new("07b_o12.kmd", None, None),
            RoomStaticInfo::new("07b_r1.kmd", None, None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_07b.glb")),
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x - 7.56836,
            y: NUKE_BLDG_OFFSET.y - 10.0,
            z: NUKE_BLDG_OFFSET.z - 43.7012,
        },
        prop_spawns: &[],
    },
    // Electric Floor Gas Chamber
    ROOM_S08A,
    ROOM_S08B, // Japanese animes
    ROOM_S08C, // Ghosts hallway
];

pub struct MgsRoomInfo {
    pub name: &'static str,
    pub tex_paks: &'static [&'static str],
    pub mdl_paks: &'static [&'static str],
    pub static_geom: &'static [RoomStaticInfo],
    pub objects: &'static [MgsObjectInfo],
    pub seal_model: Option<&'static [u8]>,
    pub origin: Vec3,
    pub prop_spawns: &'static [SPropSpawnDef],
}

pub struct RoomStaticInfo {
    pub kmd_filename: &'static str,
    pub meshes_filter: Option<&'static [usize]>,
    pub coll_meshes_filter: Option<&'static [usize]>,
}

impl RoomStaticInfo {
    pub const fn new(
        kmd_filename: &'static str,
        meshes_filter: Option<&'static [usize]>,
        coll_meshes_filter: Option<&'static [usize]>,
    ) -> Self {
        Self {
            kmd_filename,
            meshes_filter,
            coll_meshes_filter,
        }
    }
}

pub enum GenerateProp {
    Dont,
    Static,
    StaticBrk { dstr: &'static str },
}

pub struct MgsObjectInfo {
    pub kmd_filename: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub meshes_filter: Option<&'static [usize]>,
    pub coll_meshes_filter: Option<&'static [usize]>,
    pub generate_prop: GenerateProp,
}

impl MgsObjectInfo {
    pub const fn new(
        kmd_filename: &'static str,
        name: &'static str,
        desc: &'static str,
        meshes_filter: Option<&'static [usize]>,
        coll_meshes_filter: Option<&'static [usize]>,
        generate_prop: GenerateProp,
    ) -> Self {
        Self {
            kmd_filename,
            name,
            desc,
            meshes_filter,
            coll_meshes_filter,
            generate_prop,
        }
    }
}

pub struct SPropSpawnDef {
    pub prop_id: &'static str,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
}

impl Into<PropSpawnDef> for &SPropSpawnDef {
    fn into(self) -> PropSpawnDef {
        PropSpawnDef {
            id: format!("mgsimport.{}", self.prop_id),
            position: self.position,
            rotation: self.rotation,
        }
    }
}

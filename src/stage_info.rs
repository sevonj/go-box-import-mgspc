use glam::Vec3;

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
            RoomStaticInfo::new("00a.kmd", None),
            RoomStaticInfo::new("00a_o1.kmd", None),
            RoomStaticInfo::new("00a_o2.kmd", None),
            RoomStaticInfo::new("00a_o3.kmd", None),
            RoomStaticInfo::new("00a_o4.kmd", None),
            RoomStaticInfo::new("00a_r2.kmd", None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_00a.glb")),
        origin: Vec3 {
            x: -11.0,
            y: -25.0,
            z: -1.0,
        },
    },
    // helipad
    MgsRoomInfo {
        name: "s01a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("01a.kmd", None),
            RoomStaticInfo::new("01a_o1.kmd", None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -50.0,
        },
    },
    // tank hangar
    MgsRoomInfo {
        name: "s02a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("02a.kmd", None),
            RoomStaticInfo::new("02a_d1.kmd", None),
            RoomStaticInfo::new("02a_d2.kmd", None),
            RoomStaticInfo::new("02a_d3.kmd", None),
            RoomStaticInfo::new("02a_d6.kmd", None),
            RoomStaticInfo::new("02a_d7.kmd", None),
            RoomStaticInfo::new("02a_o1.kmd", None),
            RoomStaticInfo::new("02a_o2.kmd", None),
            RoomStaticInfo::new("02a_o3.kmd", None),
            RoomStaticInfo::new("02a_o4.kmd", None),
            RoomStaticInfo::new("02a_o5.kmd", None),
            RoomStaticInfo::new("02a_r1.kmd", None),
            RoomStaticInfo::new("02a_r3.kmd", None),
            RoomStaticInfo::new("02a_r4.kmd", None),
            RoomStaticInfo::new("02a_r5.kmd", None),
            RoomStaticInfo::new("02a_r6.kmd", None),
            RoomStaticInfo::new("02a_r7.kmd", None),
            RoomStaticInfo::new("02a_r9.kmd", None),
            RoomStaticInfo::new("02a_r10.kmd", None),
            RoomStaticInfo::new("02a_r11.kmd", None),
            RoomStaticInfo::new("02a_r12.kmd", None),
            RoomStaticInfo::new("02a_r13.kmd", None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_02a.glb")),
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -84.0,
        },
    },
    // brig
    MgsRoomInfo {
        name: "s03a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("03a.kmd", None),
            RoomStaticInfo::new("03a_d1.kmd", None),
            RoomStaticInfo::new("03a_d2.kmd", None),
            RoomStaticInfo::new("03a_d3.kmd", None),
            RoomStaticInfo::new("03a_d4.kmd", None),
            RoomStaticInfo::new("03a_o1a.kmd", None),
            RoomStaticInfo::new("03a_o1b.kmd", None),
            RoomStaticInfo::new("03a_o2.kmd", None),
            RoomStaticInfo::new("03a_r1.kmd", None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -4.0,
            y: -10.0,
            z: -86.0,
        },
    },
    // giant electric machine torture room that exists there for some reason ???
    MgsRoomInfo {
        name: "s03b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("03b.kmd", None),
            RoomStaticInfo::new("03b_d1.kmd", None),
            RoomStaticInfo::new("03b_d2.kmd", None),
            RoomStaticInfo::new("03b_d3.kmd", None),
            RoomStaticInfo::new("03b_d4.kmd", None),
            RoomStaticInfo::new("03b_o1.kmd", None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -9.8,
            y: -10.0,
            z: -86.0,
        },
    },
    // storage closets and trapdoors
    MgsRoomInfo {
        name: "s04a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("04a.kmd", None),
            RoomStaticInfo::new("04a_d1.kmd", None),
            RoomStaticInfo::new("04a_d2.kmd", None),
            RoomStaticInfo::new("04a_d3.kmd", None),
            RoomStaticInfo::new("04a_d4.kmd", None),
            RoomStaticInfo::new("04a_d6.kmd", None),
            RoomStaticInfo::new("04a_d7.kmd", None),
            RoomStaticInfo::new("04a_d8.kmd", None),
            RoomStaticInfo::new("04a_o1a.kmd", None),
            RoomStaticInfo::new("04a_o2a.kmd", None),
            RoomStaticInfo::new("04a_o3a.kmd", None),
            RoomStaticInfo::new("04a_o4a.kmd", None),
            RoomStaticInfo::new("04a_o5.kmd", None),
            RoomStaticInfo::new("04a_o6.kmd", None),
            RoomStaticInfo::new("04a_o7.kmd", None),
            RoomStaticInfo::new("04a_r1.kmd", None),
            RoomStaticInfo::new("04a_r2.kmd", None),
            RoomStaticInfo::new("04a_r3.kmd", None),
            RoomStaticInfo::new("04a_r4.kmd", None),
            RoomStaticInfo::new("04a_r5.kmd", None),
            RoomStaticInfo::new("04a_r6.kmd", None),
            RoomStaticInfo::new("04a_r7.kmd", None),
            RoomStaticInfo::new("04a_r8.kmd", None),
            RoomStaticInfo::new("04a_r9.kmd", None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -2.5,
            y: -20.0,
            z: -86.5,
        },
    },
    // AT prez / ocelot boss
    MgsRoomInfo {
        name: "s04b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("04b.kmd", None),
            RoomStaticInfo::new("04b_d1.kmd", None),
            RoomStaticInfo::new("04b_d2.kmd", None),
            RoomStaticInfo::new("04b_d3.kmd", None),
            RoomStaticInfo::new("04b_o1a.kmd", None),
            RoomStaticInfo::new("04b_o2a.kmd", None),
            RoomStaticInfo::new("04b_o3a.kmd", None),
            RoomStaticInfo::new("04b_r1.kmd", None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: -4.0,
            y: -20.0,
            z: -70.0,
        },
    },
    // raven tank battle
    MgsRoomInfo {
        name: "s05a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("05a.kmd", None),
            RoomStaticInfo::new("05a_d1a.kmd", None),
            RoomStaticInfo::new("05a_o1.kmd", None),
            RoomStaticInfo::new("05a_o2.kmd", None),
            RoomStaticInfo::new("05a_o3.kmd", None),
        ],
        objects: &[],
        seal_model: None,
        origin: Vec3 {
            x: 2.0,
            y: -0.0,
            z: -130.0,
        },
    },
    // nuke storage
    MgsRoomInfo {
        name: "s06a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("06a.kmd", None),
            RoomStaticInfo::new("06a_d1.kmd", None),
            RoomStaticInfo::new("06a_d2.kmd", None),
            RoomStaticInfo::new("06a_o1.kmd", None),
            RoomStaticInfo::new("06a_r1.kmd", None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_06a.glb")),
        origin: NUKE_BLDG_OFFSET,
    },
    // toilet floor
    MgsRoomInfo {
        name: "s07a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new("07a.kmd", None),
            RoomStaticInfo::new("07a_d1.kmd", None),
            RoomStaticInfo::new("07a_d2.kmd", None),
            RoomStaticInfo::new("07a_d3.kmd", None),
            RoomStaticInfo::new("07a_d4.kmd", None),
            RoomStaticInfo::new("07a_d5.kmd", None),
            RoomStaticInfo::new("07a_d6.kmd", None),
            RoomStaticInfo::new("07a_d7.kmd", None),
            RoomStaticInfo::new("07a_d8.kmd", None),
            RoomStaticInfo::new("07a_d9.kmd", None),
            RoomStaticInfo::new("07a_d10.kmd", None),
            RoomStaticInfo::new("07a_d11.kmd", None),
            RoomStaticInfo::new("07a_d12.kmd", None),
            RoomStaticInfo::new("07a_d13.kmd", None),
            RoomStaticInfo::new("07a_d14.kmd", None),
            RoomStaticInfo::new("07a_d15.kmd", None),
            RoomStaticInfo::new("07a_d16.kmd", None),
            RoomStaticInfo::new("07a_d17.kmd", None),
            RoomStaticInfo::new("07a_d18.kmd", None),
            RoomStaticInfo::new("07a_d19.kmd", None),
            RoomStaticInfo::new("07a_d20.kmd", None),
            RoomStaticInfo::new("07a_o1.kmd", None),
            RoomStaticInfo::new("07a_r1.kmd", None),
            RoomStaticInfo::new("07a_r2.kmd", None),
            RoomStaticInfo::new("07a_r3.kmd", None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_07a.glb")),
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x - 3.41797,
            y: NUKE_BLDG_OFFSET.y - 10.0,
            z: NUKE_BLDG_OFFSET.z - 9.765625,
        },
    },
    // admin office
    MgsRoomInfo {
        name: "s07b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar", "stg_mdl2.dar"],
        static_geom: &[
            RoomStaticInfo::new("07b.kmd", None),
            RoomStaticInfo::new("07b_d1a.kmd", None),
            // KmdImportInfo::new("07b_d2.kmd",   None),
            // KmdImportInfo::new("07b_d3.kmd",   None),
            // KmdImportInfo::new("07b_d4.kmd",   None),
            RoomStaticInfo::new("07b_o1.kmd", None),
            RoomStaticInfo::new("07b_o2.kmd", None),
            RoomStaticInfo::new("07b_o3.kmd", None),
            RoomStaticInfo::new("07b_o4.kmd", None),
            RoomStaticInfo::new("07b_o5.kmd", None),
            RoomStaticInfo::new("07b_o6.kmd", None),
            RoomStaticInfo::new("07b_o7.kmd", None),
            RoomStaticInfo::new("07b_o8.kmd", None),
            RoomStaticInfo::new("07b_o9.kmd", None),
            RoomStaticInfo::new("07b_o10.kmd", None),
            RoomStaticInfo::new("07b_o11.kmd", None),
            RoomStaticInfo::new("07b_o12.kmd", None),
            RoomStaticInfo::new("07b_r1.kmd", None),
        ],
        objects: &[],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_07b.glb")),
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x - 7.56836,
            y: NUKE_BLDG_OFFSET.y - 10.0,
            z: NUKE_BLDG_OFFSET.z - 43.7012,
        },
    },
    // Electric Floor Gas Chamber
    MgsRoomInfo {
        name: "s08a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new(
                "08a.kmd",
                Some(&[
                    // 0, s08c floor
                    // 1, s08c door frame sides
                    // 2, s08c door frame floor
                    // 3, s08c door frame top
                    4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
                    // 17, otacon sneak peek floor
                    // 18, otacon sneak peek wall
                    // 19, otacon sneak peek divider
                    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
                    40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58,
                ]),
            ),
            // KmdImportInfo::new("08a_d1.kmd",   None),
            // KmdImportInfo::new("08a_d2.kmd",   None),
            // KmdImportInfo::new("08a_d3.kmd",   None),
            // KmdImportInfo::new("08a_d4.kmd",   None),
            // KmdImportInfo::new("08a_d5.kmd",   None),
            // KmdImportInfo::new("08a_d6.kmd",   None),
            // KmdImportInfo::new("08a_d7.kmd",   None),
            // KmdImportInfo::new("08a_d8.kmd",   None),
            // KmdImportInfo::new("08a_d9.kmd",   None),
            // KmdImportInfo::new("08a_d10.kmd",   None),
            // KmdImportInfo::new("08a_d11.kmd",   None),
            // KmdImportInfo::new("08a_o1.kmd",   None), // electric floor
            RoomStaticInfo::new("08a_r1.kmd", None), // ev shaft
        ],
        objects: &[
            RoomObjInfo::new(
                "08a_o1a.kmd",
                "08a_o1a",
                "Electrical cabinet",
                /*Vec3 {
                    x: -4.70898,
                    y: 0.0,
                    z: -12.8174,
                },*/
                None,
            ),
            RoomObjInfo::new(
                "08a_o1b.kmd",
                "08a_o1b",
                "Electrical cabinet (broken)",
                None,
            ),
        ],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_08a.glb")),
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x - 5.85938,
            y: NUKE_BLDG_OFFSET.y - 20.0,
            z: NUKE_BLDG_OFFSET.z - 5.37109,
        },
    },
    // Japanese animes room
    MgsRoomInfo {
        name: "s08b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[
            RoomStaticInfo::new(
                "08b.kmd",
                Some(&[
                    0, 1, 2, 3, 4, 5, // 6, s08c door frame, floor
                    7, 8, 9, 10, // playstation, misc props
                    11, 12, 13, // keyboards
                    14, 15, 16, 17, 18, 19, 20,
                ]),
            ),
            RoomStaticInfo::new("08b_o4.kmd", None), // locker interior
                                                     // --- Console panels left to right
                                                     // TODO: Panel arrangement is wrong. But honestly who's gonna notice?
        ],
        objects: &[
            // --- misc
            RoomObjInfo::new("08b_d1.kmd", "08b_d1", "locker door left", None),
            RoomObjInfo::new("08b_d2.kmd", "08b_d2", "locker door right", None),
            // RoomObjInfo::new("08b_d3.kmd", "08b_d3", "door_l4", None),
            RoomObjInfo::new("08b_d4.kmd", "08b_d4", "locker something", None),
            // --- Consoles
            RoomObjInfo::new("08b_o5a.kmd", "08b_o5a", "Console screen", None),
            RoomObjInfo::new("08b_o5b.kmd", "08b_o5b", "Console screen (broken)", None),
            RoomObjInfo::new("08b_o6a.kmd", "08b_o6a", "Console panel", None),
            RoomObjInfo::new("08b_o6b.kmd", "08b_o6b", "Console panel (broken)", None),
            RoomObjInfo::new("08b_o7a.kmd", "08b_o7a", "Console panel", None),
            RoomObjInfo::new("08b_o7b.kmd", "08b_o7b", "Console panel (broken)", None),
            RoomObjInfo::new("08b_o8a.kmd", "08b_o8a", "Console screen", None),
            RoomObjInfo::new("08b_o8b.kmd", "08b_o8b", "Console screen (broken)", None),
            RoomObjInfo::new("08b_o9a.kmd", "08b_o9a", "Console panel", None),
            RoomObjInfo::new("08b_o9b.kmd", "08b_o9b", "Console panel (broken)", None),
            RoomObjInfo::new("08b_o10a.kmd", "08b_o10a", "Console panel", None),
            RoomObjInfo::new("08b_o10b.kmd", "08b_o10b", "Console panel (broken)", None),
            RoomObjInfo::new("08b_o15a.kmd", "08b_o15a", "Console panel", None),
            RoomObjInfo::new("08b_o15b.kmd", "08b_o15b", "Console panel (broken)", None),
            RoomObjInfo::new("08b_o16a.kmd", "08b_o16a", "Console panel", None),
            RoomObjInfo::new("08b_o16b.kmd", "08b_o16b", "Console panel (broken)", None),
            // --- Computers
            RoomObjInfo::new(
                "08b_o11a.kmd",
                "08b_o11a",
                "Supercomputer rack door A",
                None,
            ),
            RoomObjInfo::new(
                "08b_o11b.kmd",
                "08b_o11b",
                "Supercomputer rack door A (broken)",
                None,
            ),
            RoomObjInfo::new(
                "08b_o12a.kmd",
                "08b_o12a",
                "Supercomputer rack door B",
                None,
            ),
            RoomObjInfo::new(
                "08b_o12b.kmd",
                "08b_o112b",
                "Supercomputer rack door B (broken)",
                None,
            ),
            RoomObjInfo::new("08b_o13a.kmd", "08b_o13a", "Large beige cabinet door", None),
            RoomObjInfo::new(
                "08b_o13b.kmd",
                "08b_o13b",
                "large beige cabinet door (broken)",
                None,
            ),
            RoomObjInfo::new("08b_o14a.kmd", "08b_o14a", "Beige rack door", None),
            RoomObjInfo::new("08b_o14b.kmd", "08b_o14b", "Beige rack door (broken)", None),
        ],
        seal_model: Some(include_bytes!("../extra-data/room_seals/seal_08b.glb")),
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x + 13.184558, //12.6953,
            y: NUKE_BLDG_OFFSET.y - 20.0,
            z: NUKE_BLDG_OFFSET.z - 21.4844,
        },
    },
    // ghosts
    MgsRoomInfo {
        name: "s08c",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_geom: &[RoomStaticInfo::new("08c.kmd", None)],
        objects: &[
           // MgsRoomObjInfo::new("08c_d1.kmd", "08c_d1", None),
           // MgsRoomObjInfo::new("08c_d2.kmd", "08c_d2", None),
        ],
        seal_model: None,
        origin: Vec3 {
            // pretty good
            x: NUKE_BLDG_OFFSET.x + 6.59179,
            y: NUKE_BLDG_OFFSET.y - 20.0,
            z: NUKE_BLDG_OFFSET.z - 3.90725,
        },
    },
];

pub struct MgsRoomInfo {
    pub name: &'static str,
    pub tex_paks: &'static [&'static str],
    pub mdl_paks: &'static [&'static str],
    pub static_geom: &'static [RoomStaticInfo],
    pub objects: &'static [RoomObjInfo],
    pub seal_model: Option<&'static [u8]>,
    pub origin: Vec3,
}

pub struct RoomStaticInfo {
    pub kmd_filename: &'static str,
    /// None to include all
    pub meshes: Option<&'static [usize]>,
}

impl RoomStaticInfo {
    pub const fn new(kmd_filename: &'static str, meshes: Option<&'static [usize]>) -> Self {
        Self {
            kmd_filename,
            meshes,
        }
    }
}

pub struct RoomObjInfo {
    pub kmd_filename: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    /// None to include all
    pub meshes: Option<&'static [usize]>,
}

impl RoomObjInfo {
    pub const fn new(
        kmd_filename: &'static str,
        name: &'static str,
        desc: &'static str,
        meshes: Option<&'static [usize]>,
    ) -> Self {
        Self {
            kmd_filename,
            name,
            desc,
            meshes,
        }
    }
}

use crate::types::Vector;

pub const ROOMS: &'static [MgsRoomInfo] = &[
    // underwater entrance
    MgsRoomInfo {
        name: "s00a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "00a_o1.kmd",
            "00a_o2.kmd",
            "00a_o3.kmd",
            "00a_o4.kmd",
            "00a_r2.kmd",
            "00a.kmd",
        ],
        origin: Vector {
            x: -11200,
            y: -25 * 1024,
            z: -1024,
        },
    },
    // helipad
    MgsRoomInfo {
        name: "s01a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &["01a_o1.kmd", "01a.kmd"],
        origin: Vector {
            x: 0,
            y: 0,
            z: -50 * 1024,
        },
    },
    // tank hangar
    MgsRoomInfo {
        name: "s02a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "02a_d1.kmd",
            "02a_d2.kmd",
            "02a_d3.kmd",
            "02a_d6.kmd",
            "02a_d7.kmd",
            "02a_o1.kmd",
            "02a_o2.kmd",
            "02a_o3.kmd",
            "02a_o4.kmd",
            "02a_o5.kmd",
            "02a_r1.kmd",
            "02a_r3.kmd",
            "02a_r4.kmd",
            "02a_r5.kmd",
            "02a_r6.kmd",
            "02a_r7.kmd",
            "02a_r9.kmd",
            "02a_r10.kmd",
            "02a_r11.kmd",
            "02a_r12.kmd",
            "02a_r13.kmd",
            "02a.kmd",
        ],
        origin: Vector {
            x: 0 * 1024,
            y: 0,
            z: -84 * 1024,
        },
    },
    // brig
    MgsRoomInfo {
        name: "s03a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "03a_d1.kmd",
            "03a_d2.kmd",
            "03a_d3.kmd",
            "03a_d4.kmd",
            "03a_o1a.kmd",
            "03a_o1b.kmd",
            "03a_o2.kmd",
            "03a_r1.kmd",
            "03a.kmd",
        ],
        origin: Vector {
            x: -4 * 1025,
            y: -10 * 1024,
            z: -86 * 1024,
        },
    },
    // giant electric machine torture room that exists there for some reason ???
    MgsRoomInfo {
        name: "s03b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "03b_d1.kmd",
            "03b_d2.kmd",
            "03b_d3.kmd",
            "03b_d4.kmd",
            "03b_o1.kmd",
            "03b.kmd",
        ],
        origin: Vector {
            x: -10 * 1024 - 200,
            y: -10 * 1024,
            z: -86 * 1024,
        },
    },
    // storage closets and trapdoors
    MgsRoomInfo {
        name: "s04a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "04a_d1.kmd",
            "04a_d2.kmd",
            "04a_d3.kmd",
            "04a_d4.kmd",
            "04a_d6.kmd",
            "04a_d7.kmd",
            "04a_d8.kmd",
            "04a_o1a.kmd",
            "04a_o2a.kmd",
            "04a_o3a.kmd",
            "04a_o4a.kmd",
            "04a_o5.kmd",
            "04a_o6.kmd",
            "04a_o7.kmd",
            "04a_r1.kmd",
            "04a_r2.kmd",
            "04a_r3.kmd",
            "04a_r4.kmd",
            "04a_r5.kmd",
            "04a_r6.kmd",
            "04a_r7.kmd",
            "04a_r8.kmd",
            "04a_r9.kmd",
            "04a.kmd",
        ],
        origin: Vector {
            x: -2 * 1024 - 512,
            y: -20 * 1024,
            z: -86 * 1024 - 512,
        },
    },
    // AT prez / ocelot boss
    MgsRoomInfo {
        name: "s04b",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "04b_d1.kmd",
            "04b_d2.kmd",
            "04b_d3.kmd",
            "04b_o1a.kmd",
            "04b_o2a.kmd",
            "04b_o3a.kmd",
            "04b_r1.kmd",
            "04b.kmd",
        ],
        origin: Vector {
            x: -4 * 1024,
            y: -20 * 1024,
            z: -70 * 1024,
        },
    },
    // raven tank battle
    MgsRoomInfo {
        name: "s05a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar", "stg_tex3.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "05a_d1a.kmd",
            "05a_o1.kmd",
            "05a_o2.kmd",
            "05a_o3.kmd",
            "05a.kmd",
        ],
        origin: Vector {
            x: 2 * 1024,
            y: -0 * 1024,
            z: -130 * 1024,
        },
    },
    // nuke storage
    MgsRoomInfo {
        name: "s06a",
        tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
        mdl_paks: &["stg_mdl1.dar"],
        static_models: &[
            "06a_d1.kmd",
            "06a_d2.kmd",
            "06a_o1.kmd",
            "06a_r1.kmd",
            "06a.kmd",
        ],
        origin: Vector {
            x: -6 * 1024,
            y: -5 * 1024,
            z: -184 * 1024,
        },
    },
];

pub struct MgsRoomInfo {
    pub name: &'static str,
    pub tex_paks: &'static [&'static str],
    pub mdl_paks: &'static [&'static str],
    pub static_models: &'static [&'static str],
    pub origin: Vector,
}

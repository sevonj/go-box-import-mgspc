use crate::stage_info::MgsRoomInfo;
use crate::stage_info::NUKE_BLDG_OFFSET;
use crate::stage_info::RoomStaticInfo;
use crate::stage_info::SPropSpawnDef;
use crate::stage_info::{GenerateProp, MgsObjectInfo};
use glam::Vec3;

pub const ROOM_S08B: MgsRoomInfo = MgsRoomInfo {
    name: "s08b",
    tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
    mdl_paks: &["stg_mdl1.dar"],
    static_geom: &[
        RoomStaticInfo::new(
            "08b.kmd",
            Some(&[
                0, // bookshelves
                1, // server cabinets
                2, // consoles under
                3, // desks under
                4, // locker
                5, // locker front
                // 6, // exit to s08c
                7,  // ceiling
                8,  // ceil lights
                9,  // consoles
                10, // playstation, misc props
                11, // floor
                12, // 3x walls + s08c door frame
                13, // keyboards
                14, // top-down lights
                15, // console/desk shadow
                16, // divider, desk
                17, // rear wall
                18, // divider, desk
                19, // divider
                20, // divider, desk
            ]),
            Some(&[
                0, // bookshelves
                1, // server cabinets
                2, // consoles under
                3, // desks under
                4, // locker
                5, // locker front
                // 6, // exit to s08c
                7,  // ceiling
                8,  // ceil lights
                9,  // consoles
                10, // playstation, misc props
                11, // floor
                12, // 3x walls + s08c door frame
                13, // keyboards
                // 14, // top-down lights
                15, // console/desk shadow
                16, // divider, desk
                17, // rear wall
                18, // divider, desk
                19, // divider
                20, // divider, desk
            ]),
        ),
        RoomStaticInfo::new("08b_o4.kmd", None, None), // locker interior. incl all
    ],
    objects: &[
        // --- Named
        MgsObjectInfo::new(
            "1c_crd.kmd",
            "1c_crd",
            "Lv.4 Card",
            None,
            None,
            GenerateProp::Static,
        ),
        MgsObjectInfo::new(
            "isu.kmd",
            "isu",
            "Office chair",
            None,
            None,
            GenerateProp::Static,
        ),
        MgsObjectInfo::new(
            "katana.kmd",
            "katana",
            "Katana",
            None,
            None,
            GenerateProp::Static,
        ),
        MgsObjectInfo::new(
            "nanao.kmd",
            "nanao",
            "Computer",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "nanao_d" },
        ),
        MgsObjectInfo::new(
            "nanao_d.kmd",
            "nanao_d",
            "Computer (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "nja_ball.kmd",
            "nja_ball",
            "Ninja electric discharge attack effect",
            None,
            None,
            GenerateProp::Static,
        ),
        // --- Misc
        MgsObjectInfo::new(
            "08b_d1.kmd",
            "08b_d1",
            "Locker door left",
            None,
            None,
            GenerateProp::Static,
        ),
        MgsObjectInfo::new(
            "08b_d2.kmd",
            "08b_d2",
            "Locker door right",
            None,
            None,
            GenerateProp::Static,
        ),
        // RoomObjInfo::new("08b_d3.kmd", "08b_d3", "door_l4", None),
        MgsObjectInfo::new(
            "08b_d4.kmd",
            "08b_d4",
            "Locker something",
            None,
            None,
            GenerateProp::Static,
        ),
        // --- Consoles
        MgsObjectInfo::new(
            "08b_o5a.kmd",
            "08b_o5a",
            "Console screen",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o5b" },
        ),
        MgsObjectInfo::new(
            "08b_o5b.kmd",
            "08b_o5b",
            "Console screen (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o6a.kmd",
            "08b_o6a",
            "Console panel",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o6b" },
        ),
        MgsObjectInfo::new(
            "08b_o6b.kmd",
            "08b_o6b",
            "Console panel (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o7a.kmd",
            "08b_o7a",
            "Console panel",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o7b" },
        ),
        MgsObjectInfo::new(
            "08b_o7b.kmd",
            "08b_o7b",
            "Console panel (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o8a.kmd",
            "08b_o8a",
            "Console screen",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o8b" },
        ),
        MgsObjectInfo::new(
            "08b_o8b.kmd",
            "08b_o8b",
            "Console screen (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o9a.kmd",
            "08b_o9a",
            "Console panel",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o9b" },
        ),
        MgsObjectInfo::new(
            "08b_o9b.kmd",
            "08b_o9b",
            "Console panel (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o10a.kmd",
            "08b_o10a",
            "Console panel",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o10b" },
        ),
        MgsObjectInfo::new(
            "08b_o10b.kmd",
            "08b_o10b",
            "Console panel (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o15a.kmd",
            "08b_o15a",
            "Console panel",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o15b" },
        ),
        MgsObjectInfo::new(
            "08b_o15b.kmd",
            "08b_o15b",
            "Console panel (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o16a.kmd",
            "08b_o16a",
            "Console panel",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o16b" },
        ),
        MgsObjectInfo::new(
            "08b_o16b.kmd",
            "08b_o16b",
            "Console panel (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        // --- Computers
        MgsObjectInfo::new(
            "08b_o11a.kmd",
            "08b_o11a",
            "Supercomputer rack door A",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o11b" },
        ),
        MgsObjectInfo::new(
            "08b_o11b.kmd",
            "08b_o11b",
            "Supercomputer rack door A (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o12a.kmd",
            "08b_o12a",
            "Supercomputer rack door B",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o12b" },
        ),
        MgsObjectInfo::new(
            "08b_o12b.kmd",
            "08b_o12b",
            "Supercomputer rack door B (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o13a.kmd",
            "08b_o13a",
            "Large beige cabinet door",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o13b" },
        ),
        MgsObjectInfo::new(
            "08b_o13b.kmd",
            "08b_o13b",
            "large beige cabinet door (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
        MgsObjectInfo::new(
            "08b_o14a.kmd",
            "08b_o14a",
            "Beige rack door",
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08b_o14b" },
        ),
        MgsObjectInfo::new(
            "08b_o14b.kmd",
            "08b_o14b",
            "Beige rack door (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
    ],
    seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_08b.glb")),
    origin: Vec3 {
        // pretty good
        x: NUKE_BLDG_OFFSET.x + 13.184558, //12.6953,
        y: NUKE_BLDG_OFFSET.y - 20.0,
        z: NUKE_BLDG_OFFSET.z - 21.4844,
    },
    prop_spawns: &[
        // --- Server area
        // Dark blue server doors left to right
        SPropSpawnDef {
            prop_id: "08b_o12a",
            position: [-10.0098, 0.0, 1.95312],
            rotation: [0.0, 90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o11a",
            position: [-10.0098, 0.0, 0.976562],
            rotation: [0.0, 90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o11a",
            position: [-10.0098, 0.0, 0.0],
            rotation: [0.0, 90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o11a",
            position: [-10.0098, 0.0, -0.976562],
            rotation: [0.0, 90.0, 0.0],
        },
        // Bigger beige
        SPropSpawnDef {
            prop_id: "08b_o13a",
            position: [-10.9863, 0.0, -3.17383],
            rotation: [0.0, 0.0, 0.0],
        },
        // Smaller beige
        SPropSpawnDef {
            prop_id: "08b_o14a",
            position: [-9.39941, 0.0, -3.41797],
            rotation: [0.0, 0.0, 0.0],
        },
        // --- Consoles left to right
        // Group 1
        SPropSpawnDef {
            prop_id: "08b_o6a",
            position: [-5.61523, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o7a",
            position: [-4.39453, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o8a",
            position: [-3.41797, 0.732422, -3.66211],
            rotation: [0.0, -45.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o9a",
            position: [-2.68555, 0.732422, -2.92871],
            rotation: [0.0, -90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o10a",
            position: [-2.68555, 0.732422, -1.95312],
            rotation: [0.0, -90.0, 0.0],
        },
        // Group 2
        SPropSpawnDef {
            prop_id: "08b_o15a",
            position: [-1.2207, 0.732422, -0.488281],
            rotation: [0.0, 90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o7a",
            position: [-1.2207, 0.732422, -1.95313],
            rotation: [0.0, 90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o8a",
            position: [-1.2207, 0.732422, -2.92969],
            rotation: [0.0, 45.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o9a",
            position: [-0.487305, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o16a",
            position: [0.488281, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        // middle screen
        SPropSpawnDef {
            prop_id: "08b_o5a",
            position: [1.70898, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        // Group 3 (1 repeated)
        SPropSpawnDef {
            prop_id: "08b_o6a",
            position: [-5.61523 + 8.30078, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o7a",
            position: [-4.39453 + 8.30078, 0.732422, -3.66211],
            rotation: [0.0, 0.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o8a",
            position: [-3.41797 + 8.30078, 0.732422, -3.66211],
            rotation: [0.0, -45.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o9a",
            position: [-2.68555 + 8.30078, 0.732422, -2.92871],
            rotation: [0.0, -90.0, 0.0],
        },
        SPropSpawnDef {
            prop_id: "08b_o10a",
            position: [-2.68555 + 8.30078, 0.732422, -1.95312],
            rotation: [0.0, -90.0, 0.0],
        },
    ],
};

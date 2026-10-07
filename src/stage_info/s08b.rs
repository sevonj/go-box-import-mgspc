use crate::stage_info::MgsRoomInfo;
use crate::stage_info::NUKE_BLDG_OFFSET;
use crate::stage_info::RoomObjInfo;
use crate::stage_info::RoomStaticInfo;
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
        RoomObjInfo::new("1c_crd.kmd", "1c_crd", "Lv.4 Card", None, None),
        RoomObjInfo::new("isu.kmd", "isu", "Office chair", None, None),
        RoomObjInfo::new("katana.kmd", "katana", "Katana", None, None),
        RoomObjInfo::new("nanao.kmd", "nanao", "Computer", None, None),
        RoomObjInfo::new("nanao_d.kmd", "nanao_d", "Computer (destroyed)", None, None),
        RoomObjInfo::new(
            "nja_ball.kmd",
            "nja_ball",
            "Ninja electric discharge attack effect",
            None,
            None,
        ),
        // --- Misc
        RoomObjInfo::new("08b_d1.kmd", "08b_d1", "Locker door left", None, None),
        RoomObjInfo::new("08b_d2.kmd", "08b_d2", "Locker door right", None, None),
        // RoomObjInfo::new("08b_d3.kmd", "08b_d3", "door_l4", None),
        RoomObjInfo::new("08b_d4.kmd", "08b_d4", "Locker something", None, None),
        // --- Consoles
        RoomObjInfo::new("08b_o5a.kmd", "08b_o5a", "Console screen", None, None),
        RoomObjInfo::new(
            "08b_o5b.kmd",
            "08b_o5b",
            "Console screen (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o6a.kmd", "08b_o6a", "Console panel", None, None),
        RoomObjInfo::new(
            "08b_o6b.kmd",
            "08b_o6b",
            "Console panel (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o7a.kmd", "08b_o7a", "Console panel", None, None),
        RoomObjInfo::new(
            "08b_o7b.kmd",
            "08b_o7b",
            "Console panel (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o8a.kmd", "08b_o8a", "Console screen", None, None),
        RoomObjInfo::new(
            "08b_o8b.kmd",
            "08b_o8b",
            "Console screen (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o9a.kmd", "08b_o9a", "Console panel", None, None),
        RoomObjInfo::new(
            "08b_o9b.kmd",
            "08b_o9b",
            "Console panel (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o10a.kmd", "08b_o10a", "Console panel", None, None),
        RoomObjInfo::new(
            "08b_o10b.kmd",
            "08b_o10b",
            "Console panel (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o15a.kmd", "08b_o15a", "Console panel", None, None),
        RoomObjInfo::new(
            "08b_o15b.kmd",
            "08b_o15b",
            "Console panel (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o16a.kmd", "08b_o16a", "Console panel", None, None),
        RoomObjInfo::new(
            "08b_o16b.kmd",
            "08b_o16b",
            "Console panel (destroyed)",
            None,
            None,
        ),
        // --- Computers
        RoomObjInfo::new(
            "08b_o11a.kmd",
            "08b_o11a",
            "Supercomputer rack door A",
            None,
            None,
        ),
        RoomObjInfo::new(
            "08b_o11b.kmd",
            "08b_o11b",
            "Supercomputer rack door A (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new(
            "08b_o12a.kmd",
            "08b_o12a",
            "Supercomputer rack door B",
            None,
            None,
        ),
        RoomObjInfo::new(
            "08b_o12b.kmd",
            "08b_o112b",
            "Supercomputer rack door B (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new(
            "08b_o13a.kmd",
            "08b_o13a",
            "Large beige cabinet door",
            None,
            None,
        ),
        RoomObjInfo::new(
            "08b_o13b.kmd",
            "08b_o13b",
            "large beige cabinet door (destroyed)",
            None,
            None,
        ),
        RoomObjInfo::new("08b_o14a.kmd", "08b_o14a", "Beige rack door", None, None),
        RoomObjInfo::new(
            "08b_o14b.kmd",
            "08b_o14b",
            "Beige rack door (destroyed)",
            None,
            None,
        ),
    ],
    seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_08b.glb")),
    origin: Vec3 {
        // pretty good
        x: NUKE_BLDG_OFFSET.x + 13.184558, //12.6953,
        y: NUKE_BLDG_OFFSET.y - 20.0,
        z: NUKE_BLDG_OFFSET.z - 21.4844,
    },
};

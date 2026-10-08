use crate::stage_info::MgsObjectInfo;
use crate::stage_info::NUKE_BLDG_OFFSET;
use crate::stage_info::RoomStaticInfo;
use crate::stage_info::{GenerateProp, MgsRoomInfo};
use glam::Vec3;

pub const ROOM_S08A: MgsRoomInfo = MgsRoomInfo {
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
                4,  // cubicle desk tops
                5,  // above-door lights
                6,  // above-door lights
                7,  // floor pieces
                8,  // glassdoor danger notice
                9,  // chair legs
                10, // chair legs
                11, // chair legs
                12, // chair legs
                13, // chair legs
                14, // chair legs
                15, // chair legs
                16, // chair legs
                // 17, otacon sneak peek floor
                // 18, otacon sneak peek wall
                // 19, otacon sneak peek divider
                20, // window HV notice
                21, // cubicle wall parts
                22, // wall lights, blackout
                23, // cubicle/hallway wall parts
                24, // wall, misc
                25, // cubicle machine
                26, // wall
                27, // wall, misc
                28, // wall
                29, // ev door frame
                30, // top-down lights
                31, // gas window
                32, // teal glass doors
                33, // chair seats, leg bottoms
                34, // wall light pieces
                35, // wall
                36, // floor pieces
                37, // floor pieces
                38, // floor pieces
                39, // floor pieces
                40, // wall
                41, // wall
                42, // backrests/computers/utility cabinet
                43, // lab desk, airlock
                44, // cubicle furniture
                45, // lab/cubicle furniture
                46, // wall tops
                47, // elec. machine
                48, // misc
                49, // elec. entrance parts
                50, // elec. room wall
                51, // wall
                52, // wall top piece
                53, // entrance walls
                54, // lab/cubicle furniture
                55, // lab/cubicle furniture
                56, // misc
                57, // wall, ev shaft
                58, // ev shaft blackoug
            ]),
            Some(&[
                // 0, s08c floor
                // 1, s08c door frame sides
                // 2, s08c door frame floor
                // 3, s08c door frame top
                4, // cubicle desk tops
                5, // above-door lights
                6, // above-door lights
                7, // floor pieces
                // 8,  // glassdoor danger notice
                9,  // chair legs
                10, // chair legs
                11, // chair legs
                12, // chair legs
                13, // chair legs
                14, // chair legs
                15, // chair legs
                16, // chair legs
                // 17, otacon sneak peek floor
                // 18, otacon sneak peek wall
                // 19, otacon sneak peek divider
                // 20, // window HV notice
                21, // cubicle wall parts
                22, // wall lights, blackout
                23, // cubicle/hallway wall parts
                24, // wall, misc
                25, // cubicle machine
                26, // wall
                27, // wall, misc
                28, // wall
                29, // ev door frame
                // 30, // top-down lights
                31, // gas window
                32, // teal glass doors
                33, // chair seats, leg bottoms
                34, // wall light pieces
                35, // wall
                36, // floor pieces
                37, // floor pieces
                38, // floor pieces
                39, // floor pieces
                40, // wall
                41, // wall
                42, // backrests/computers/utility cabinet
                43, // lab desk, airlock
                44, // cubicle furniture
                45, // lab/cubicle furniture
                46, // wall tops
                47, // elec. machine
                48, // misc
                49, // elec. entrance parts
                50, // elec. room wall
                51, // wall
                52, // wall top piece
                53, // entrance walls
                54, // lab/cubicle furniture
                55, // lab/cubicle furniture
                56, // misc
                57, // wall, ev shaft
                    // 58, // ev shaft blackoug
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
        RoomStaticInfo::new("08a_r1.kmd", None, None), // ev shaft
    ],
    objects: &[
        // --- named
        MgsObjectInfo::new(
            "gca_arm.kmd",
            "gca_arm",
            "Gun camera arm",
            None,
            None,
            GenerateProp::Static,
        ),
        MgsObjectInfo::new(
            "gca_gun.kmd",
            "gca_gun",
            "Gun camera",
            None,
            None,
            GenerateProp::Static,
        ),
        // --- misc
        MgsObjectInfo::new(
            "08a_o1a.kmd",
            "08a_o1a",
            "Electrical cabinet",
            /*Vec3 {
                x: -4.70898,
                y: 0.0,
                z: -12.8174,
            },*/
            None,
            None,
            GenerateProp::StaticBrk { dstr: "08a_o1b" },
        ),
        MgsObjectInfo::new(
            "08a_o1b.kmd",
            "08a_o1b",
            "Electrical cabinet (destroyed)",
            None,
            None,
            GenerateProp::Dont,
        ),
    ],
    seal_model: Some(include_bytes!("../../extra-data/room_seals/seal_08a.glb")),
    origin: Vec3 {
        // pretty good
        x: NUKE_BLDG_OFFSET.x - 5.85938,
        y: NUKE_BLDG_OFFSET.y - 20.0,
        z: NUKE_BLDG_OFFSET.z - 5.37109,
    },
    prop_spawns: &[],
};

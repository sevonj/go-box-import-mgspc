use crate::stage_info::MgsRoomInfo;
use crate::stage_info::NUKE_BLDG_OFFSET;
use crate::stage_info::MgsObjectInfo;
use crate::stage_info::RoomStaticInfo;
use glam::Vec3;

pub const ROOM_S08C: MgsRoomInfo = MgsRoomInfo {
    name: "s08c",
    tex_paks: &["stg_tex1.dar", "stg_tex2.dar"],
    mdl_paks: &["stg_mdl1.dar"],
    static_geom: &[RoomStaticInfo::new(
        "08c.kmd",
        Some(&[
            0, // sentry part
            1, // floor guns
            2, // door frame sides
            // 3, // both exits floors
            4,  // ceil airlock
            5,  // ceil hallway
            6,  // door frame bottoms
            7,  // door frame tops
            8,  // sentry part
            9,  // airlock machine, locker
            10, // wall device thing
            11, // wall airlock
            12, // floors
            13, // wall hallway
            14, // airlock wall conduit
            15, // sentry part
            16, // sentry part
            17, // sentry part
            18, // sentry part
            19, // sentry part
            20, // sentry part
            21, // sentry part
            22, // sentry part
            23, // sentry part
            24, // sentry part
            25, // sentry part
            26, // sentry part
            27, // sentry part
            28, // sentry part
        ]),
        Some(&[
            0, // sentry part
            1, // floor guns
            2, // door frame sides
            // 3, // both exits floors
            4,  // ceil airlock
            5,  // ceil hallway
            6,  // door frame bottoms
            7,  // door frame tops
            8,  // sentry part
            9,  // airlock machine, locker
            10, // wall device thing
            11, // wall airlock
            12, // floors
            13, // wall hallway
            14, // airlock wall conduit
            15, // sentry part
            16, // sentry part
            17, // sentry part
            18, // sentry part
            19, // sentry part
            20, // sentry part
            21, // sentry part
            22, // sentry part
            23, // sentry part
            24, // sentry part
            25, // sentry part
            26, // sentry part
            27, // sentry part
            28, // sentry part
        ]),
    )],
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
    prop_spawns: &[]
};

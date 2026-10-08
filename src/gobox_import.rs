use crate::error::MgsError;
use crate::gobox_types::ContentPak;
use crate::gobox_types::PropSpawnDef;
use crate::gobox_types::PropStaticDef;
use crate::gobox_types::StageDef;
use crate::gobox_types::{Coll, PropStaticBrkDef};
use crate::intermediary_mesh::IntermediaryMatData;
use crate::intermediary_mesh::IntermediaryMesh;
use crate::intermediary_mesh::IntermediarySurf;
use crate::intermediary_mesh::IntermediaryTexInfo;
use crate::intermediary_mesh::MatTranspMode;
use crate::mgs_types::Darchive;
use crate::mgs_types::Kmd;
use crate::stage_info::*;
use crate::util::*;
use image::DynamicImage;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use zip::ZipArchive;

pub struct TempRoomData {
    pub static_mesh: IntermediaryMesh,
    pub static_coll_mesh: IntermediaryMesh,
    pub objects: Vec<TempLooseModelData>,
}

#[derive(Debug)]
pub struct TempLooseModelData {
    pub name: String,
    pub mesh: IntermediaryMesh,
    pub coll: Coll,
    pub prop_def: TempPropDefEnum,
}

#[derive(Debug)]
enum TempPropDefEnum {
    None,
    Static(PropStaticDef),
    StaticBrk(PropStaticBrkDef),
}

pub fn gobox_import(game_dir: &Path, out_path: &Path) {
    let _ = std::fs::remove_dir_all(out_path);
    let mut stage_zip = ZipArchive::new(File::open(game_dir.join("stage.mgz")).unwrap()).unwrap();

    std::fs::create_dir_all(&out_path).unwrap();
    std::fs::write(out_path.join(".gdignore"), []).unwrap();
    std::fs::write(out_path.join(".gitignore"), "*\n".as_bytes()).unwrap();

    let out_temp_path = out_path.join("temp");
    let out_models_path = out_path.join("models");
    let out_textures_path = out_path.join("textures");
    let out_stage_path = out_path.join("stages").join("shadmo");
    let out_props_path = out_path.join("props");

    std::fs::create_dir_all(&out_temp_path).unwrap();
    std::fs::create_dir_all(&out_models_path).unwrap();
    std::fs::create_dir_all(&out_textures_path).unwrap();
    std::fs::create_dir_all(&out_stage_path).unwrap();
    std::fs::create_dir_all(&out_props_path).unwrap();

    let mut pak = ContentPak {
        stages: vec![String::from("shadmo")],
        props_static: vec![],
        props_static_brk: vec![],
    };

    let mut stage_def = StageDef {
        id: String::from("mgsimport.shadmo"),
        version: 1,
        external: true,
        name: String::from("Shadow Moses Island"),
        description: String::from("Imported from Metal Gear Solid"),
        thumbnail_path: String::from("mgsimport/textures/m3r_path_door1.png"),
        model_path: String::from("mgsimport/models/stage.glb"),
        coll_path: String::from("mgsimport/models/stage.coll"),
        player_start_position: [-17.5, -25.0, -6.0],
        player_start_rotation: [0.0, 0.0, 0.0],
        props_static: vec![],
    };

    let rooms: Vec<TempRoomData> = ROOMS
        .iter()
        .map(|r| get_a_room(&mut stage_zip, r, &out_temp_path))
        .collect();

    let mut world_mesh = IntermediaryMesh::empty();
    let mut world_coll_mesh = IntermediaryMesh::empty();
    for room in &rooms {
        world_mesh = world_mesh.join(&room.static_mesh);
        world_coll_mesh = world_coll_mesh.join(&room.static_coll_mesh);
    }
    world_mesh
        .surfaces
        .values_mut()
        .for_each(|surf| surf.dedupe());

    for surf in world_mesh.surfaces.values_mut() {
        place_texture(surf, &out_textures_path);
    }

    let stage_glb_path = out_models_path.join("stage.glb");
    std::fs::write(&stage_glb_path, world_mesh.to_glb()).unwrap();

    let stage_coll_path = stage_glb_path.with_extension("coll");
    std::fs::write(&stage_coll_path, world_coll_mesh.to_coll().to_bytes()).unwrap();

    for room in &rooms {
        for loose_model in &room.objects {
            for surf in loose_model.mesh.surfaces.values() {
                place_texture(surf, &out_textures_path);
            }

            let glb_path = out_models_path
                .join(&loose_model.name)
                .with_extension("glb");
            assert!(!glb_path.exists());
            std::fs::write(&glb_path, loose_model.mesh.to_glb()).unwrap();

            let coll_path = glb_path.with_extension("coll");
            assert!(!coll_path.exists());
            std::fs::write(&coll_path, loose_model.coll.to_bytes()).unwrap();

            let def_path = out_props_path
                .join(&loose_model.name)
                .with_extension("json");
            assert!(!def_path.exists());

            match &loose_model.prop_def {
                TempPropDefEnum::None => continue,
                TempPropDefEnum::Static(def) => {
                    std::fs::write(def_path, serde_json::to_string(def).unwrap().as_bytes())
                        .unwrap();
                    pak.props_static.push(loose_model.name.clone());
                }
                TempPropDefEnum::StaticBrk(def) => {
                    std::fs::write(def_path, serde_json::to_string(def).unwrap().as_bytes())
                        .unwrap();
                    pak.props_static_brk.push(loose_model.name.clone());
                }
            }
        }
    }

    for room in ROOMS {
        for spawn in room.prop_spawns {
            let mut spawn: PropSpawnDef = spawn.into();
            spawn.position[0] += room.origin.x;
            spawn.position[1] += room.origin.y;
            spawn.position[2] += room.origin.z;
            stage_def.props_static.push(spawn);
        }
    }

    File::create(out_stage_path.join("stage.json"))
        .unwrap()
        .write_all(serde_json::to_string(&stage_def).unwrap().as_bytes())
        .unwrap();

    File::create(out_path.join("pak.json"))
        .unwrap()
        .write_all(serde_json::to_string(&pak).unwrap().as_bytes())
        .unwrap();
}

fn get_a_room(
    stage_zip: &mut ZipArchive<File>,
    room_info: &MgsRoomInfo,
    temp_path: &Path,
) -> TempRoomData {
    let room_name = room_info.name;
    let room_temp_path = temp_path.join("stages").join(room_name);
    std::fs::create_dir_all(&room_temp_path).unwrap();

    let mut room_textures: HashMap<String, DynamicImage> = HashMap::new();
    for filename in room_info.tex_paks {
        let filepath = format!("stage/{}/{filename}", room_name);
        let archive = Darchive::read(&file_from_zip(stage_zip, &filepath), &mut 0).unwrap();
        let pak_textures = get_packed_textures(&archive).unwrap();
        for key in pak_textures.keys() {
            assert!(!room_textures.contains_key(key));
        }
        room_textures.extend(pak_textures);
    }

    let mut room_texture_names: HashMap<u16, String> = HashMap::new();
    for stem in room_textures.keys() {
        let hash = file_name_hash(stem);
        assert!(!room_texture_names.contains_key(&hash));
        room_texture_names.insert(hash, stem.to_owned());
    }

    let mut models: HashMap<String, Kmd> = HashMap::new();
    for filename in room_info.mdl_paks {
        let filepath = format!("stage/{}/{filename}", room_name);
        let archive = Darchive::read(&file_from_zip(stage_zip, &filepath), &mut 0).unwrap();

        for file in archive.files() {
            let filename = file.name();
            if !filename.ends_with(".kmd") {
                continue;
            }
            let kmd = Kmd::read(file.contents(), &mut 0).unwrap();
            models.insert(filename.to_string(), kmd);
        }
    }

    let mut static_mesh = IntermediaryMesh::empty();
    let mut static_coll_mesh = IntermediaryMesh::empty();
    for static_info in room_info.static_geom {
        let kmd = models
            .get(static_info.kmd_filename)
            .expect(static_info.kmd_filename);

        if let Some(curated) = static_info.meshes_filter {
            for i in curated {
                let mut joiner = IntermediaryMesh::from_kmd(&kmd.meshes[*i], room_name.to_owned());
                joiner.origin += room_info.origin;
                static_mesh = static_mesh.join(&joiner);
            }
        } else {
            for mesh in &kmd.meshes {
                let mut joiner = IntermediaryMesh::from_kmd(mesh, room_name.to_owned());
                joiner.origin += room_info.origin;
                static_mesh = static_mesh.join(&joiner);
            }
        };

        if let Some(curated) = static_info.coll_meshes_filter {
            for i in curated {
                let mut joiner = IntermediaryMesh::from_kmd(&kmd.meshes[*i], room_name.to_owned());
                joiner.origin += room_info.origin;
                static_coll_mesh = static_coll_mesh.join(&joiner);
            }
        } else {
            for mesh in &kmd.meshes {
                let mut joiner = IntermediaryMesh::from_kmd(mesh, room_name.to_owned());
                joiner.origin += room_info.origin;
                static_coll_mesh = static_coll_mesh.join(&joiner);
            }
        };
    }
    static_mesh.surfaces.values_mut().for_each(|surf| {
        process_surf_material(surf, &room_textures, &room_texture_names, &room_temp_path)
    });
    if let Some(glb) = room_info.seal_model {
        static_mesh = static_mesh.join(&IntermediaryMesh::from_seal_glb(glb, room_info.origin));
    }

    let mut loose_models = Vec::new();
    for obj_info in room_info.objects {
        let name = obj_info.name.to_owned();
        let kmd = models
            .get(obj_info.kmd_filename)
            .expect(obj_info.kmd_filename);

        let mut mesh = IntermediaryMesh::empty();
        if let Some(curated) = obj_info.meshes_filter {
            for i in curated {
                let joiner = IntermediaryMesh::from_kmd(&kmd.meshes[*i], room_name.to_owned());
                mesh = mesh.join(&joiner);
            }
        } else {
            for kmd_mesh in &kmd.meshes {
                let joiner = IntermediaryMesh::from_kmd(kmd_mesh, room_name.to_owned());
                mesh = mesh.join(&joiner);
            }
        };
        for surf in mesh.surfaces.values_mut() {
            process_surf_material(surf, &room_textures, &room_texture_names, &room_temp_path);
            surf.dedupe();
        }

        let mut coll_mesh = IntermediaryMesh::empty();
        if let Some(curated) = obj_info.coll_meshes_filter {
            for i in curated {
                let joiner = IntermediaryMesh::from_kmd(&kmd.meshes[*i], room_name.to_owned());
                coll_mesh = coll_mesh.join(&joiner);
            }
        } else {
            for kmd_mesh in &kmd.meshes {
                let joiner = IntermediaryMesh::from_kmd(kmd_mesh, room_name.to_owned());
                coll_mesh = coll_mesh.join(&joiner);
            }
        };
        let coll = coll_mesh.to_coll();

        let def = match obj_info.generate_prop {
            GenerateProp::Dont => TempPropDefEnum::None,
            GenerateProp::Static => TempPropDefEnum::Static(PropStaticDef {
                id: format!("mgsimport.{name}"),
                name: name.clone(),
                description: obj_info.desc.to_owned(),
                model_path: format!("mgsimport/models/{name}.glb"),
                coll_path: format!("mgsimport/models/{name}.coll"),
            }),
            GenerateProp::StaticBrk { dstr } => TempPropDefEnum::StaticBrk(PropStaticBrkDef {
                id: format!("mgsimport.{name}"),
                name: name.clone(),
                description: obj_info.desc.to_owned(),
                model_path: format!("mgsimport/models/{name}.glb"),
                coll_path: format!("mgsimport/models/{name}.coll"),
                model_destroy_path: format!("mgsimport/models/{dstr}.glb"),
                coll_destroy_path: format!("mgsimport/models/{dstr}.coll"),
            }),
        };

        loose_models.push(TempLooseModelData {
            name,
            mesh,
            coll,
            prop_def: def,
        })
    }

    TempRoomData {
        static_mesh,
        static_coll_mesh,
        objects: loose_models,
    }
}

fn file_from_zip(zip: &mut ZipArchive<File>, name: &str) -> Vec<u8> {
    let mut file = zip.by_name(name).unwrap();
    let mut bytes = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut bytes).unwrap();
    bytes
}

pub fn get_packed_textures(archive: &Darchive) -> Result<HashMap<String, DynamicImage>, MgsError> {
    let mut hashmap = HashMap::new();

    for file in archive.files() {
        let pcx_filename = file.name();
        if let Some(stem) = pcx_filename.strip_suffix(".pcx") {
            let img = process_pcx(file.contents());
            hashmap.insert(stem.to_owned(), img);
        }
    }

    Ok(hashmap)
}

fn process_surf_material(
    surf: &mut IntermediarySurf,
    room_textures: &HashMap<String, DynamicImage>,
    room_texture_names: &HashMap<u16, String>,
    tex_save_dir: &Path,
) {
    let IntermediaryMatData::NotFound(hash) = &surf.material else {
        unreachable!()
    };
    let Some(stem) = room_texture_names.get(hash).map(|n| n.as_str()) else {
        return;
    };
    let img = room_textures.get(stem).unwrap();

    let png_path = tex_save_dir.join(stem).with_extension("png");
    if !png_path.exists() {
        img.save(png_path).unwrap();
    }

    let transparency = if stem.ends_with("hlf") {
        MatTranspMode::Half
    } else if stem.ends_with("add") {
        MatTranspMode::Additive
    } else if stem.ends_with("msk") {
        MatTranspMode::Mask
    } else {
        MatTranspMode::Opaque
    };
    let double_sided = match transparency {
        MatTranspMode::Half | MatTranspMode::Additive => true,
        MatTranspMode::Mask | MatTranspMode::Opaque => false,
    }; // TODO: maybe
    surf.material = IntermediaryMatData::Texture(IntermediaryTexInfo {
        stem: stem.to_string(),
        path: tex_save_dir.join(stem).with_extension("pcx"),
        double_sided,
        transparency,
    });
}

fn place_texture(surf: &IntermediarySurf, out_dir: &Path) {
    let IntermediaryMatData::Texture(tex_info) = &surf.material else {
        return;
    };
    let temp_png_path = tex_info.path.with_extension("png");
    let final_png_path = out_dir.join(&tex_info.stem).with_extension("png");
    if final_png_path.exists() {
        return;
    }
    std::fs::copy(temp_png_path, final_png_path).unwrap();
}

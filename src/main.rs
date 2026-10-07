mod dump;
mod error;
mod gobox_types;
mod intermediary_mesh;
mod mgs_types;
mod stage_info;
mod util;

use crate::dump::dump_assets;
use crate::error::MgsError;
use crate::intermediary_mesh::IntermediaryMatData;
use crate::intermediary_mesh::IntermediaryTexInfo;
use crate::intermediary_mesh::MatTranspMode;
use crate::mgs_types::Darchive;
use crate::mgs_types::Kmd;
use crate::stage_info::*;
use crate::util::*;
use clap::Parser;
use clap::ValueEnum;
use glam::Vec2;
use glam::Vec3;
use image::DynamicImage;
use intermediary_mesh::IntermediaryMesh;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use zip::ZipArchive;

#[derive(Debug, ValueEnum, Clone, Copy)]
enum ExportFormat {
    Glb,
    Obj,
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    game_dir: PathBuf,

    #[arg(short, long)]
    out_dir: Option<PathBuf>,

    #[arg(short = 'f', long)]
    format: Option<ExportFormat>,
}

fn main() {
    image_extras::register();
    let args = Args::parse();

    let game_dir = args.game_dir;
    if !game_dir.is_dir() {
        println!("no such directory: {:?}", game_dir);
        return;
    }

    let out_path = args.out_dir.unwrap_or(
        game_dir
            .parent()
            .unwrap()
            .join((game_dir.file_name().unwrap().to_string_lossy() + "_extracted").to_string()),
    );

    if let Some(format) = args.format {
        dump_assets(format, &game_dir, &out_path);
    } else {
        gobox_import(&game_dir, &out_path);
    }
}

fn gobox_import(game_dir: &Path, out_path: &Path) {
    let _ = std::fs::remove_dir_all(out_path);
    let mut stage_zip = ZipArchive::new(File::open(game_dir.join("stage.mgz")).unwrap()).unwrap();

    let out_temp_path = PathBuf::from(&out_path).join("temp");
    let out_stage_path = PathBuf::from(&out_path).join("stages").join("shadmo");

    std::fs::create_dir_all(out_path).unwrap();
    std::fs::create_dir_all(&out_temp_path).unwrap();
    std::fs::create_dir_all(&out_stage_path).unwrap();

    File::create(out_path.join(".gdignore")).unwrap();
    File::create(out_path.join("manifest.json"))
        .unwrap()
        .write_all(include_str!("../extra-data/manifest.json").as_bytes())
        .unwrap();

    File::create(out_stage_path.join("manifest.json"))
        .unwrap()
        .write_all(include_str!("../extra-data/stages/shadmo/manifest.json").as_bytes())
        .unwrap();

    let mut intermediary = IntermediaryMesh::empty();
    for room_info in ROOMS {
        // ROOMS[ROOMS.len() - 5..].iter() {
        let room_mesh = get_a_room(&mut stage_zip, room_info, &out_temp_path);
        intermediary = intermediary.join(&room_mesh);
    }

    for surf in intermediary.surfaces.values_mut() {
        surf.dedupe();
    }

    let textures_path = out_stage_path.join("textures");
    std::fs::create_dir_all(&textures_path).unwrap();
    for surf in intermediary.surfaces.values_mut() {
        let IntermediaryMatData::Texture(tex_info) = &surf.material else {
            continue;
        };
        std::fs::copy(
            tex_info.path.with_extension("png"),
            textures_path.join(&tex_info.name).with_extension("png"),
        )
        .unwrap();
    }

    let glb_path = out_stage_path.join("stage.glb");
    std::fs::write(&glb_path, intermediary.to_glb()).unwrap();

    let mut intermediary_coll = intermediary.clone();
    for surf in intermediary.surfaces.values_mut() {
        for n in surf.normals.iter_mut() {
            *n = Vec3::ZERO;
        }
        for uv in surf.uvs.iter_mut() {
            *uv = Vec2::ZERO;
        }
    }
    intermediary_coll.collapse_materials();
    let coll = intermediary_coll.to_coll();
    let mut coll_file = File::create(out_stage_path.join("stage.coll")).unwrap();
    coll.serialize(&mut coll_file).unwrap();
    coll_file.flush().unwrap();
}

fn get_a_room(
    stage_zip: &mut ZipArchive<File>,
    room_info: &MgsRoomInfo,
    temp_path: &Path,
) -> IntermediaryMesh {
    let room_name = room_info.name;
    let room_temp_path = temp_path.join("stages").join(room_name);
    std::fs::create_dir_all(&room_temp_path).unwrap();

    let mut room_static_geom = IntermediaryMesh::empty();
    let mut room_textures: HashMap<String, DynamicImage> = HashMap::new();
    let mut room_texture_names = HashMap::new();

    for filename in room_info.tex_paks {
        let filepath = format!("stage/{}/{filename}", room_name);
        let archive = Darchive::read(&file_from_zip(stage_zip, &filepath), &mut 0).unwrap();
        let pak_textures = get_packed_textures(&archive).unwrap();
        for key in pak_textures.keys() {
            assert!(!room_textures.contains_key(key));
        }
        room_textures.extend(pak_textures);
    }

    for stem in room_textures.keys() {
        let hash = file_name_hash(stem);
        assert!(!room_texture_names.contains_key(&hash));
        room_texture_names.insert(hash, stem);
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

    for static_info in room_info.static_geom {
        let kmd = models
            .get(static_info.kmd_filename)
            .expect(static_info.kmd_filename);

        let tex_prefix = room_info.name.to_owned();

        if let Some(curated) = static_info.meshes {
            for i in curated {
                let mut joiner = IntermediaryMesh::from_kmd(&kmd.meshes[*i], tex_prefix.clone());
                joiner.origin += room_info.origin;
                room_static_geom = room_static_geom.join(&joiner);
            }
        } else {
            for mesh in &kmd.meshes {
                let mut joiner = IntermediaryMesh::from_kmd(mesh, tex_prefix.clone());
                joiner.origin += room_info.origin;
                room_static_geom = room_static_geom.join(&joiner);
            }
        };
    }

    for surf in room_static_geom.surfaces.values_mut() {
        let IntermediaryMatData::NotFound(hash) = &surf.material else {
            unreachable!()
        };
        let Some(stem) = room_texture_names.get(hash).map(|n| n.as_str()) else {
            continue;
        };
        let img = room_textures.get(stem).unwrap();

        let png_path = room_temp_path.join(stem).with_extension("png");
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
            name: stem.to_string(),
            path: room_temp_path.join(stem).with_extension("pcx"),
            double_sided,
            transparency,
        });
    }

    if let Some(glb) = room_info.seal_model {
        room_static_geom =
            room_static_geom.join(&IntermediaryMesh::from_seal_glb(glb, room_info.origin));
    }

    room_static_geom
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

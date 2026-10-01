mod error;
mod stage_info;
mod types;
mod util;

use crate::error::MgsError;
use crate::stage_info::*;
use crate::types::Darchive;
use crate::types::Kmd;
use crate::types::KmdMesh;
use clap::Parser;
use clap::ValueEnum;
use glam::Vec3;
use go_box_import_template::gobox_types::Coll;
use image::ImageReader;
use std::collections::HashMap;
use std::collections::HashSet;
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
    format: ExportFormat,
}

fn main() {
    image_extras::register();
    let args = Args::parse();

    let game_dir = args.game_dir;
    if !game_dir.is_dir() {
        println!("no such directory: {:?}", game_dir);
        return;
    }
    let out_dir = args.out_dir.unwrap_or(
        game_dir
            .parent()
            .unwrap()
            .join((game_dir.file_name().unwrap().to_string_lossy() + "_extracted").to_string()),
    );
    let _ = std::fs::remove_dir_all(&out_dir);

    let stage_mgz_path = game_dir.join("stage.mgz");
    let temp_path = PathBuf::from(&out_dir).join("temp");
    let out_stage_path = PathBuf::from(&out_dir).join("stages").join("shadmo");

    let mut stage_zip = ZipArchive::new(File::open(&stage_mgz_path).unwrap()).unwrap();

    std::fs::create_dir_all(&temp_path).unwrap();
    std::fs::create_dir_all(&out_stage_path).unwrap();
    File::create(&out_dir.join(".gdignore")).unwrap();
    File::create(out_stage_path.join("manifest.json"))
        .unwrap()
        .write_all(include_str!("../extra-data/stage/manifest.json").as_bytes())
        .unwrap();
    let mut stage_merged = Vec::new();

    for room in ROOMS.iter() {
        extract_scenes(
            &mut stage_zip,
            room,
            &mut stage_merged,
            &temp_path,
            args.format,
        );
    }
    let coll = generate_coll(&stage_merged);
    let mut coll_file = File::create(&out_stage_path.join("stage.coll")).unwrap();
    coll.serialize(&mut coll_file).unwrap();
    coll_file.flush().unwrap();

    let glb_path = out_stage_path.join("stage.glb");
    std::fs::write(&glb_path, Kmd::to_glb(&stage_merged)).unwrap();
}

fn extract_scenes(
    zip: &mut ZipArchive<File>,
    room: &MgsRoomInfo,
    stage_merged: &mut Vec<KmdMesh>,
    extract_path: &Path,
    format: ExportFormat,
) {
    let mut tex_hashes = HashMap::new();

    let room_extract_path = extract_path.join(room.name);
    std::fs::create_dir_all(&room_extract_path).unwrap();

    for filename in room.tex_paks {
        let filepath = format!("stage/{}/{filename}", room.name);
        let archive = Darchive::read(&file_from_zip(zip, &filepath), &mut 0).unwrap();
        collect_tex_hashes(&archive, &mut tex_hashes);
        extract_textures(&archive, &room_extract_path).unwrap();
    }

    let mut models = HashMap::new();
    for filename in room.mdl_paks {
        let filepath = format!("stage/{}/{filename}", room.name);
        let archive = Darchive::read(&file_from_zip(zip, &filepath), &mut 0).unwrap();

        for file in archive.files() {
            let filename = file.name();
            let extracted_filepath = room_extract_path.join(filename);
            if filename.ends_with(".kmd") {
                assert!(!models.contains_key(&extracted_filepath));
                models.insert(
                    extracted_filepath,
                    Kmd::read(file.contents(), &mut 0).unwrap(),
                );
            }
        }
    }

    let num_textures = tex_hashes.len();
    let mut num_matches = 0;
    let mut checked_hashes = HashSet::new();
    for (file_path, kmd) in &models {
        for model in kmd.meshes() {
            for hash in model.material_ids() {
                if checked_hashes.contains(hash) {
                    continue;
                }
                checked_hashes.insert(*hash);
                if tex_hashes.contains_key(hash) {
                    num_matches += 1;
                }
            }
        }
        match format {
            ExportFormat::Glb => {
                let glb_path = file_path.with_added_extension("glb");
                std::fs::write(&glb_path, Kmd::to_glb(kmd.meshes())).unwrap();
            }
            ExportFormat::Obj => {
                let wavefront_path = file_path.with_added_extension("obj");
                std::fs::write(&wavefront_path, kmd.to_wavefront(&tex_hashes)).unwrap();
            }
        }
    }

    write_mtl(&mut tex_hashes, &room_extract_path.join("mats.mtl")).unwrap();

    println!("matches: {num_matches}/{num_textures}");

    for filename in room.static_models {
        stage_merged.extend(
            models
                .get(&room_extract_path.join(filename))
                .unwrap()
                .meshes()
                .iter()
                .cloned()
                .map(|mut mesh| {
                    mesh.header.pos += room.origin;
                    mesh
                }),
        );
    }

    /*
        let room_combined: Vec<KmdMesh> = room
            .static_models
            .iter()
            .map(|filename| {
                models
                    .get(&room_extract_path.join(filename))
                    .unwrap()
                    .meshes()
            })
            .flatten()
            .cloned()
            .collect();
        let glb_path = room_extract_path
            .join("_room_combined")
            .with_added_extension("glb");
        std::fs::write(&glb_path, Kmd::to_glb(&room_combined)).unwrap();
    */
}

fn file_from_zip(zip: &mut ZipArchive<File>, name: &str) -> Vec<u8> {
    let mut file = zip.by_name(name).unwrap();
    let mut bytes = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut bytes).unwrap();
    bytes
}

fn collect_tex_hashes(archive: &Darchive, tex_hashes: &mut HashMap<u16, String>) {
    for file in archive.files() {
        if let Some(stem) = file.name().strip_suffix(".pcx") {
            let hash = tex_hash(&stem);
            tex_hashes.insert(hash, stem.to_string());
        }
    }
}

fn extract_textures(archive: &Darchive, path: &Path) -> Result<(), MgsError> {
    for file in archive.files() {
        let filename = file.name();
        if filename.ends_with(".pcx") {
            let pcx_path = path.join(filename);
            std::fs::write(&pcx_path, file.contents())?;
            let img = ImageReader::open(&pcx_path)?.decode().unwrap();
            // let img = ImageReader::new(Cursor::new(file.contents()))
            //     .decode()
            //     .unwrap();
            img.save(path.join(filename).with_extension("png")).unwrap();
        }
    }
    Ok(())
}

fn write_mtl(tex_hashes: &mut HashMap<u16, String>, path: &Path) -> Result<(), MgsError> {
    let mut out = String::new();
    for name in tex_hashes.values() {
        out.push_str(&format!("newmtl {name}\n"));
        out.push_str(&format!("map_Kd {name}.png\n"));
    }

    std::fs::write(path, out.as_bytes())?;
    Ok(())
}

fn tex_hash(name: &str) -> u16 {
    let mut v = 0_u16;
    for c in name.chars() {
        v = (v << 5) | (v >> 11);
        v = v.wrapping_add(c as u16);
    }
    v
}

fn generate_coll(meshes: &[KmdMesh]) -> Coll {
    let mut coll = Coll::default();
    for mesh in meshes {
        let pos: Vec3 = mesh.header().pos.into();
        let vertices = mesh.vertices();
        for face in mesh.vertex_faces() {
            let a: Vec3 = vertices[face[0] as usize].into();
            let b: Vec3 = vertices[face[1] as usize].into();
            let c: Vec3 = vertices[face[2] as usize].into();
            let d: Vec3 = vertices[face[3] as usize].into();

            coll.vbuf.extend((a + pos).to_array());
            coll.vbuf.extend((b + pos).to_array());
            coll.vbuf.extend((c + pos).to_array());
            coll.vbuf.extend((c + pos).to_array());
            coll.vbuf.extend((d + pos).to_array());
            coll.vbuf.extend((a + pos).to_array());
            coll.num_tris += 2;
        }
    }
    coll
}

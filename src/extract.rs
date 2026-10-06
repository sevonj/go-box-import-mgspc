use crate::ExportFormat;
use crate::collect_tex_hashes;
use crate::error::MgsError;
use crate::file_from_zip;
use crate::mgs_types::Darchive;
use crate::mgs_types::Kmd;
use crate::stage_info::MgsRoomInfo;
use crate::util::tex_hash;
use image::GenericImageView;
use image::ImageReader;
use image::Rgba;
use image::RgbaImage;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

pub fn extract_room_models(
    zip: &mut ZipArchive<File>,
    room: &MgsRoomInfo,
    extract_path: &Path,
    format: ExportFormat,
) {
    let mut tex_hashes = HashMap::new();

    let room_extract_path = extract_path.join("stages").join(room.name);
    std::fs::create_dir_all(&room_extract_path).unwrap();

    for filename in room.tex_paks {
        let filepath = format!("stage/{}/{filename}", room.name);
        let archive = Darchive::read(&file_from_zip(zip, &filepath), &mut 0).unwrap();
        collect_tex_hashes(&archive, &mut tex_hashes);
        // extract_textures(&archive, &room_extract_path).unwrap();
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
}

pub fn extract_textures(archive: &Darchive, path: &Path) -> Result<(), MgsError> {
    for file in archive.files() {
        let pcx_filename = file.name();
        if let Some(stem) = pcx_filename.strip_suffix(".pcx") {
            let tex_id = tex_hash(&stem);
            let pcx_path = path.join(pcx_filename);
            let png_path = pcx_path.with_extension("png");
            std::fs::write(&pcx_path, file.contents())?;
            let img = ImageReader::open(&pcx_path)?.decode().unwrap();

            let mut has_alpha: bool = false;
            // pure black is transparent
            for (_, _, val) in img.pixels() {
                if val.0[0] == 0 && val.0[1] == 0 && val.0[2] == 0 {
                    has_alpha = true;
                    break;
                }
            }

            if !has_alpha {
                img.save(png_path).unwrap();
                continue;
            }

            let mut rgba: RgbaImage = img.to_rgba8();
            for Rgba([r, g, b, a]) in rgba.pixels_mut() {
                if *r == 0 && *g == 0 && *b == 0 {
                    *a = 0;
                }
            }

            rgba.save(png_path).unwrap();
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

mod error;
mod types;
mod util;

use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

use image::ImageReader;

use crate::error::MgsError;
use crate::types::Darchive;
use crate::types::Kmd;

fn main() {
    image_extras::register();

    let _ = std::fs::remove_dir_all("extracted");

    let extract_path = PathBuf::from("extracted").join("s00a");
    std::fs::create_dir_all(&extract_path).unwrap();

    let stg_tex1 = Darchive::from_file("gamedata/stage/s00a/stg_tex1.dar").unwrap();
    let stg_tex2 = Darchive::from_file("gamedata/stage/s00a/stg_tex2.dar").unwrap();
    let stg_tex3 = Darchive::from_file("gamedata/stage/s00a/stg_tex3.dar").unwrap();

    let mut tex_hashes = HashMap::new();
    collect_tex_hashes(&stg_tex1, &mut tex_hashes);
    collect_tex_hashes(&stg_tex2, &mut tex_hashes);
    collect_tex_hashes(&stg_tex3, &mut tex_hashes);

    extract_textures(&stg_tex1, &extract_path).unwrap();
    extract_textures(&stg_tex2, &extract_path).unwrap();
    extract_textures(&stg_tex3, &extract_path).unwrap();

    let mut models = vec![];
    let stg_mdl1 = Darchive::from_file("gamedata/stage/s00a/stg_mdl1.dar").unwrap();

    for file in stg_mdl1.files() {
        let filename = file.name();
        let extracted_filepath = extract_path.join(filename);
        if filename.ends_with(".kmd") {
            models.push((
                extracted_filepath,
                Kmd::read(file.contents(), &mut 0).unwrap(),
            ));
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
        let wavefront_path = file_path.with_added_extension("obj");
        std::fs::write(&wavefront_path, kmd.to_wavefront(&tex_hashes)).unwrap();

        let glb_path = file_path.with_added_extension("glb");
        std::fs::write(&glb_path, kmd.to_glb()).unwrap();
    }

    write_mtl(&mut tex_hashes, &extract_path.join("mats.mtl")).unwrap();

    println!("matches: {num_matches}/{num_textures}");
}

fn collect_tex_hashes(archive: &Darchive, tex_hashes: &mut HashMap<u16, String>) {
    for file in archive.files() {
        let filename = file.name();
        if filename.ends_with(".pcx") {
            let stem = filename[..filename.len() - 4].to_string();
            let hash = str_code(&stem);
            tex_hashes.insert(hash, stem);
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

fn str_code(name: &str) -> u16 {
    let mut v = 0_u16;
    for c in name.chars() {
        v = (v << 5) | (v >> 11);
        v = v.wrapping_add(c as u16);
    }
    v
}

use crate::ExportFormat;
use crate::error::MgsError;
use crate::mgs_types::Darchive;
use crate::mgs_types::Kmd;
use crate::util::file_name_hash;
use image::GenericImageView;
use image::ImageReader;
use image::Rgba;
use image::RgbaImage;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use zip::ZipArchive;

pub fn dump_assets(format: ExportFormat, game_dir: &Path, out_dir: &Path) {
    let mut stage_zip = ZipArchive::new(File::open(&game_dir.join("stage.mgz")).unwrap()).unwrap();

    if std::fs::exists(&out_dir).unwrap() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    std::fs::create_dir_all(&out_dir).unwrap();

    for i in 0..stage_zip.len() {
        let zipfile = stage_zip.by_index(i).unwrap();
        if zipfile.is_dir() {
            std::fs::create_dir_all(&out_dir.join(zipfile.name())).unwrap();
        }
    }

    for i in 0..stage_zip.len() {
        let mut zipfile = stage_zip.by_index(i).unwrap();
        if zipfile.is_dir() {
            continue;
        }

        let filename = zipfile.name().to_string();

        if filename.ends_with(".dar") {
            let dar_path = out_dir.join(zipfile.name());
            std::fs::create_dir_all(&dar_path).unwrap();

            let mut bytes = vec![0; zipfile.size() as usize];
            zipfile.read_exact(&mut bytes).unwrap();
            let darchive = Darchive::read(&bytes, &mut 0).unwrap();

            for file in darchive.files() {
                let filename = file.name();
                let out_filepath = dar_path.join(&filename);
                std::fs::write(&out_filepath, file.contents()).unwrap();
            }
            continue;
        }

        let out_filepath = out_dir.join(zipfile.name());
        let mut bytes = vec![0; zipfile.size() as usize];
        zipfile.read_exact(&mut bytes).unwrap();
        std::fs::write(&out_filepath, &bytes).unwrap();
    }

    let mut texture_table_table = HashMap::new();

    for pcx_path in glob::glob(&format!("{}/**/*.pcx", out_dir.to_str().unwrap())).unwrap() {
        let pcx_path = pcx_path.unwrap();
        let parent = pcx_path
            .as_path()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        if !texture_table_table.contains_key(&parent) {
            texture_table_table.insert(parent.clone(), HashMap::new());
        }
        let stem = pcx_path.file_stem().unwrap().to_str().unwrap();
        let hash = file_name_hash(stem);
        let texture_table = texture_table_table.get_mut(&parent).unwrap();
        assert!(!texture_table.contains_key(&hash));
        texture_table.insert(hash, stem.to_string());
        let convert_dir = parent.join("converted_models");
        std::fs::create_dir_all(&convert_dir).unwrap();
        let png_path = convert_dir.join(stem).with_extension("png");

        convert_pcx(&pcx_path, &png_path);
    }

    for kmd_path in glob::glob(&format!("{}/**/*.kmd", out_dir.to_str().unwrap())).unwrap() {
        let kmd_path = kmd_path.unwrap();
        let parent = kmd_path
            .as_path()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let convert_dir = parent.join("converted_models");
        std::fs::create_dir_all(&convert_dir).unwrap();

        if let Some(texture_hashes) = texture_table_table.get(&parent) {
            convert_kmd(kmd_path.as_path(), texture_hashes, format, &convert_dir);
        } else {
            println!("Couldn't find texture hashmap for {parent:?}");
            convert_kmd(kmd_path.as_path(), &HashMap::new(), format, &convert_dir);
        }
    }
}

fn convert_pcx(pcx_path: &Path, png_path: &Path) {
    let rgb = ImageReader::open(pcx_path).unwrap().decode().unwrap();

    let mut has_alpha: bool = false;
    // pure black is transparent
    for (_, _, val) in rgb.pixels() {
        if val.0[0] == 0 && val.0[1] == 0 && val.0[2] == 0 {
            has_alpha = true;
            break;
        }
    }

    if !has_alpha {
        rgb.save(png_path).unwrap();
        return;
    }

    let mut rgba: RgbaImage = rgb.to_rgba8();
    for Rgba([r, g, b, a]) in rgba.pixels_mut() {
        if *r == 0 && *g == 0 && *b == 0 {
            *a = 0;
        }
    }

    rgba.save(png_path).unwrap();
}

fn convert_kmd(
    kmd_path: &Path,
    texture_hashes: &HashMap<u16, String>,
    format: ExportFormat,
    out_dir: &Path,
) {
    let kmd = Kmd::read(&std::fs::read(kmd_path).unwrap(), &mut 0).unwrap();

    let stem = kmd_path.file_stem().unwrap().to_str().unwrap();

    match format {
        ExportFormat::Glb => {
            let glb_path = out_dir.join(stem).with_extension("glb");
            std::fs::write(&glb_path, Kmd::to_glb(&kmd.meshes)).unwrap();
        }
        ExportFormat::Obj => {
            let obj_path = out_dir.join(stem).with_extension("obj");
            let mtl_path = obj_path.with_file_name("extracted_materials.mtl");
            std::fs::write(&obj_path, kmd.to_wavefront(&texture_hashes)).unwrap();

            let mut mtl_contents = String::new();
            for tex_stem in texture_hashes.values() {
                mtl_contents.push_str(&format!("newmtl {tex_stem}\n"));
                mtl_contents.push_str(&format!("map_Kd {tex_stem}.png\n"));
            }

            std::fs::write(mtl_path, mtl_contents.as_bytes()).unwrap();
        }
    }
}

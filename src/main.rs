mod dump;
mod error;
mod gobox_import;
mod gobox_types;
mod intermediary_mesh;
mod mgs_types;
mod stage_info;
mod util;

use crate::dump::dump_assets;
use crate::gobox_import::gobox_import;
use clap::Parser;
use clap::ValueEnum;

use std::path::PathBuf;

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

mod files_map;
mod logger;
mod ui;

use std::env;
use log::LevelFilter;
use log::{info, error, debug};

fn print_usage() {
    info!("unique-picture path_to_dir_source1 [path_to_dir_source2] path_to_dir_target");
    std::process::exit(-1);
}

fn on_total(total: i32) {
    info!("Total d'éléments {:?}", total);
}

fn on_progress(path: &std::path::PathBuf) {
    info!("On progress path: {:?}", path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let logger_result= logger::init(logger::Type::CONSOLE, LevelFilter::Debug);
        if logger_result.is_err() {
            error!("Cannot instantiate logger");
            std::process::exit(-1);
        }
        if args.len() < 3 {
           print_usage();
        }
        debug!(
            "args: {:?} {:?} last: {:?}",
            args,
            &args[1..args.len() - 1],
            &args[args.len() - 1]
        );
        let mut sources: Vec<String> = Vec::new();
        for i in 1..args.len() - 1 {
            sources.push(args[i].clone());
        }
        on_total(files_map::get_total_files(&sources));
        files_map::start_copy(&args[args.len() - 1], &sources, true, &on_progress, &on_progress);
    } else {
        ui::window_application();
    }
}

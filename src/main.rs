mod files_map;
mod logger;
use std::env;
use log::LevelFilter;
use log::{info, error, debug};

fn print_usage() {
    info!("unique-picture path_to_dir_source1 [path_to_dir_source2] path_to_dir_target");
    std::process::exit(-1);
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
        let mut target = files_map::TargetFileBrowser::new(&args[args.len() - 1]);
        target.init();
        for i in 1..args.len() - 1 {
            let mut source = files_map::SourceFileBrowser::new(&args[i], &target);
            source.init();
        }
    } else {
        println!("Should run as GUI");
    }
}

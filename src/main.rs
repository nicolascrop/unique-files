mod files_map;
mod logger;
use std::env;

fn print_usage() {
    println!("unique-picture path_to_dir_source1 [path_to_dir_source2] path_to_dir_target");
    std::process::exit(-1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        logger::init(logger::Type::CONSOLE);
        if args.len() < 3 {
           print_usage();
        }
        println!(
            "args: {:?} {:?} last: {:?}",
            args,
            &args[1..args.len() - 1],
            &args[args.len() - 1]
        );
        let target = files_map::FilesMap::new(&args[args.len() - 1]);
        target.browse();
    } else {
        println!("Should run as GUI");
    }
}

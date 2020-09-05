mod logger;
mod parser;
mod zipup;
use clap::clap_app;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "data-niklas")
        (about: "Creates backups and applies ZIP's")
        (@arg backup: --backup "Creates a backup")
        (@arg apply: --apply "Applies a ZIP")
        (@arg INPUT: +required "Select a file")
    ).get_matches();
    
    let backup = matches.is_present("backup");
    let apply = matches.is_present("apply");
}


mod files;
mod logger;
mod parser;
mod zipup;

use anyhow::Result;
use clap::clap_app;
use std::fs::*;
use std::path::PathBuf;
use std::io::BufReader;
use zipup::Zipup;

fn main() -> Result<()>{
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "data-niklas")
        (about: "Creates backups and applies ZIP's")
        (@arg backup: --backup "Creates a backup")
        (@arg apply: --apply "Applies a ZIP")
        (@arg file: +required "Select a file")
    )
    .get_matches();

    let isbackup = matches.is_present("backup");
    let isapply = matches.is_present("apply");
    if isbackup && isapply {
        logger::stop("error:", "Can't apply and backup at the same time!");
    } else if !isbackup && !isapply {
        logger::stop("error:", "One of --backup or --apply needs to be selected");
    } else if isbackup {
        backup(matches.value_of("file").unwrap())?;
        logger::success("success:", "tar created");
    } else {
        apply(matches.value_of("file").unwrap())?;
        logger::success("success:", "tar applied");
    }
    Ok(())
}

fn backup(configfile: &str) -> Result<()> {
    let path: PathBuf = files::get_path(configfile)?;
    let reader = BufReader::new(File::open(path.as_path())?);
    let mut zipup = Zipup::new(path);
    zipup = parser::parse(zipup,reader, false)?;
    zipup.create_zip()?;
    Ok(())
}

fn apply(zipfile: &str) -> Result<()> {
    let path: PathBuf = files::get_path(zipfile)?;
    let zipup = Zipup::new(path);
    let archive = tar::Archive::new(File::open(&zipup.options.inputfile)?);
    zipup.apply_zip(archive)?;
    Ok(())
}

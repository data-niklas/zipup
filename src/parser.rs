use std::path::Path;
use super::zipup::Zipup;

pub fn parse(file: &String) -> Zipup{
    Zipup::new(&Path::new(file))
}


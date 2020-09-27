use super::zipup::Zipup;
use std::io::{BufReader, prelude::*};
use std::fs::File;
use anyhow::Result;

pub fn parse(mut zipup: Zipup, buffer: BufReader<File>, apply: bool) -> Result<Zipup>{
    for line in buffer.lines(){
        let line: String = line.unwrap().trim().to_string();
        if line.is_empty() || line.starts_with("//") {continue}
        if line.contains(" = "){
            //Key value pair
            let pair: Vec<&str> = line.splitn(2, " = ").collect();
            let key = pair[0].trim_end().to_string();

            if zipup.options.map.contains_key(&key){//Option
                zipup.options.map.insert(key, pair[1].trim_start().to_string());
            }
            else {//Variable
                let extracted = pair[1].trim_start().to_string();
                let varpath = super::files::resolve_path(extracted, &zipup.options).unwrap_or(pair[1].trim_start().to_string());
                zipup.options.variables.insert(key, varpath);
            }
        }
        else{
            //Must be a file
            if !apply{
                if let Err(e) = zipup.add_file(line.clone()){
                    super::logger::warning(
                        "warning:",
                        &("file ".to_owned() + line.as_str() + " could not be read"),
                    );
                    println!("{}",e);
                }
            }
        }

    }
    Ok(zipup)
}


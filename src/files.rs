use super::zipup::Options;
use anyhow::Result;
use anyhow::*;
use std::env;
use std::path::PathBuf;
use chrono::prelude::*;
use std::collections::HashMap;
use std::hash::{Hash,Hasher};

pub fn get_path(file: &str) -> Result<PathBuf> {
    let path = PathBuf::from(file);
    if path.exists() {
        return Ok(path);
    } else {
        return Err(anyhow!("File does not exist!"));
    }
}

fn substring(string: &String, from: usize, to: usize) -> String {
    string
        .chars()
        .skip(from)
        .take(to - from)
        .collect::<String>()
}

fn replace_var(transformed_file: &mut String, chars: &Vec<char>, oldend: usize, start: usize, index: usize, file: &String, options: &Options) -> bool{
    if (chars[start] == '#' || chars[start] == '$') && index > 0 {
        transformed_file.push_str(substring(&file, oldend, start).as_str());
        let var = substring(&file, start, index);
        if chars[start] == '#' {
            if var == "#DATE"{
                transformed_file.push_str(Local::now().format(options.map.get("date_format").unwrap()).to_string().as_str());
            }
            else {
                let varoption = options.variables.get(&substring(&var, 1, index - start));
                transformed_file.push_str(
                    (match varoption {
                        Some(s) => s,
                        None => &var,
                    })
                    .as_str(),
                );
            }
        } else {
            let varoption = env::var(&substring(&var, 1, index - start));
            transformed_file.push_str(
                (match varoption {
                    Ok(s) => s,
                    Err(e) => var,
                })
                .as_str(),
            );
        }
        return true;
    }
    else {
        return false;
    }
}


pub fn resolve_path(mut file: String, options: &Options) -> Result<String> {
    if file.starts_with("~") {
        file.replace_range(
            0..1,
            dirs::home_dir()
                .unwrap_or(PathBuf::from("~"))
                .to_str()
                .unwrap(),
        );
    } else if file.starts_with("./") {
        let mut tempbuf = PathBuf::from(options.inputfile.clone().parent().unwrap());
        tempbuf.push(file.chars().skip(2).collect::<String>().as_str());
        file = tempbuf.to_str().unwrap().to_string();
    }
    let mut transformed_file = String::with_capacity(file.capacity());
    let mut oldend: usize = 0;
    let mut start: usize = 0;
    let chars: Vec<char> = file.chars().collect();
    for (index, c) in chars.iter().enumerate() {
        match c {
            '_' | '.' | '/' | '$' | '#' => {
                //Definitely not a # or $
                if replace_var(&mut transformed_file, &chars, oldend, start, index, &file, options){
                    oldend = index;
                }
                start = index;
            }

            _ => {}
        }
    }
    if chars[oldend] == '#' || chars[oldend] == '$'{
        replace_var(&mut transformed_file, &chars, oldend, start, chars.len(), &file, options);
    }
    else {
        transformed_file.push_str(file.chars().skip(oldend).collect::<String>().as_str());
    }
    Ok(transformed_file)
}

pub fn generalize_string(path: String, vars: &HashMap<String, String>) -> String{
    let mut path = path.clone();
    let homedir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
    if path.starts_with(&homedir){
        path.replace_range(..homedir.len(), "~");
    }
    for (key, value) in vars.iter(){
        path = path.replace(value, key);
    }
    path
}

pub struct VarFile {
    pub file: String,
    pub path: String,
}

impl VarFile {
    pub fn new(path: String, options: &Options) -> Result<VarFile> {
        let file = resolve_path(path.clone(), options)?;
        Ok(VarFile {
            file: file,
            path: path,
        })
    }
}

impl PartialEq for VarFile {
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file
    }
}
impl Eq for VarFile {}

impl Hash for VarFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file.hash(state);
    }
}
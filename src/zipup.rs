use super::files::*;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use tar::*;
use tempfile::tempfile;

pub struct Zipup {
    pub files: HashSet<VarFile>,
    pub commands: Vec<String>,
    pub options: Options,
}

impl Zipup {
    pub fn new(inputfile: PathBuf) -> Zipup {
        Zipup {
            files: HashSet::new(),
            commands: Vec::new(),
            options: Options::new(inputfile),
        }
    }

    pub fn add_file(&mut self, mut file: String) -> Result<()> {
        file = generalize_string(file, &self.options.variables);
        let home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string();
        if file.starts_with(&home_dir) {
            file.replace_range(0..home_dir.len(), "~");
        }
        let varfile = VarFile::new(file.clone(), &self.options)?;
        let metadata = fs::symlink_metadata(&varfile.file)?;

        //Actually start reading the metadata
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            let mut children = 0;
            for entry in fs::read_dir(Path::new(&varfile.file))? {
                let entry = entry.unwrap();
                let path = entry.path();
                children += 1;
                let mut varpath = PathBuf::from(varfile.path.clone());
                varpath.push(path.file_name().unwrap());
                self.add_file(varpath.to_str().unwrap().to_string())?;
            }
            if children == 0 {
                //New command to create the folder
                self.commands
                    .push("DIR ".to_string() + varfile.path.as_str());
                super::logger::info("added directory", file.as_str());
            }
        } else if file_type.is_symlink() {
            let link_from = fs::read_link(varfile.file.clone())?;
            let link_from = generalize_string(
                link_from.to_str().unwrap().to_string(),
                &self.options.variables,
            );
            self.commands
                .push("LINK ".to_string() + varfile.path.as_str() + " - " + link_from.as_str());

            super::logger::info("added symlink", file.as_str());
            if Path::new(&link_from).is_relative() {
                let mut parent = PathBuf::from(Path::new(&file).parent().unwrap());
                parent.push(link_from);
                self.add_file(parent.to_str().unwrap().to_string())?;
            } else {
                self.add_file(link_from.to_string())?;
            }
        } else {
            if self.files.contains(&varfile) {
                super::logger::warning(
                    "duplicate file",
                    &(file.as_str().to_string() + " will already be added to the archive"),
                );
            } else {
                self.files.insert(varfile);
                super::logger::info("added file", file.as_str());
            }
        }
        Ok(())
    }

    pub fn create_zip(self) -> Result<()> {
        let output = self.options.map.get("output_file").unwrap().clone();
        let output = resolve_path(output, &self.options)?;

        let file = std::fs::File::create(output)?;
        let mut builder = Builder::new(file);

        let mut commandfile = tempfile()?;
        for line in self.commands.iter() {
            commandfile.write(line.as_bytes())?;
            commandfile.write("\n".as_bytes())?;
        }
        commandfile.seek(SeekFrom::Start(0))?;

        builder.append_file(
            "zipup.conf",
            &mut File::open(&self.options.inputfile)?,
        )?;
        builder.append_file("commands.conf", &mut commandfile)?;

        for varfile in self.files.iter() {
            if let Err(e) = builder.append_file(
                varfile.path.clone().replace("/", "%2F"),
                &mut File::open(&varfile.file)?,
            ) {
                super::logger::error(
                    "error:",
                    format!("Could not add file {} to the tar", &varfile.path).as_str(),
                );
            }
        }
        Ok(())
    }

    pub fn apply_zip(mut self, mut archive: Archive<File>) -> Result<()> {
        //First entry should be the zipup.conf file!

        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            let filename = path.to_str().unwrap().to_string();
            if filename == "commands.conf" {
                let reader = BufReader::new(entry);
                for line in reader.lines() {
                    self.commands.push(line?);
                }
            } else if filename == "zipup.conf" {
                let mut conffile = PathBuf::from(self.options.inputfile.clone().parent().unwrap());
                conffile.push("zipup.conf");
                entry.unpack(&conffile)?;
                self =
                    super::parser::parse(self, BufReader::new(File::open(conffile)?), true)
                        .unwrap();
            } else {
                let varfile = VarFile::new(filename.replace("%2F", "/"), &self.options)?;
                println!("{} {}", &varfile.file, &varfile.path);
                let parent = Path::new(&varfile.file).parent().unwrap();
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
                entry.unpack(varfile.file)?;
            }
        }

        for command in self.commands {
            if command.starts_with("DIR ") {
                let dir = command.chars().skip(4).collect::<String>();
                let resolveddir = resolve_path(dir, &self.options)?;
                let dirpath = Path::new(&resolveddir);
                if !dirpath.exists() {
                    fs::create_dir_all(dirpath)?;
                }
            } else if command.starts_with("LINK ") {
                let links = command.chars().skip(5).collect::<String>();
                let parts: Vec<&str> = links.split(" - ").collect();
                let from = resolve_path(parts[1].to_string(), &self.options)?;
                let to = resolve_path(parts[0].to_string(), &self.options)?;
                if !Path::new(&to).exists() {
                    std::os::unix::fs::symlink(from, to)?;
                }
            }
        }
        Ok(())
    }
}

pub struct Options {
    pub inputfile: PathBuf,
    pub map: HashMap<String, String>,
    pub variables: HashMap<String, String>,
}

impl Options {
    pub fn new(inputfile: PathBuf) -> Options {
        Options {
            inputfile: inputfile,
            map: Options::default_map(),
            variables: HashMap::new(),
        }
    }

    fn default_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("output_file".to_string(), "./zipup_#DATE.tar".to_string());
        map.insert("date_format".to_string(), "%Y-%m-%d_%H:%M:%S".to_string());
        map
    }
}

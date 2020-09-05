use std::vec::Vec;
use std::path::Path;
use std::collections::HashMap;

//Vars:
//#VAR Text replacement
//$VAR System ENV Var
//Can be terminated with a _ or / or $
pub struct Zipup<'b>{
    files: Vec<String>,
    options: Options<'b>,
}

impl<'b> Zipup<'b>{

    pub fn new(configfile: &'b Path) -> Zipup<'b>{
        Zipup{
            files: Vec::new(),
            options: Options::new(configfile)
        }
    }

    pub fn add_file(mut file: String){
        if file.chars().next().unwrap() == '~'{
            file.remove(0);
            file.insert_str(0,"$HOME");
        }
        else if file.starts_with("../"){
            
        }
        else if file.starts_with("./"){

        }

    }

}

pub struct Options<'a>{
    configfile: &'a Path,
    map: HashMap<String, String>,
    variables: HashMap<String, String>,
}

impl<'a> Options<'a>{
    pub fn new(configfile: &'a Path) -> Options<'a>{
        Options{
            configfile: configfile,
            map: Options::default_map(),
            variables: HashMap::new(),
        }
    }

    fn default_map() -> HashMap<String, String>{
        let mut map = HashMap::new();
        map.insert(String::from("project_relative"), String::from("false"));
        map.insert(String::from("backup_before_apply"), String::from("false"));
        map.insert(String::from("output_file"), String::from("./zipup_$DATE.zip"));
        map
    }
}

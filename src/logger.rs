use colored::*;

pub fn success(title: &str, message: &str){
    println!("{} {}", title.green().bold(), message);
}

pub fn warning(title: &str, message: &str){
    println!("{} {}", title.yellow().bold(), message);
}

pub fn error(title: &str, message: &str){
    println!("{} {}", title.red().bold(), message);
}

pub fn info(title: &str, message: &str){
    println!("{} {}", title.bold(), message);
}

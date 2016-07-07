extern crate yaml_rust;

use std::fs::File;
use std::io;

use std::io::prelude::*;
use std::process; 

use self::yaml_rust::{YamlLoader};

#[derive(Debug)]
pub struct Config {
    // dirs content is String so Config owns its string data
    pub dirs: Vec<String>,
}

pub fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(filename));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

pub fn new_config(path: &str) -> Config {  
    match read_file(path) {
        Ok(s) => {
            let parsed = YamlLoader::load_from_str(&s).unwrap();
            let dirs = parsed[0]["dirs"].as_vec().unwrap(); 
            let mut v1 = Vec::new();
            for item in dirs {
                match item.as_str() {
                    Some(x) => v1.push(x.to_string()),
                    _ => ()
                }
            }
            let conf = Config{dirs: v1};
            return conf;
        }
        Err(error) => {
            println!("{:?}", error);
            process::exit(1);  
        }
    }
}


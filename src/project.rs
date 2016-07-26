use std::error::Error;
use std::ffi::OsString;
use std::fs::{read_dir, DirEntry};
use std::io::prelude::*;

pub type ProjectError = String;
pub type Proc<T> = Result<T, ProjectError>;

pub fn discover(dir: &str) -> Proc<Vec<GitRepo>> {
    match read_dir(dir) {
        Err(error) => Err(error.description().to_string()),
        Ok(files) => {
            for file_result in files {
                match file_result {
                    Ok(file) => {
                        let x = try!(git_repo(file));
                        println!("Found path: {:?}", x);
                    }
                    Err(err) => {
                        return Err(err.description().to_string());
                    }
                }
            }
            Err("Finished work".to_string())
        },
    }
}

// TODO: Better parameter type for representing files?
pub fn git_repo(root: DirEntry) -> Proc<GitRepo> {
    match read_dir(root.path()) {
        Err(error) =>
            Err(error.description().to_string()),
        Ok(files) => {
            let v: Vec<GitRepo> = files.filter_map(|f| {
                f.ok().and_then(|file| {
                    file.metadata().ok().and_then(|fd| {
                        if fd.is_dir() { 
                            Some(GitRepo {
                                name: "GitRepo name".to_string(),
                                root: file.path().into_os_string(),
                            })
                        }
                        else {
                            None
                        }
                    })
                })
            }).collect();
            println!("Debug shit: {:?}", v);
            Err("Found shit".to_string())
        },
    }
}

#[derive(Debug)]
pub struct GitRepo {
    name: String,
    root: OsString,
}

// impl GitRepo for Project<GitRepo> {
//     fn get_metrics(&self) -> Vec<MetricInfo> {
//         MetricInfo {
//             name: self.name,
//         }
//     }
// }

// trait Project<T> {
//     fn get_metrics() -> Vec<MetricInfo>;
// }

// struct MetricInfo {
//     name: String;
// }

// trait Calc<In, Out> {

//     fn calc(in: In) -> Out;
// }

// struct Loc {
//     lines: i64;
// }

// impl LocOfFile for Metric<DirEntry, Loc> {
//     fn calc(in: DirEntry) -> Loc {
//         Loc {
//             lines: 100;
//         }
//     }
// }

// impl GitRepo for Project<>
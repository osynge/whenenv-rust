use clap::{ArgMatches};
use std::path::Path;
use db;
use std::fs::File;
use std::io::Read;
use rustc_serialize;
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::result::Result;
use rusqlite::Connection;
use rusqlite::Error;

use dbFsFile;
use std::result;
use json_loader_elephant::json_loader_elephant;
use elephant;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}




pub fn listy(direcory: &str) {
    let path = Path::new(direcory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("aaaa{:?}", entry.path());
        }
    }
}


pub fn job_files_list(direcory: &str)  {
    let path = Path::new(direcory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("job_files_list{:?}", entry.path());
        }
    }
}



pub fn deligate(conn: &Connection, matches : &ArgMatches) {
    if let Some(in_v) = matches.values_of("dir-scripts") {
        let str_shell_files_list = String::from("shell_files");
        let pk_directory_type_shell = elephant::elephant_directory_type(&conn, &str_shell_files_list);
        for in_file in in_v {
            let filename = in_file.to_string();
            db::insert_fs_dir(&conn, pk_directory_type_shell, filename);
        }
    }
    if let Some(in_v) = matches.values_of("dir-jobs") {
        let str_job_files_list = String::from("job_files");
        let pk_directory_type_jobs = elephant::elephant_directory_type(&conn, &str_job_files_list);
        for in_file in in_v {
            let filename = in_file.to_string();
            db::insert_fs_dir(&conn, pk_directory_type_jobs, filename);
        }
    }
}


pub fn enviroment(conn: &Connection, pk_session : i32, matches : &ArgMatches) {
    if let Some(in_v) = matches.values_of("env") {
        for in_file in in_v {
            let filename = in_file.to_string();
            let mut text = "".to_string();
            let pk_variable_name = elephant::elephant_variable_pk(&conn, &filename);
            let session = elephant::elephant_enviroment(&conn, &pk_session, &pk_variable_name);
        }
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn list_provider() {
        use db;
        let conn = db::connect();
        db::create_tables(&conn);
        for filename in db::list_provider(&conn) {
            println!("list_provider:{:?}", filename);
        }

    }

    #[test]
    fn list_job_depend() {
        use db;
        let conn = db::connect();
        db::create_tables(&conn);
        for filename in db::list_job_depend(&conn) {
            println!("list_job_depend:{:?}", filename);
        }
    }
    #[test]
    fn list_variable_name() {
        use db;
        let conn = db::connect();
        db::create_tables(&conn);
        for filename in db::list_variable_name(&conn) {
            println!("list_variable_name:{:?}", filename);
        }
    }

}

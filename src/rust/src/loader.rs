use clap::ArgMatches;
use std::path::Path;
use log;
use db;
use std::fs::File;
use std::io::Read;
use rustc_serialize;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::result::Result;
use rusqlite::Connection;
use rusqlite::Error;
use std::env;

use dbFsFile;
use std::result;
use json_loader_elephant::json_loader_elephant;
use elephant;
use std::collections::HashSet;
use autoconf;

pub fn actions_get(matches: &ArgMatches) -> HashSet<String> {
    let mut vec_actions = HashSet::<String>::new();

    if let Some(in_v) = matches.values_of("list-provides") {
        let bill = String::from("load-jobs");
        vec_actions.insert(bill);
    }
    if let Some(in_v) = matches.values_of("session") {
        let bill = String::from("load-jobs");
        vec_actions.insert(bill);
        let str_load_scripts = String::from("load-scripts");
        vec_actions.insert(str_load_scripts);
    }
    return vec_actions;
}





pub fn listy(conn: &Connection, pk_directory: &i32, direcory: &str) -> Result<i32, &'static str> {
    let path = Path::new(direcory);
    if path.exists() == false {
        return Err("File not found");
    }
    if path.is_dir() == false {
        return Err("File not found");
    }
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path_result = entry.path();
            let path_string = path_result.to_str();
            match path_string {
                Some(pk) => {
                    elephant::elephant_file(&conn, pk_directory, pk);
                }
                None => {}
            }
        }
    }
    return Ok(0);
}


pub fn job_files_list(direcory: &str) {
    let path = Path::new(direcory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            debug!("job_files_list{:?}", entry.path());
        }
    }
}



pub fn deligate(conn: &Connection, matches: &ArgMatches) {
    let actions = actions_get(matches);
    let matcher = String::from("load-jobs");
    if actions.contains(&matcher) {
        if let Some(in_v) = matches.values_of("dir-jobs") {
            let str_job_files_list = String::from("job_files");
            let result_dir_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
            match result_dir_type {
                Ok(pk_directory_type_jobs) => {
                    for in_dir in in_v {
                        let dirname = in_dir.to_string();
                        let pk_directory =
                            elephant::elephant_directory(&conn, &pk_directory_type_jobs, &dirname);
                        listy(&conn, &pk_directory, &dirname);
                    }
                }
                Err(_) => {}
            }
        }
    } else {
        let str_job_files_list = String::from(autoconf::jobdir());
        let result_dir_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
        let pk_directory_type_jobs = result_dir_type.unwrap();
        let pk_directory =
            elephant::elephant_directory(&conn, &pk_directory_type_jobs, &str_job_files_list);
        listy(&conn, &pk_directory, &str_job_files_list);
    }
    let str_load_scripts = String::from("load-scripts");
    if actions.contains(&str_load_scripts) {
        let str_shell_files_list = String::from("shell_files");
        let result_dir_type = elephant::elephant_directory_type(&conn, &str_shell_files_list);
        let pk_directory_type_shell = result_dir_type.unwrap();
        if let Some(in_v) = matches.values_of("dir-scripts") {
            let str_shell_files_list = String::from("shell_files");
            let result_dir_type = elephant::elephant_directory_type(&conn, &str_shell_files_list);
            match result_dir_type {
                Ok(pk_directory_type_shell) => {
                    for in_dir in in_v {
                        let dirname = in_dir.to_string();
                        let pk_directory =
                            elephant::elephant_directory(&conn, &pk_directory_type_shell, &dirname);
                        listy(&conn, &pk_directory, &dirname);
                    }
                }

                Err(_) => {}
            }



        } else {
            let dirname = String::from(autoconf::shelldir());
            let pk_directory =
                elephant::elephant_directory(&conn, &pk_directory_type_shell, &dirname);
            listy(&conn, &pk_directory, &dirname);
        }
        if let Some(in_v) = matches.values_of("dir-py") {
            let str_py_files_list = String::from("python_files");
            let result_dir_type = elephant::elephant_directory_type(&conn, &str_py_files_list);
            let pk_directory_type_py = result_dir_type.unwrap();
            for in_dir in in_v {
                let dirname = in_dir.to_string();
                let pk_directory =
                    elephant::elephant_directory(&conn, &pk_directory_type_py, &dirname);
                listy(&conn, &pk_directory, &dirname);
            }
        }
    }
}


pub fn enviroment(conn: &Connection, pk_session: i32, matches: &ArgMatches) {
    if let Some(in_v) = matches.values_of("env") {
        for enviroment_variable in in_v {
            let env_var = enviroment_variable.to_string();
            let result_elephant_variable = elephant::elephant_variable_pk(&conn, &env_var);
            match result_elephant_variable {
                Ok(pk_variable_name) => {
                    let mut value = String::from("");
                    match env::var(env_var) {
                        Ok(lang) => value = String::from(lang),
                        Err(e) => {
                            error!(
                                "Couldn't read Enviroment variable: ({})",
                                enviroment_variable.to_string()
                            );
                            continue;
                        }
                    };
                    let result_elephant_variable_value =
                        elephant::elephant_variable_pair_pk(&conn, &pk_variable_name, &value);
                    match result_elephant_variable_value {
                        Ok(pk_variable_pair) => {
                            debug!("pk_session:{:?}", pk_session);
                            debug!("pk_variable_pair:{:?}", pk_variable_pair);
                            let result_elephant_enviroment = elephant::elephant_enviroment(
                                &conn,
                                &pk_session,
                                &pk_variable_pair,
                            );
                            match result_elephant_enviroment {
                                Ok(pk_variable_pair) => {}
                                Err(_) => {}
                            }
                        }
                        Err(_) => {}

                    }
                }
                Err(_) => {}
            }
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
            debug!("list_provider:{:?}", filename);
        }
    }
    #[test]
    fn list_job_depend() {
        use db;
        let conn = db::connect();
        db::create_tables(&conn);
        for filename in db::list_job_depend(&conn) {
            debug!("list_job_depend:{:?}", filename);
        }
    }
    #[test]
    fn list_variable_name() {
        use db;
        let conn = db::connect();
        db::create_tables(&conn);
        for filename in db::list_variable_name(&conn) {
            debug!("list_variable_name:{:?}", filename);
        }
    }

}

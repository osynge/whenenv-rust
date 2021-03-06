use autoconf;
use cfg;
use clap::ArgMatches;
use db;
use elephant;
use rusqlite::Connection;
use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::result::Result;
use std::sync::Arc;
use std::sync::Mutex;

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

pub fn connect_deligate(matches: &ArgMatches) -> Connection {
    if let Some(in_v) = matches.values_of("rdbms") {
        for enviroment_variable in in_v {
            let env_var = enviroment_variable.to_string();
            debug!("connect to sqllite:{:?}", env_var);
            return db::connect_file(&env_var);
        }
    }
    return db::connect();
}

pub fn deligate(conn: &Connection, cfg_rt_arc_mu: &Arc<Mutex<cfg::Config>>) {
    let cfg_rt = cfg_rt_arc_mu.lock().unwrap();
    if cfg_rt.actions.contains(&cfg::Action::LoadJobs) {
        let str_job_files_list = String::from("job_files");
        let result_dir_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
        match result_dir_type {
            Ok(pk_directory_type_jobs) => for in_dir in cfg_rt.path_jobs.iter() {
                let dirname = in_dir.to_string();
                let pk_directory =
                    elephant::elephant_directory(&conn, &pk_directory_type_jobs, &dirname);
                let _ = listy(&conn, &pk_directory, &dirname);
            },
            Err(_) => {}
        }
    }
    if cfg_rt.actions.contains(&cfg::Action::LoadScripts) {
        let str_shell_files_list = String::from("shell_files");
        let result_dir_type = elephant::elephant_directory_type(&conn, &str_shell_files_list);
        match result_dir_type {
            Ok(pk_directory_type_shell) => for in_dir in cfg_rt.path_shell.iter() {
                let dirname = in_dir.to_string();
                let pk_directory =
                    elephant::elephant_directory(&conn, &pk_directory_type_shell, &dirname);
                let _ = listy(&conn, &pk_directory, &dirname);
            },
            Err(_) => {}
        }
    }
    if cfg_rt.actions.contains(&cfg::Action::LoadScripts) {
        let str_py_files_list = String::from("python_files");
        let result_dir_type = elephant::elephant_directory_type(&conn, &str_py_files_list);
        let pk_directory_type_py = result_dir_type.unwrap();
        for in_dir in cfg_rt.path_python.iter() {
            let dirname = in_dir.to_string();
            let pk_directory = elephant::elephant_directory(&conn, &pk_directory_type_py, &dirname);
            let _ = listy(&conn, &pk_directory, &dirname);
        }
    }
}

pub fn enviroment(conn: &Connection, cfg_rt_arc_mu: &Arc<Mutex<cfg::Config>>, pk_session: i32) {
    let cfg_rt = cfg_rt_arc_mu.lock().unwrap();
    for enviroment_variable in cfg_rt.enviroment.iter() {
        let env_var = enviroment_variable.to_string();
        let result_elephant_variable = elephant::elephant_variable_pk(&conn, &env_var);
        match result_elephant_variable {
            Ok(pk_variable_name) => {
                let value: String;
                match env::var(env_var) {
                    Ok(lang) => value = String::from(lang),
                    Err(_) => {
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
                        let result_elephant_enviroment =
                            elephant::elephant_enviroment(&conn, &pk_session, &pk_variable_pair);
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

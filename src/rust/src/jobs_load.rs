use db;
use elephant;
use json_loader_elephant::json_loader_elephant;
use json_loader_optional;
use rusqlite::Connection;
use rustc_serialize::json::Json;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn list_dir(direcory: &str, items: &mut Vec<String>) -> Result<(), &'static str> {
    let path = Path::new(direcory);
    if path.exists() == false {
        return Err("Path does not exist");
    }
    if path.is_dir() == false {
        return Err("Is not a directory");
    }
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let pb_2 = path.as_path();
            let as_path_buf = pb_2.as_os_str();
            let path = String::new() + as_path_buf.to_str().unwrap();
            items.push(path);
        }
    }
    Ok(())
}

pub fn loader(name: &str) -> String {
    let mut f = File::open(name).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    return contents;
}

pub fn json_loader_name(conn: &Connection, pk_file: &i32, content: &str) {
    let json = Json::from_str(&content);
    match json {
        Ok(json) => {
            json_loader_elephant(conn, &pk_file, &json);
            //json_loader_elephant_deps_depth1(conn, &pk_file, &json);
        }
        Err(_) => {}
    }
}

fn jdl_loader(conn: &Connection, pk_file: &i32, jdl_item: &json_loader_optional::DeserializeJdl) {
    //dbg!(jdl_item);
    let pk_job = elephant::elephant_job_pk(conn, &pk_file, &jdl_item.name);
    for value in &jdl_item.variables.require_keys {
        let variable_name_result = elephant::elephant_variable_pk(conn, &value);

        let pk_variable_name = match variable_name_result {
            Ok(pk_variable_name) => pk_variable_name,
            Err(code) => {
                error!("elephant::elephant_variable_pk:{}", code);
                continue;
            }
        };
        let ejpv_rc = elephant::elephant_job_require_variables(conn, &pk_job, &pk_variable_name);
        match ejpv_rc {
            Ok(pk_variable_name) => {}
            Err(code) => {
                error!("elephant::elephant_job_require_variables:{}", code);
            }
        }
    }
    let mut order_job_depend: i32 = 0;
    for value in &jdl_item.depends {
        let variable_name_result = elephant::elephant_variable_pk(conn, &value);

        let pk_variable_name = match variable_name_result {
            Ok(pk_variable_name) => pk_variable_name,
            Err(code) => {
                error!("elephant::elephant_variable_pk:{}", code);
                continue;
            }
        };
        let ejpv_rc =
            elephant::elephant_job_depend_pk(conn, &pk_job, &pk_variable_name, &order_job_depend);
        order_job_depend += 10;
    }

    let pk_job: i32;
    let mut pk_provider: i32;
    debug!("job_name:{}", jdl_item.name);
    pk_job = elephant::elephant_job_pk(conn, &pk_file, &jdl_item.name);
    debug!("jdl_item.name::pk_job:{}", pk_job);
    for item in &jdl_item.variables.provides_keys {
        debug!("jdl_item.variables.provides_keys:{}", item);
        let variable_name_result = elephant::elephant_variable_pk(conn, &item);
        match variable_name_result {
            Ok(pk_variable_name) => {
                debug!(
                    "elephant::elephant_job_provide_variables::pk_job:{}",
                    pk_job
                );
                debug!(
                    "elephant::elephant_job_provide_variables::pk_variable_name:{}",
                    pk_variable_name
                );
                let ejpv_rc =
                    elephant::elephant_job_require_variables(conn, &pk_job, &pk_variable_name);
                match ejpv_rc {
                    Ok(pk_variable_name) => {}
                    Err(code) => {
                        error!("elephant::elephant_job_require_variables:{}", code);
                    }
                }
            }
            Err(code) => {
                error!("elephant::elephant_variable_pk:{}", code);
            }
        }
    }
    for item in &jdl_item.variables.require_values {
        let (a, b) = item;
        debug!("jdl_item.variables.require_values:{}", a);

        let result_variable_pair = elephant::elephant_variable_pk(&conn, &a);
        match result_variable_pair {
            Ok(variable_pair_pk) => {
                debug!(
                    "jdl_item.variables.require_values::variable_pair_pk={}",
                    variable_pair_pk
                );
                let job_depend_pair_pk =
                    elephant::elephant_job_depend_pair_pk(&conn, &pk_job, &variable_pair_pk);
                debug!(
                    "jdl_item.variables.require_values::job_depend_pair_pk={}",
                    job_depend_pair_pk
                );
            }
            Err(_) => {}
        }
    }
    for item in &jdl_item.provides {
        //debug!("job_provides:{}", item);
        let result_variable_pk = elephant::elephant_variable_pk(conn, &item);
        let variable_pk;
        match result_variable_pk {
            Ok(pk) => {
                variable_pk = pk;
            }
            Err(code) => {
                error!("job_provides::elephant_variable_pk:{}", code);
                continue;
            }
        }

        // println!("elephant_provider_pk={}", foo);
        // let sq_order = 1;
        // pk_provider = elephant::elephant_job_depend_pk(conn, &pk_job, &pk_provider, &sq_order);
        debug!("job_provides::pk_provider={}", variable_pk);

        let job_provide_variables_rc =
            elephant::elephant_job_provide_variables(&conn, &pk_job, &variable_pk);
    }
    let mut order_job_depend: i32 = 0;
    for item in &jdl_item.depends {
        debug!("job_depends:{}", item);
        let result_variable_pk = elephant::elephant_variable_pk(conn, &item);
        let variable_pk;
        match result_variable_pk {
            Ok(pk) => {
                variable_pk = pk;
            }
            Err(code) => {
                error!("job_depends::elephant_variable_pk:{}", code);
                continue;
            }
        }
        let pk_job_depend =
            elephant::elephant_job_depend_pk(conn, &pk_job, &variable_pk, &order_job_depend);
        debug!("job_depends::pk_job_depend:{}", pk_job_depend);
        order_job_depend += 10;
    }
    for (k, v) in &jdl_item.variables.require_values {
        let variable_name_result = elephant::elephant_variable_pk(conn, &k);
        match variable_name_result {
            Ok(pk_variable_name) => {
                let variable_name_result =
                    elephant::elephant_variable_pair_pk(conn, &pk_variable_name, &v);
                match variable_name_result {
                    Ok(pk_variable_pair) => {
                        let pk_variable_pair_dep =
                            elephant::elephant_job_depend_pair_pk(conn, &pk_job, &pk_variable_pair);
                        //dbg!(pk_variable_pair);
                        //dbg!(pk_variable_pair_dep);
                    }
                    Err(_) => {}
                }
                //dbg!(pk_variable_name);
                //println!("job_requires_vaiable_pair.key:{}", item.key);
                //println!("job_requires_vaiable_pair.value:{}", item.value);
                //println!("job_requires_vaiable_pair.pk_variable_name:{}",pk_variable_name);
                //println!("job_requires_vaiable_pair.pk_variable_pair:{}",pk_variable_pair);
                //println!("job_requires_vaiable_pair.pk_variable_pair_dep:{}",pk_variable_pair_dep);
            }
            Err(_) => {}
        }
    }

    //println!("{:?}", value);
    //let variable_name_result = elephant::elephant_variable_pk(conn, &item);
    //elephant::elephant_job_require_variables(conn, &pk_job, &pk_variable_name);
    //let result_variable_pair = elephant::elephant_variable_pk(&conn, &item);
    //elephant::elephant_job_depend_pair_pk(&conn, &pk_job, &variable_pair_pk);
    //let result_variable_pk = elephant::elephant_variable_pk(conn, &item);
    //elephant::elephant_job_provide_variables(&conn, &pk_job, &variable_pk);
    // let result_variable_pk = elephant::elephant_variable_pk(conn, &item);
    // elephant::elephant_job_depend_pk(conn, &pk_job, &variable_pk, &order_job_depend);
    //let variable_name_result = elephant::elephant_variable_pk(conn, &item.key);
    //elephant::elephant_variable_pair_pk(conn, &pk_variable_name, &item.value);
    //let pk_variable_pair_dep = elephant::elephant_job_depend_pair_pk
    //pub fn json_loader_elephant(conn: &Connection, pk_file: &i32, json: &rustc_serialize::json::Json)
}

pub fn json_loader_name2(conn: &Connection, pk_file: &i32, content: &str) {
    let request = serde_json::from_str(&content);
    match request {
        Ok(j) => {
            jdl_loader(conn, pk_file, &j);
        }
        Err(c) => println!("{:?}", c),
    }
}

pub fn load(conn: &Connection) {
    for directory in db::list_fs_dir(&conn) {
        let foo = directory.id;
        let foo_name = directory.name;
        let mut dir_content = vec![];
        match list_dir(&foo_name, &mut dir_content) {
            Ok(_) => {}
            Err(error) => continue,
        }
        for fnccc in dir_content {
            //println!("An input file: {}", fnccc);
            let s1 = fnccc.clone();
            let _ = db::insert_fs_file(&conn, foo, fnccc);
            let mut pk = 10;
            db::pk_fs_file_by_name(&conn, s1, &mut pk);
            //println!("ssss: {}", pk);
        }
    }
    let mut scores = HashMap::new();
    let str_job_files_list = String::from("job_files");
    let result_dir_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
    if result_dir_type.is_err() {
        return;
    }

    let pk_directory_type_jobs = result_dir_type.unwrap();
    let mut jdl_content = Vec::<json_loader_optional::DeserializeJdl>::new();
    for filename in db::list_fs_file_type(&conn, &pk_directory_type_jobs) {
        let name = String::from(filename.name);
        let name2 = name.clone();
        let content = loader(name2.trim());
        let content2 = content.clone();
        scores.insert(name, content2);

        let json_from_str = serde_json::from_str(&content);
        match json_from_str {
            Ok(p) => {
                jdl_content.push(p);
                //json_loader_elephant_deps_depth1(conn, &pk_file, &json);
            }
            Err(_) => {}
        }
    }
    // iterate over everything.
    for (filename, contents) in &scores {
        let mut pkfsfile: i32 = 0;
        let filename_str = filename.clone();
        let _ = db::pk_fs_file_by_name(&conn, filename_str, &mut pkfsfile);
        let _ = json_loader_name(&conn, &pkfsfile, &contents);
        let _ = json_loader_name2(&conn, &pkfsfile, &contents);
    }
    db::variable_pair_list(&conn);
}

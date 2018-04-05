use rusqlite::Connection;
use db;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use rustc_serialize::json::Json;
use json_loader_elephant::json_loader_elephant;
use elephant;

pub fn listy2(direcory: &str) -> Vec<String> {
    let mut items = Vec::<String>::new();

    let path = Path::new(direcory);
    if path.exists() == false {
        return items;
    }
    if path.is_dir() == false {
        return items;
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
    return items;
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

pub fn load(conn: &Connection) {
    for directory in db::list_fs_dir(&conn) {
        let foo = directory.id;
        let foo_name = directory.name;
        let file_list = listy2(&foo_name);
        for fnccc in file_list {
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
    for filename in db::list_fs_file_type(&conn, &pk_directory_type_jobs) {
        let name = String::from(filename.name);
        let name2 = name.clone();
        let loader_rc = loader(name2.trim());
        scores.insert(name, loader_rc);
    }

    // iterate over everything.
    for (filename, contents) in &scores {
        let mut pkfsfile: i32 = 0;
        let filename_str = filename.clone();
        let _ = db::pk_fs_file_by_name(&conn, filename_str, &mut pkfsfile);
        let _ = json_loader_name(&conn, &pkfsfile, &contents);
    }
    db::variable_pair_list(&conn);
}

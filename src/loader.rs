use clap::{ArgMatches};
use std::path::Path;
use db;
use std::fs::File;
use std::io::Read;

use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;

use std::collections::HashMap;
use std::result::Result;
use rusqlite::Connection;
use rusqlite::Error;

use dbFsFile;

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




pub fn listy2(direcory: &str) -> Vec<String>{
    println!("listy2{:?}", direcory);
    let mut items = Vec::<String>::new();

    let path = Path::new(direcory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let pb_2 = path.as_path();
            let as_path_buf = pb_2.as_os_str();
            let path = String::new() + as_path_buf.to_str().unwrap() ;
            items.push(path);

        }
    }
    return items;
}

pub fn json_loader_old(name: &str)  {
    let mut f = File::open(name).expect("file not found");


    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");




    let json = Json::from_str(&contents).unwrap();
    //println!("json={}", json);
    let found = json.find_path(&["provides"]);
    if found != None {
        for item in found {
            println!("provides={}", item);
            let as_a_str = item.to_string();
            //db::insert_provider(&conn, as_a_str);
        }
    }
    let found = json.find_path(&["name"]);
    if found != None {
        for item in found {
            println!("name={}", item);
        }
    }
    let found = json.find_path(&["script"]);
    if found != None {
        for item in found {
            println!("script={}", item);
        }
    }
    let found = json.find_path(&["variables"]);
    if found != None {

        for item in found {
            println!("variables={}", item);
            let bill = item.find_path(&["require_keys"]);
            if bill != None {
                for item2 in bill {
                    println!("variables::require_keys={}", item2);
                }
            }
            let bill = item.find_path(&["provides_keys"]);
            if bill != None {
                for item2 in bill {
                    println!("variables::provides_keys={}", item2);
                }
            }
            let bill = item.find_path(&["require_values"]);
            if bill != None {
                for item2 in bill {
                    let ben = item2.find_path(&[""]);

                    for itme3 in ben {
                        println!("variables::itme3={}", itme3);
                    }
                }
            }
        }

    }
    let found = json.find_path(&["depends"]);
    if found != None {
        for item in found {
            println!("depends={}", item);
        }
    }

}



pub fn loader(name: &str) -> String {
    let mut f = File::open(name).expect("file not found");


    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    return contents;

}



pub fn json_loader_name(conn: &Connection, content: &str)  {
    let mut contents = String::new();
    let json = Json::from_str(&content);



    match json {
        Ok(json) => {
            let found = json.find_path(&["name"]);
            if found != None {
                for item in found {

                    db::insert_job(conn, item.to_string());

                    //println!("name={}", item);
                }
            }
        }
        Err(_)=> {}
    }
    //return json.unwrap();
}


pub fn json_loader_provides_keys(conn: &Connection, content: &str)  {
    let mut contents = String::new();
    let json = Json::from_str(&content);



    match json {
        Ok(json) => {
            let found = json.find_path(&["provides_keys"]);
            if found != None {
                for item in found {
                    let str_item = item.to_string();
                    let str_item_clone1 = str_item.clone();
                    db::insert_provider(conn, item.to_string());

                    //println!("provides_keys={}", item);
                }
            }
        }
        Err(_)=> {}
    }
    //return json.unwrap();
}



pub fn json_loader_require_keys(conn: &Connection, content: &str)  {
    let mut contents = String::new();
    let json = Json::from_str(&content);
    match json {
        Ok(json) => {
            let found = json.find_path(&["require_keys"]);
            if found != None {
                for item in found {

                    db::insert_job_variable_name(conn, item.to_string());

                    //println!("require_keys={}", item);
                }
            }
        }
        Err(_)=> {}
    }
    //return json.unwrap();
}




pub fn deligate(matches : ArgMatches) {

    let conn = db::connect();
    db::create_tables(&conn);



    if let Some(in_v) = matches.values_of("dir-scripts") {
        for in_file in in_v {
            //println!("An input dir-scripts: {}", in_file);
           listy(&in_file);

        }
    }


        // If we specified the multiple() setting we can get all the values
    if let Some(in_v) = matches.values_of("env") {
        for in_file in in_v {
            println!("An input file: {}", in_file);
            job_files_list(&in_file);
        }
    }

    if let Some(in_v) = matches.values_of("dir-jobs") {
        for in_file in in_v {
            let filename = in_file.to_string();
            db::insert_fs_dir(&conn, 0, filename);
        }
    }
    for directory in db::list_fs_dir(&conn) {
        let foo = directory.id;
        let foo_name = directory.name;
        let file_list = listy2(&foo_name);
        for fnccc in file_list {
            println!("An input file: {}", fnccc);
            let s1 = fnccc.clone();
            db::insert_fs_file(&conn, foo, fnccc);
            let mut pk = 10;
            db::pk_fs_file_by_name(&conn, s1, &mut pk);
            println!("ssss: {}", pk);
        }
    }


    let mut scores = HashMap::new();
    for filename in db::list_fs_file(&conn) {
        let mut name = String::from(filename.name);
        let name2 = name.clone();
        let loader_rc = loader(name2.trim());
        scores.insert(name,loader_rc );
    }




    for filename in db::list_fs_file(&conn) {
        let name = filename.name;
        json_loader_name(&conn, &loader(&name));

    }
    for filename in db::list_fs_file(&conn) {
        let name = filename.name;
        json_loader_provides_keys(&conn, &loader(&name));

    }

    for filename in db::list_fs_file(&conn) {
        let name = filename.name;
        json_loader_require_keys(&conn, &loader(&name));

    }


    for filename in db::list_fs_file(&conn) {
        let name = filename.name;
        json_loader_require_keys(&conn, &loader(&name));

    }


    for filename in db::list_provider(&conn) {
        println!("list_provider{:?}", filename);
    }
}




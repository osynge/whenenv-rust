use clap::{ArgMatches};
use std::path::Path;
use db;
use std::fs::File;
use std::io::Read;
use rustc_serialize;
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;

use std::collections::HashMap;
use std::result::Result;
use rusqlite::Connection;
use rusqlite::Error;

use dbFsFile;

use std::result;

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


fn elephant_variable_pk(conn: &Connection, text :&String) -> i32 {
    let mut pk_variable :i32 = 0;
    let rc = db::pk_variable_name_by_name(conn, &text, &mut pk_variable);
    match rc {
        Ok(pk) => {
            return pk_variable;
        }
        Err(_) => {
            let doink = db::insert_variable_name(conn, &text);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_variable_name_by_name(conn, &text, &mut pk_variable);
                    match doin3k {
                        Ok(pk) => {
                            return pk_variable;
                            }
                        Err(_) => {
                                println!("Failed to select variable");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert variable");
                    return 0;
                }
            }

        }
    }
    return pk_variable;
}


fn elephant_job_pk(conn: &Connection, pk_file :&i32, text :&String) -> i32 {
    let mut pk_job :i32 = 0;
    let rc = db::pk_job_by_name(conn, &text, &mut pk_job);
    match rc {
        Ok(pk) => {
            return pk_job;
        }
        Err(_) => {
            let doink = db::insert_job(conn, &pk_file, &text);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_by_name(conn, &text, &mut pk_job);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job;
                            }
                        Err(_) => {
                                println!("Failed to select job");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job");
                    return 0;
                }
            }

        }
    }
    return pk_job;
}


fn elephant_provider_pk(conn: &Connection, in_text :&str) -> i32 {
    let text = String::from(in_text);

    let mut pk_provider :i32 = 0;
    let rc = db::pk_provider_by_name(conn, &text, &mut pk_provider);
    match rc {
        Ok(pk) => {
            return pk_provider;
        }
        Err(_) => {
            let doink = db::insert_provider(conn, &text);
            if doink.is_err() {
                println!("Failed to insert");
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_provider_by_name(conn, &text, &mut pk_provider);
                    match doin3k {
                        Ok(pk) => {
                            return pk_provider;
                            }
                        Err(_) => {
                                println!("Failed to select provider");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert provider");
                    return 0;
                }
            }
        }
    }
    return pk_provider;
}



fn elephant_job_depend_pk(conn: &Connection, job: i32, provider: i32, sq_order: i32) -> i32 {
    let mut pk_job_depend :i32 = 0;
    let rc = db::pk_job_depend_by_all(conn, &job, &provider, &sq_order, &mut pk_job_depend);
    match rc {
        Ok(pk) => {
            return pk_job_depend;
        }
        Err(_) => {
            let doink = db::insert_job_depend(conn, &job, &provider, &sq_order);
            if doink.is_err() {
                println!("Failed to insert");
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_depend_by_all(conn, &job, &provider, &sq_order, &mut pk_job_depend);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job_depend;
                            }
                        Err(_) => {
                                println!("Failed to select job_depend");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job_depend");
                    return 0;
                }
            }
        }
    }
    return pk_job_depend;
}



pub fn json_loader_elephant(conn: &Connection, pk_file: &i32, json :&rustc_serialize::json::Json) {
    let mut pk_job :i32 = 0;
    let mut pk_provider :i32 = 0;
    let mut pk_job_depend :i32 = 0;
    let mut pk_variable_name :i32 = 0;
    let mut order_job_depend :i32 = 0;
    let found = json.find_path(&["name"]);
    if found != None {
        for item in found {
            let str_item = item.to_string();
            pk_job = elephant_job_pk(conn, &pk_file, &str_item);
            println!("pk_job::name={}", pk_job);
        }
    }
    let found = json.find_path(&["provides"]);
    if found != None {
        for item in found {
            if item.is_array() {
                    let ssd = item.as_array();
                    let sdf = ssd.unwrap();
                    let george = sdf.len();
                    let itemfdsd = sdf.iter();
                    for elem in itemfdsd{
                        if elem.is_string() {
                                let sss = elem.as_string();
                                let foo = sss.unwrap();
                                        pk_provider = elephant_provider_pk(conn, &foo);

                                }


                        let str_item = elem.to_string();
                        //pk_provider = elephant_provider_pk(conn, &str_item);
                        //println!("pk_provider::name={}", pk_provider);
                        }
                    }
                }
            }
    let found = json.find_path(&["depends"]);
    if found != None {
        for item in found {
            if item.is_array() {
                let ssd = item.as_array();
                let sdf = ssd.unwrap();
                let george = sdf.len();
                let itemfdsd = sdf.iter();
                for elem in itemfdsd{

                    if elem.is_string() {
                                let sss = elem.as_string();
                                let foo = sss.unwrap();
                                        pk_provider = elephant_provider_pk(conn, &foo);

                                order_job_depend += 10;
                                pk_provider = elephant_provider_pk(conn, &foo);
                                pk_job_depend = elephant_job_depend_pk(conn, pk_job, pk_provider, order_job_depend);
                                println!("pk_provider::name={}", pk_provider);

                                }

                    }
            }

        }
    }
    println!("pk_provider::name={}", pk_provider);

    let found = json.find_path(&["variables"]);
    if found != None {

        for item in found {
            let bill = item.find_path(&["require_keys"]);
            if bill != None {
                for item2 in bill {
                    if item2.is_array() {
                        let ssd = item2.as_array();
                        let sdf = ssd.unwrap();
                        let george = sdf.len();
                        let itemfdsd = sdf.iter();
                        for elem in itemfdsd{
                            if elem.is_string() {
                                let sss = elem.as_string();
                                let foo = sss.unwrap();
                                let name = String::from(foo);
                                pk_variable_name = elephant_variable_pk(conn, &name);
                                }
                            }
                        }
                    }
                }
            let bill = item.find_path(&["provides_keys"]);
            if bill != None {
                for item2 in bill {

                    if item2.is_array() {
                        let ssd = item2.as_array();
                        let sdf = ssd.unwrap();
                        let george = sdf.len();
                        let itemfdsd = sdf.iter();
                        for elem in itemfdsd{
                            if elem.is_string() {
                                let sss = elem.as_string();
                                let foo = sss.unwrap();
                                let name = String::from(foo);
                                pk_variable_name = elephant_variable_pk(conn, &name);
                                }

                            }
                        }
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
}




pub fn json_loader_name(conn: &Connection, pk_file: &i32, content: &str)  {
    let mut contents = String::new();
    let json = Json::from_str(&content);
    let mut pk_job :i32 = 0;
    match json {
        Ok(json) => {
            json_loader_elephant(conn, &pk_file, &json);
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
        let pk_file = filename.id;
        let name = filename.name;
        json_loader_name(&conn, &pk_file, &loader(&name));

    }



    for filename in db::list_provider(&conn) {
        println!("list_provider{:?}", filename);
    }

    for filename in db::list_job_depend(&conn) {
        println!("list_job_depend{:?}", filename);
    }

    for filename in db::list_variable_name(&conn) {
        println!("list_variable_name{:?}", filename);
    }
}




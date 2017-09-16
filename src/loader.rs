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


fn elephant_job_require_variables(conn: &Connection, pk_job :&i32, pk_variable: &i32) -> i32 {
    let mut pk_job_require_variables :i32 = 0;
    let rc = db::pk_job_require_variable_by_name(conn, &pk_job, &pk_variable, &mut pk_job_require_variables);
    match rc {
        Ok(pk) => {
            return pk_job_require_variables;
        }
        Err(_) => {
            let doink = db::insert_job_require_variable(conn, &pk_job, &pk_variable);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_require_variable_by_name(conn, &pk_job, &pk_variable, &mut pk_job_require_variables);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job_require_variables;
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
    return pk_job_require_variables;
}


fn elephant_job_pk(conn: &Connection, pk_file :&i32, in_text :&str) -> i32 {
    let text = String::from(in_text);
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

fn elephant_job_provide_variables(conn: &Connection, job: i32, provider: i32) -> i32 {
    let mut pk_job_provide :i32 = 0;
    let rc = db::pk_job_provide_by_all(conn, &job, &provider,  &mut pk_job_provide);
    match rc {
        Ok(pk) => {
            return pk_job_provide;
        }
        Err(_) => {
            let doink = db::insert_job_provide(conn, &job, &provider);
            if doink.is_err() {
                println!("Failed to insert");
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_provide_by_all(conn, &job, &provider, &mut pk_job_provide);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job_provide;
                            }
                        Err(_) => {
                                println!("Failed to select job_provide");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job_provide");
                    return 0;
                }
            }
        }
    }
    return pk_job_provide;
}


pub fn json_loader_elephant(conn: &Connection, pk_file: &i32, json :&rustc_serialize::json::Json) {
    let mut pk_job :i32 = 0;
    let mut pk_provider :i32 = 0;
    let mut pk_job_depend :i32 = 0;
    let mut pk_variable_name :i32 = 0;
    let mut order_job_depend :i32 = 0;

    if json.is_object() {
        let sssbill = json.as_object();
        for &movie in &sssbill {

            if movie.contains_key("name") {
                let resulkt = movie.get("name");
                let sdf = resulkt.unwrap();
                let mut itemfdsd = sdf.clone();
                if itemfdsd.is_string() {
                    let str_item = itemfdsd.as_string();
                    let foo = str_item.unwrap();
                    pk_job = elephant_job_pk(conn, &pk_file, &foo);
                    //println!("pk_job::name={}", pk_job);
                }
            }
            else
            {
                continue;
            }
            if movie.contains_key("provides") {
                let resulkt = movie.get("provides");
                let sdf = resulkt.unwrap();
                let mut itemfdsd = sdf.clone();
                if itemfdsd.is_array() {
                    //println!("is_arrayxxxxx");

                    let ssd = itemfdsd.as_array();
                    let sdf = ssd.unwrap();
                    let itemfdsd = sdf.iter();
                    for elem in itemfdsd{
                        if elem.is_string() {
                            let sss = elem.as_string();
                            let foo = sss.unwrap();
                            pk_provider = elephant_provider_pk(conn, &foo);
                            //println!("elephant_provider_pk={}", foo);
                            let sq_order = 1;
                            pk_provider = elephant_job_depend_pk(conn, pk_job, pk_provider, sq_order);
                            //println!("pk_provider::name={}", pk_provider);
                        }
                    }
                }
            }
            if movie.contains_key("depends") {
                //println!("depends");
                let resulkt = movie.get("provides");
                let sdf = resulkt.unwrap();
                let mut itemfdsd = sdf.clone();
                if itemfdsd.is_array() {
                    let ssd = itemfdsd.as_array();
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
                                    //println!("pk_provider::name={}", pk_provider);

                        }
                    }
                }
            }
            if movie.contains_key("variables") {
                println!("variables");
                let resulkt = movie.get("variables");
                let sdf = resulkt.unwrap();
                let mut itemfdsd = sdf.clone();
                if itemfdsd.is_object() {
                    let objVariables = itemfdsd.as_object();
                    for &iVariables in &objVariables {
                        if iVariables.contains_key("require_keys") {
                            let resulkdt = movie.get("require_keys");
                            let sdf = resulkt.unwrap();
                            let mut itemfdsd = sdf.clone();
                            if itemfdsd.is_array() {
                                let ssd = itemfdsd.as_array();
                                let sdf = ssd.unwrap();
                                let george = sdf.len();
                                let itemfdsd = sdf.iter();
                                for elem in itemfdsd{
                                    if elem.is_string() {
                                        let sss = elem.as_string();
                                        let foo = sss.unwrap();
                                        let name = String::from(foo);
                                        pk_variable_name = elephant_variable_pk(conn, &name);
                                        elephant_job_require_variables(conn, &pk_job, &pk_variable_name);
                                    }
                                }
                            }
                        }
                        if iVariables.contains_key("provides_keys") {
                            let resulkdt = movie.get("provides_keys");
                            let sdf = resulkt.unwrap();
                            let mut itemfdsd = sdf.clone();
                            if itemfdsd.is_array() {
                                let ssd = itemfdsd.as_array();
                                let sdf = ssd.unwrap();
                                let george = sdf.len();
                                let itemfdsd = sdf.iter();
                                for elem in itemfdsd{
                                    if elem.is_string() {
                                        let sss = elem.as_string();
                                        let foo = sss.unwrap();
                                        let name = String::from(foo);
                                        pk_variable_name = elephant_variable_pk(conn, &name);
                                        elephant_job_provide_variables(conn, pk_job, pk_variable_name);
                                    }
                                }
                            }
                        }
                        if iVariables.contains_key("require_values") {
                            let resulkdtvv = iVariables.get("require_values");
                            let sdfc = resulkdtvv.unwrap();
                            let mut itemfdsdff = sdfc.clone();

                            if itemfdsdff.is_object(){
                                let sssbill = itemfdsdff.as_object();
                                for &dict_key in &sssbill {
                                    //let value_deep = sssbill.get(&dict_key);
                                    if dict_key.contains_key("provides_keys") {

                                        println!("require_values:dict_key:{:?}", dict_key);
                                    }
                                    if iVariables.contains_key("provides_keys") {
                                        println!("require_values:dict_key:{:?}", dict_key);
                                    }
                                    if iVariables.contains_key("provides_keys") {
                                        println!("require_values:dict_key:{:?}", dict_key);
                                    }
                                    if iVariables.contains_key("provides_keys") {
                                        println!("require_values:dict_key:{:?}", dict_key);
                                    }
                                }
                            }
                        }
                    }

                }
            }
        }
    }


    let found = json.find_path(&["variables"]);
    if found != None {

        for item in found {

            let bill = item.find_path(&["require_values"]);

            if bill != None {
                for item2 in bill {
                    if item2.is_array(){
                        println!("is_array:{:?}", item2);
                    }
                    if item2.is_object() {
                        println!("pk_variable_name:{:?}", pk_variable_name);
                        let object = item2.as_object();

                        println!("is_object:{:?}", object);

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
            //json_loader_elephant_deps_depth1(conn, &pk_file, &json);
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
            //println!("An input file: {}", in_file);
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
            //println!("An input file: {}", fnccc);
            let s1 = fnccc.clone();
            db::insert_fs_file(&conn, foo, fnccc);
            let mut pk = 10;
            db::pk_fs_file_by_name(&conn, s1, &mut pk);
            //println!("ssss: {}", pk);
        }
    }


    let mut scores = HashMap::new();
    for filename in db::list_fs_file(&conn) {
        let mut name = String::from(filename.name);
        let name2 = name.clone();
        let loader_rc = loader(name2.trim());
        scores.insert(name,loader_rc );
    }


    // iterate over everything.
    for (filename, contents) in &scores {
        let mut pkfsfile : i32 = 0;
        let filenameStr = filename.clone();
        let doop = db::pk_fs_file_by_name(&conn, filenameStr, & mut pkfsfile);
        json_loader_name(&conn, &pkfsfile, &contents);

    }


    db::variable_pair_list(&conn);
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

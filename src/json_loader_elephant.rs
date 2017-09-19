use rusqlite::Connection;
use db;
use rustc_serialize;
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;




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



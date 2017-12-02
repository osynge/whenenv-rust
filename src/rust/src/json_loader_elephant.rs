use rusqlite::Connection;
use db;
use rustc_serialize;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;
use elephant;
use std::collections::HashSet;
use std::vec::Vec;


pub struct dependencyPair {
    key: String,
    value: String,
}


pub fn json_loader_elephant(conn: &Connection, pk_file: &i32, json: &rustc_serialize::json::Json) {
    let mut job_name = String::new();
    let mut job_provides: HashSet<String> = HashSet::new();
    let mut job_depends: HashSet<String> = HashSet::new();
    let mut job_vaiable_depends: HashSet<String> = HashSet::new();
    let mut job_vaiable_provides: HashSet<String> = HashSet::new();
    let mut job_requires_vaiable_pair: Vec<dependencyPair> = Vec::new();


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
                    job_name = String::from(foo);
                }
            } else {
                continue;
            }
            if movie.contains_key("provides") {
                let resulkt = movie.get("provides");
                let sdf = resulkt.unwrap();
                let mut itemfdsd = sdf.clone();
                if itemfdsd.is_array() {
                    let ssd = itemfdsd.as_array();
                    let sdf = ssd.unwrap();
                    let itemfdsd = sdf.iter();
                    for elem in itemfdsd {
                        if elem.is_string() {
                            let sss = elem.as_string();
                            let foo = sss.unwrap();
                            let providers = String::from(foo);
                            job_provides.insert(providers);
                        }
                    }
                }
                if itemfdsd.is_string() {
                    let sss = itemfdsd.as_string();
                    let foo = sss.unwrap();
                    let providers = String::from(foo);
                    job_provides.insert(providers);
                }
            }
            if movie.contains_key("depends") {
                //println!("depends");
                let resulkt = movie.get("depends");
                let sdf = resulkt.unwrap();
                let mut itemfdsd = sdf.clone();
                if itemfdsd.is_array() {
                    let ssd = itemfdsd.as_array();
                    let sdf = ssd.unwrap();
                    let george = sdf.len();
                    let itemfdsd = sdf.iter();
                    for elem in itemfdsd {

                        if elem.is_string() {
                            let sss = elem.as_string();
                            let foo = sss.unwrap();
                            let depends = String::from(foo);
                            job_depends.insert(depends);
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
                                for elem in itemfdsd {
                                    if elem.is_string() {
                                        let sss = elem.as_string();
                                        let foo = sss.unwrap();
                                        let name = String::from(foo);
                                        job_vaiable_depends.insert(name);
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
                                for elem in itemfdsd {
                                    if elem.is_string() {
                                        let sss = elem.as_string();
                                        let foo = sss.unwrap();
                                        let name = String::from(foo);
                                        job_vaiable_provides.insert(name);
                                        // pk_variable_name = elephant::elephant_variable_pk(conn, &name);
                                        // elephant::elephant_job_provide_variables(conn, &pk_job, &pk_variable_name);
                                    }
                                }
                            }
                        }
                        if iVariables.contains_key("require_values") {
                            let resulkdtvv = iVariables.get("require_values");
                            let sdfc = resulkdtvv.unwrap();
                            let mut itemfdsdff = sdfc.clone();
                            if itemfdsdff.is_object() {
                                let sssbill = itemfdsdff.as_object();
                                for &dict_key in &sssbill {
                                    //let value_deep = sssbill.get(&dict_key);
                                    let mut key_want: String;
                                    let mut value_want: String;
                                    for variable_name in dict_key.keys() {
                                        let variable_name_clone = variable_name.clone();

                                        let value = dict_key.get(variable_name);
                                        let unwrapped = value.unwrap();
                                        if unwrapped.is_string() {
                                            let sss = unwrapped.as_string();
                                            let foo = sss.unwrap();
                                            let name = String::from(foo);

                                            let dp = dependencyPair {
                                                key: variable_name_clone,
                                                value: name,
                                            };
                                            //job_requires_vaiable_pair
                                            job_requires_vaiable_pair.push(dp);

                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let mut pk_job: i32 = 0;
        let mut pk_provider: i32 = 0;
        let mut pk_job_depend: i32 = 0;
        let mut pk_variable_name: i32 = 0;
        println!("job_name:{}", job_name);
        pk_job = elephant::elephant_job_pk(conn, &pk_file, &job_name);
        println!("job_name::pk_job:{}", pk_job);
        for item in job_vaiable_provides {
            println!("job_vaiable_provides:{}", item);
            let pk_variable_name = elephant::elephant_variable_pk(conn, &item);
            let job_vaiable_provides_pk =
                elephant::elephant_job_require_variables(conn, &pk_job, &pk_variable_name);
            println!(
                "job_vaiable_provides::job_vaiable_provides_pk={}",
                pk_provider
            );
        }
        for item in job_vaiable_depends {
            println!("job_vaiable_depends:{}", item);
            let variable_pair_pk =
                elephant::elephant_variable_pair_pk(&conn, &pk_variable_name, &item);
            println!("job_vaiable_depends::variable_pair_pk={}", variable_pair_pk);
            let job_depend_pair_pk =
                elephant::elephant_job_depend_pair_pk(&conn, &pk_job, &variable_pair_pk);
            println!("job_vaiable_depends::job_depend_pair_pk={}", pk_provider);
        }
        for item in job_provides {
            println!("job_provides:{}", item);
            pk_provider = elephant::elephant_provider_pk(conn, &item);
            // println!("elephant_provider_pk={}", foo);
            // let sq_order = 1;
            // pk_provider = elephant::elephant_job_depend_pk(conn, &pk_job, &pk_provider, &sq_order);
            println!("job_provides::pk_provider={}", pk_provider);
        }
        let mut order_job_depend: i32 = 0;
        for item in job_depends {
            println!("job_depends:{}", item);
            let item_pk = elephant::elephant_provider_pk(conn, &item);
            let pk_job_depend =
                elephant::elephant_job_depend_pk(conn, &pk_job, &item_pk, &order_job_depend);
            println!("job_depends::pk_job_depend:{}", pk_job_depend);
            order_job_depend += 10;
        }
        for item in job_requires_vaiable_pair {
            let pk_variable_name = elephant::elephant_variable_pk(conn, &item.key);
            let pk_variable_pair =
                elephant::elephant_variable_pair_pk(conn, &pk_variable_name, &item.value);
            let pk_variable_pair_dep =
                elephant::elephant_job_depend_pair_pk(conn, &pk_job, &pk_variable_pair);
            println!("job_requires_vaiable_pair.key:{}", item.key);
            println!("job_requires_vaiable_pair.value:{}", item.value);
            println!(
                "job_requires_vaiable_pair.pk_variable_name:{}",
                pk_variable_name
            );
            println!(
                "job_requires_vaiable_pair.pk_variable_pair:{}",
                pk_variable_pair
            );
            println!(
                "job_requires_vaiable_pair.pk_variable_pair_dep:{}",
                pk_variable_pair_dep
            );
        }
    }
}

use rusqlite::Connection;
use db;
use rustc_serialize;
use rustc_serialize::{Encodable};
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::json::Json;
use elephant;



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
                    pk_job = elephant::elephant_job_pk(conn, &pk_file, &foo);
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
                            pk_provider = elephant::elephant_provider_pk(conn, &foo);
                            //println!("elephant_provider_pk={}", foo);
                            let sq_order = 1;
                            pk_provider = elephant::elephant_job_depend_pk(conn, &pk_job, &pk_provider, &sq_order);
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
                                    pk_provider = elephant::elephant_provider_pk(conn, &foo);

                                    order_job_depend += 10;
                                    pk_provider = elephant::elephant_provider_pk(conn, &foo);
                                    pk_job_depend = elephant::elephant_job_depend_pk(conn, &pk_job, &pk_provider, &order_job_depend);
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
                                        pk_variable_name = elephant::elephant_variable_pk(conn, &name);
                                        elephant::elephant_job_require_variables(conn, &pk_job, &pk_variable_name);
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
                                        pk_variable_name = elephant::elephant_variable_pk(conn, &name);
                                        elephant::elephant_job_provide_variables(conn, &pk_job, &pk_variable_name);
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
                                    let mut key_want : String;
                                    let mut value_want : String;
                                    for variable_name in dict_key.keys(){
										pk_variable_name = elephant::elephant_variable_pk(&conn, &variable_name);
                                        let value = dict_key.get(variable_name);
                                        let unwrapped = value.unwrap();
                                        if unwrapped.is_string(){
                                            let sss = unwrapped.as_string();
                                            let foo = sss.unwrap();
                                            let name = String::from(foo);
                                            let variable_pair_pk = elephant::elephant_variable_pair_pk(&conn, &pk_variable_name, &name);
                                            let job_depend_pair_pk = elephant::elephant_job_depend_pair_pk(&conn, &pk_job, &variable_pair_pk);
										}
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



use clap::{ArgMatches};
use std::path::Path;

use db;


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




pub fn job_files_list(direcory: &str) {
    let path = Path::new(direcory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let p1 = entry.path();
            let path = p1.as_path();
            println!("dddd{:?}", path);

        }
    }
}


fn elephanc() {
    let conn = db::connect();
    db::create_tables(&conn);
    db::insert_provider(&conn, "Steven".to_string());
    db::provider_list(&conn);
    db::insert_job(&conn, "Steven".to_string(), "".to_string());
    db::insert_job_provide(&conn, 1, 1);
    db::insert_job_depend(&conn, 1, 1,10);
    db::insert_job_variable_name(&conn, "Steven".to_string());
    db::insert_job_require_variable(&conn, 1, 1);
    db::insert_job_variable_pair(&conn, 1, "Steven".to_string());
    db::insert_job_require_variable_pair(&conn, 1, 1);
    conn.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )", &[]).unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None
    };
    conn.execute("INSERT INTO person (name, data)
                  VALUES (?1, ?2)",
                 &[&me.name, &me.data]).unwrap();

    let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Person {
            id: row.get(0),
            name: row.get(1),
            data: row.get(2)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}



pub fn deligate(matches : ArgMatches) {
    if let Some(in_v) = matches.values_of("dir-scripts") {
        for in_file in in_v {
            //println!("An input dir-scripts: {}", in_file);
           //listy(&in_file);
           job_files_list(&in_file);
        }
    }


        // If we specified the multiple() setting we can get all the values
    if let Some(in_v) = matches.values_of("env") {
        for in_file in in_v {
            println!("An input file: {}", in_file);
        }
    }

    if let Some(in_v) = matches.values_of("dir-jobs") {
        for in_file in in_v {
            println!("An input file: {}", in_file);
            listy(&in_file);
        }
    }
    elephanc();
}




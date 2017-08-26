
use rusqlite::Connection;


use clap::{ArgMatches};
use std::path::Path;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>
}



#[derive(Debug)]
struct Provider {
    id: i32,
    name: String,
}



#[derive(Debug)]
struct Job {
    id: i32,
    name: String,
    shell: String,
}

#[derive(Debug)]
struct Variable_ID {
    id: i32,
    name: String
}

#[derive(Debug)]
struct Variable_Value {
    id: i32,
    value: String
}


#[derive(Debug)]
struct Variable_Pair {
    id: i32,
    variable_id: i32,
    value: String
}


#[derive(Debug)]
struct JobRequireVariable_ID {
    id: i32,
    variable_id: i32,
}



#[derive(Debug)]
struct JobRequireVariable_Pair {
    id: i32,
    variable_pair: i32,
}



#[derive(Debug)]
struct JobProvide {
    id: i32,
    provider: i32,
    job: i32,
}


pub fn listy(direcory: &str) {
    let path = Path::new(direcory);
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
}



fn elephanc() {
    let conn = Connection::open_in_memory().unwrap();

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
            println!("An input dir-scripts: {}", in_file);
           listy(&in_file);
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




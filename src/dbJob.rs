use rusqlite::Connection;
use std::result;


#[derive(Debug)]
pub enum Version { Version1, Version2 }

#[derive(Debug)]
pub struct Job {
    pub id: i32,
    pub name: String,
}

trait JobModuleTrait {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;
    fn hashmap(conn: &Connection)-> Vec<Job>;
    // fn update_hashcache(&self, &hashcache) -> &'static str;
}

pub fn table_create_job(conn: &Connection) {
    let load_table = conn.execute("CREATE TABLE JOB (
                  id              INTEGER PRIMARY KEY ASC,
                  name            TEXT NOT NULL UNIQUE
                  )", &[]);
    if load_table.is_err() {
        println!("table_create_job Failed {:?}", load_table);
        return;
    }
    load_table.unwrap();
}



pub fn insert_job(conn: &Connection, name: &String) -> Result<Version, &'static str> {

    let me = Job {
        id: 0,
        name: name.clone(),
    };
    let load_instance = conn.execute("INSERT INTO JOB (name)
                  VALUES (?1)",
                 &[&me.name]);
    if load_instance.is_err() {
        return Err("Insert failed");
    }
    load_instance.unwrap();
    return Ok(Version::Version1);
}

pub fn list_job(conn: &Connection)-> Vec<Job> {
    let mut stmt = conn.prepare("SELECT id, name FROM JOB").unwrap();
    let wraped_job_iter = stmt.query_map(&[], |row| {
        Job {
            id: row.get(0),
            name: row.get(1)
        }
    });
    let mut items = Vec::<Job>::new();
    if wraped_job_iter.is_err() {
        return items;
    }
    let job_iter = wraped_job_iter.unwrap();
    for person in job_iter {
        items.push(person.unwrap());
    }
    return items;
}


pub fn pk_job_by_name(conn: &Connection, name: &String, pk: &mut i32) -> Result<Version, &'static str>{
    let bill = name.clone();
    let mut stmt = conn.prepare("SELECT id, name  FROM JOB WHERE name = ?1").unwrap();
    let job_iter = stmt.query_map(&[&bill], |row| {
        Job {
            id: row.get(0),
            name: row.get(1),
        }
    }).unwrap();
    let mut found = 0;
    let mut items = Vec::<Job>::new();
    for person in job_iter {
        let bill= person.unwrap();
        *pk = bill.id;
        found = 1;
    }
    if found == 1 {
        return Ok(Version::Version1);
    }
    return Err("None found");
}

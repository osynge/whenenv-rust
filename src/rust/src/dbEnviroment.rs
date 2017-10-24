use rusqlite::Connection;
use std::result;

#[derive(Debug)]
pub struct WhenenvEnviroment {
    id: i32,
    fk_session: i32,
    fk_variable_name: i32,
}


pub fn table_create_enviroment(conn: &Connection)  {
    conn.execute("CREATE TABLE WHENENV_ENVIROMENT (
                  id                    INTEGER PRIMARY KEY ASC,
                  fk_session            INTEGER NOT NULL,
                  fk_variable_name           INTEGER NOT NULL,
                  FOREIGN KEY(fk_session) REFERENCES WHENENV_SESSION(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_variable_name) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
}


pub fn insert_enviroment(conn: &Connection, job: &i32, variable: &i32) -> Result<i32, &'static str> {
    let me = WhenenvEnviroment {
        id: 0,
        fk_session: *job,
        fk_variable_name: *variable,

    };
    let enviroment_instance = conn.execute("INSERT INTO WHENENV_ENVIROMENT (fk_session, fk_variable_name)
                  VALUES (?1, ?2)",
                 &[&me.fk_session, &me.fk_variable_name]);
    if enviroment_instance.is_err() {
        return Err("Insert failed");
    }
    enviroment_instance.unwrap();
    return Ok(0);
}


pub fn list_enviroment(conn: &Connection)-> Vec<WhenenvEnviroment> {
    let mut stmt = conn.prepare("SELECT id, fk_session, fk_variable_name  FROM WHENENV_ENVIROMENT").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        WhenenvEnviroment {
            id: row.get(0),
            fk_session: row.get(1),
            fk_variable_name: row.get(2),
        }
    });
    let mut items = Vec::<WhenenvEnviroment>::new();
    if wraped_fs_file_iter.is_err() {
        return items;

    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn enviroment_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, fk_session, fk_variable_name  FROM WHENENV_ENVIROMENT").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        WhenenvEnviroment {
            id: row.get(0),
            fk_session: row.get(1),
            fk_variable_name: row.get(2),
        }
    }).unwrap();

    for person in person_iter {
        println!("Found enviroment {:?}", person.unwrap());
    }
}


pub fn pk_enviroment_by_name(conn: &Connection, job: &i32, variable: &i32, pk: &mut i32) -> Result<i32, &'static str>{
    let mut stmt = conn.prepare("SELECT id, fk_session, fk_variable_name  FROM WHENENV_ENVIROMENT WHERE fk_session = ?1 AND fk_variable_name = ?2").unwrap();
    let enviroment_iter = stmt.query_map(&[job, variable], |row| {
        WhenenvEnviroment {
            id: row.get(0),
            fk_session: row.get(1),
            fk_variable_name: row.get(2),
        }
    });
    if enviroment_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = enviroment_iter.unwrap();
    let mut found = 0;
    let mut items = Vec::<i32>::new();
    for person in result {
        let bill= person.unwrap();
        *pk = bill.id;
        found = 1;
    }
    if found != 0 {
        return Ok(found);
    }
    return Err("None found");
}



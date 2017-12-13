use rusqlite::Connection;
use std::result;
use log;


#[derive(Debug)]
pub struct WhenenvEnviroment {
    id: i32,
    fk_session: i32,
    fk_variable_pair: i32,
}


pub fn table_create_enviroment(conn: &Connection) {
    conn.execute(
        "CREATE TABLE WHENENV_ENVIROMENT (
                  id                    INTEGER PRIMARY KEY ASC,
                  fk_session            INTEGER NOT NULL,
                  fk_variable_pair           INTEGER NOT NULL,
                  FOREIGN KEY(fk_session) REFERENCES WHENENV_SESSION(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_variable_pair) REFERENCES VARIABLE_PAIR(id) ON UPDATE CASCADE
                  )",
        &[],
    ).unwrap();
}


pub fn insert_enviroment(
    conn: &Connection,
    session: &i32,
    variable_pair: &i32,
) -> Result<i32, &'static str> {
    let me = WhenenvEnviroment {
        id: 0,
        fk_session: *session,
        fk_variable_pair: *variable_pair,
    };
    let enviroment_instance = conn.execute(
        "INSERT INTO WHENENV_ENVIROMENT (fk_session, fk_variable_pair)
                  VALUES (?1, ?2)",
        &[&me.fk_session, &me.fk_variable_pair],
    );
    if enviroment_instance.is_err() {
        println!("failed insert_enviroment {:?}", me);
        return Err("insert_enviroment");
    }
    enviroment_instance.unwrap();
    return Ok(0);
}

pub fn list_enviroment(conn: &Connection) -> Vec<WhenenvEnviroment> {
    let mut stmt = conn.prepare(
        "SELECT id, fk_session, fk_variable_pair  FROM WHENENV_ENVIROMENT",
    ).unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        WhenenvEnviroment {
            id: row.get(0),
            fk_session: row.get(1),
            fk_variable_pair: row.get(2),
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
    let mut stmt = conn.prepare(
        "SELECT id, fk_session, fk_variable_pair  FROM WHENENV_ENVIROMENT",
    ).unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        WhenenvEnviroment {
            id: row.get(0),
            fk_session: row.get(1),
            fk_variable_pair: row.get(2),
        }
    }).unwrap();

    for person in person_iter {
        info!("Found enviroment {:?}", person.unwrap());
    }
}


pub fn pk_enviroment_by_name(
    conn: &Connection,
    session: &i32,
    variable_pair: &i32,
) -> Result<i32, &'static str> {
    let mut output = 0;
    let mut stmt = conn.prepare(
        "SELECT
        WHENENV_ENVIROMENT.id,
        WHENENV_ENVIROMENT.fk_session,
        WHENENV_ENVIROMENT.fk_variable_pair
        FROM WHENENV_ENVIROMENT
        WHERE WHENENV_ENVIROMENT.fk_session = ?1 AND WHENENV_ENVIROMENT.fk_variable_pair = ?2",
    ).unwrap();
    let enviroment_iter = stmt.query_map(&[session, variable_pair], |row| {
        WhenenvEnviroment {
            id: row.get(0),
            fk_session: row.get(1),
            fk_variable_pair: row.get(2),
        }
    });
    if enviroment_iter.is_err() {
        println!("pk_enviroment_by_name cccccccccccccccccccc");
        return Err("Insert failed dfdf");
    }
    let result = enviroment_iter.unwrap();
    let mut found = 0;
    let mut items = Vec::<i32>::new();
    for person in result {
        let bill = person.unwrap();
        output = bill.id;
    }
    if output != 0 {
        return Ok(output);
    }
    //println!("None found session {:?}", session);
    //println!("None found variable_pair {:?}", variable_pair);
    return Err("None found");
}

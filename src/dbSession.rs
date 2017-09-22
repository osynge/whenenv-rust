use rusqlite::Connection;
use rusqlite::Error;


#[derive(Debug)]
pub struct Session {
    pub id: i32,
    pub uuid: String,
}


pub fn table_create_session(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE WHENENV_SESSION (
                    id              INTEGER PRIMARY KEY ASC,
                    uuid            TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}


pub fn insert_session(conn: &Connection, uuid: String) -> Result<i32, &'static str> {

    let session = Session {
        id: 0,
        uuid: uuid,
    };
    let mut found = 0;
    let dir_instance = conn.execute("INSERT INTO WHENENV_SESSION (uuid)
                 VALUES (?1)",
                 &[&session.uuid]);
    if dir_instance.is_err() {
        
        return Err("ssss");
    }
    dir_instance.unwrap();
    return Ok(0);
}





pub fn list_session(conn: &Connection)-> Vec<Session> {
    let mut stmt = conn.prepare("SELECT id, uuid FROM WHENENV_SESSION").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        Session {
            id: row.get(0),
            uuid: row.get(1),
        }
    });
    let mut items = Vec::<Session>::new();
    if wraped_fs_file_iter.is_err() {
        return items;
    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {
        items.push(person.unwrap());
    }
    return items;
}



pub fn pk_session_by_uuid(conn: &Connection, uuid: &String, pk: &mut i32) -> Result<i32, &'static str>{
    let mut stmt = conn.prepare("SELECT id, uuid  FROM WHENENV_SESSION WHERE uuid = ?1").unwrap();
    let insert_session_iter = stmt.query_map(&[uuid], |row| {
        Session {
            id: row.get(0),
            uuid: row.get(1),
        }
    });
    if insert_session_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = insert_session_iter.unwrap();
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



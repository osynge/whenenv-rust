use rusqlite::Connection;

#[derive(Debug)]
pub struct FsDirType {
    id: i32,
    name: String,
}


pub fn table_create_fs_dir_type(conn: &Connection)  -> &Connection {
    conn.execute("CREATE TABLE FS_DIR_TYPE (
                  id              INTEGER PRIMARY KEY ASC,
                  name            TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}


pub fn insert_fs_dir_type(conn: &Connection,  name: &String) -> Result<i32, &'static str> {
    let bill = name.clone();
    let me = FsDirType {
        id: 0,
        name: bill,
    };
    let provider_instance = conn.execute("INSERT INTO FS_DIR_TYPE (name)
                  VALUES (?1)",
                 &[&me.name]);
    if provider_instance.is_err() {
        return Err("Insert failed");
    }
    provider_instance.unwrap();
    return Ok(0);
}



pub fn list_fs_dir_type(conn: &Connection)-> Vec<FsDirType> {
    let mut stmt = conn.prepare("SELECT id, name FROM FS_DIR_TYPE").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        FsDirType {
            id: row.get(0),
            name: row.get(1),
        }
    });
    let mut items = Vec::<FsDirType>::new();
    if wraped_fs_file_iter.is_err() {
        return items;
    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {
        items.push(person.unwrap());
    }
    return items;
}



pub fn fs_dir_type_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name  FROM FS_DIR_TYPE").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        FsDirType {
            id: row.get(0),
            name: row.get(1),
        }
    }).unwrap();

    for person in person_iter {
        println!("Found fs_dir_type {:?}", person.unwrap());
    }
}


pub fn pk_fs_dir_type_by_name(conn: &Connection, name: &String, pk: &mut i32) -> Result<i32, &'static str>{
    let mut stmt = conn.prepare("SELECT id, name  FROM FS_DIR_TYPE WHERE name = ?1").unwrap();
    let variable_name_iter = stmt.query_map(&[name], |row| {
        FsDirType {
                id: row.get(0),
                name: row.get(1)
            }
    });
    if variable_name_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = variable_name_iter.unwrap();
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






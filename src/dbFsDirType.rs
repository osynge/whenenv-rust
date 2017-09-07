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



pub fn insert_fs_dir_type(conn: &Connection, name: String) {

    let me = FsDirType {
        id: 0,
        name: name,
    };
    let load_instance = conn.execute("INSERT INTO FS_DIR_TYPE (name)
                  VALUES (?1)",
                 &[&me.name]);
    if load_instance.is_err() {
        return;
    }
    load_instance.unwrap();
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

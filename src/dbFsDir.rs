use rusqlite::Connection;
use rusqlite::Error;


#[derive(Debug)]
pub struct FsDir {
    pub id: i32,
    pub fk_type: i32,
    pub name: String,
}


pub fn table_create_fs_dir(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE FS_DIR (
                  id              INTEGER PRIMARY KEY ASC,
                  fk_type             INTEGER,
                  name            TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}


pub fn insert_fs_dir(conn: &Connection, fk_type: i32, name: String) {

    let fs_dir = FsDir {
        id: 0,
        name: name,
        fk_type: fk_type,
    };
    let dir_instance = conn.execute("INSERT INTO FS_DIR (fk_type, name)
                 VALUES (?1, ?2)",
                 &[&fs_dir.fk_type, &fs_dir.name]);
    if dir_instance.is_err() {
        return;
    }
    dir_instance.unwrap();
}





pub fn list_fs_dir(conn: &Connection)-> Vec<FsDir> {
    let mut stmt = conn.prepare("SELECT id, fk_type, name FROM FS_DIR").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        FsDir {
            id: row.get(0),
            fk_type: row.get(1),
            name: row.get(2),
        }
    });
    let mut items = Vec::<FsDir>::new();
    if wraped_fs_file_iter.is_err() {
        return items;
    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {
        items.push(person.unwrap());
    }
    return items;
}




pub fn list_fs_dir_filter(conn: &Connection)-> Vec<FsDir> {
    let filter = "elephant".to_string();
    let mut stmt = conn.prepare("SELECT id, fk_type, name FROM FS_DIR WHERE name = elephant").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[&filter], |row| {
        FsDir {
            id: row.get(0),
            fk_type: row.get(1),
            name: row.get(2),
        }
    });
    let mut items = Vec::<FsDir>::new();
    if wraped_fs_file_iter.is_err() {
        return items;
    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {
        items.push(person.unwrap());
    }
    return items;
}

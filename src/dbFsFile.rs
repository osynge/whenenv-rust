use rusqlite::Connection;


#[derive(Debug)]
pub struct FsFile {
    pub id: i32,
    pub fk_fs_dir: i32,
    pub name: String,
}


pub fn table_create_fs_file(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE FS_FILE (
                  id                INTEGER PRIMARY KEY ASC,
                  fk_fs_dir         INTEGER NOT NULL,
                  name              TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}


pub fn insert_fs_file(conn: &Connection, fk_fs_dir: i32, name: String) {

    let me = FsFile {
        id: 0,
        name: name,
        fk_fs_dir : fk_fs_dir,
    };
    conn.execute("INSERT INTO FS_FILE (name, fk_fs_dir)
                  VALUES (?1, ?2)",
                 &[&me.name, &me.fk_fs_dir]).unwrap();
}

pub fn list_fs_file(conn: &Connection) -> Vec<FsFile> {
    let mut stmt = conn.prepare("SELECT id, fk_fs_dir,name  FROM FS_FILE").unwrap();
    let fs_file_iter = stmt.query_map(&[], |row| {
        FsFile {
            id: row.get(0),
            fk_fs_dir: row.get(1),
            name: row.get(2),
        }
    }).unwrap();
    let mut items = Vec::<FsFile>::new();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn pk_fs_file_by_name(conn: &Connection, name: String, pk: &mut i32) {
    let mut stmt = conn.prepare("SELECT id, fk_fs_dir,name  FROM FS_FILE WHERE name =?1").unwrap();
    let fs_file_iter = stmt.query_map(&[&name], |row| {
        FsFile {
            id: row.get(0),
            fk_fs_dir: row.get(1),
            name: row.get(2),
        }
    }).unwrap();
    let mut items = Vec::<FsFile>::new();
    for person in fs_file_iter {
        let bill= person.unwrap();
        *pk = bill.id;
        return;
    }
}

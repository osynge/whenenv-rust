use rusqlite::Connection;


#[derive(Debug)]
pub struct FsFile {
    pub id: i32,
    pub fk_fs_dir: i32,
    pub name: String,
}


pub fn table_create_fs_file(conn: &Connection) -> &Connection {
    conn.execute(
        "CREATE TABLE FS_FILE (
                  id                INTEGER PRIMARY KEY ASC,
                  fk_fs_dir         INTEGER NOT NULL,
                  name              TEXT NOT NULL UNIQUE
                  )",
        &[],
    ).unwrap();
    return conn;
}




pub fn insert_fs_file(
    conn: &Connection,
    fk_fs_dir: i32,
    name: String,
) -> Result<i32, &'static str> {
    let me = FsFile {
        id: 0,
        name: name,
        fk_fs_dir: fk_fs_dir,
    };
    let variable_pair_instance = conn.execute(
        "INSERT INTO FS_FILE (name, fk_fs_dir)
                  VALUES (?1, ?2)",
        &[&me.name, &me.fk_fs_dir],
    );
    if variable_pair_instance.is_err() {
        return Err("Insert failed");
    }
    variable_pair_instance.unwrap();
    return Ok(0);
}



pub fn list_fs_file(conn: &Connection) -> Vec<FsFile> {
    let mut stmt = conn.prepare("SELECT id, fk_fs_dir,name  FROM FS_FILE")
        .unwrap();
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
    let mut stmt = conn.prepare("SELECT id, fk_fs_dir,name  FROM FS_FILE WHERE name =?1")
        .unwrap();
    let fs_file_iter = stmt.query_map(&[&name], |row| {
        FsFile {
            id: row.get(0),
            fk_fs_dir: row.get(1),
            name: row.get(2),
        }
    }).unwrap();
    for person in fs_file_iter {
        let bill = person.unwrap();
        *pk = bill.id;
        return;
    }
}

pub fn list_fs_file_type(conn: &Connection, fk_fs_type: &i32) -> Vec<FsFile> {
    let mut stmt = conn.prepare(
        "SELECT FS_FILE.id, FS_FILE.name, FS_FILE.fk_fs_dir FROM FS_FILE
        INNER JOIN FS_DIR
        on FS_FILE.fk_fs_dir = FS_DIR.id
        INNER JOIN FS_DIR_TYPE
        on FS_DIR.fk_type = FS_DIR_TYPE.id
        WHERE FS_DIR_TYPE.id =?1
        ",
    ).unwrap();
    let fs_file_iter = stmt.query_map(&[fk_fs_type], |row| {
        FsFile {
            id: row.get(0),
            fk_fs_dir: row.get(2),
            name: row.get(1),
        }
    }).unwrap();
    let mut items = Vec::<FsFile>::new();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}



#[cfg(test)]
mod tests {
    #[test]
    fn insert_fs_file() {
        use db;
        use dbFsDirType;
        use dbFsDir;
        let conn = db::connect();
        db::create_tables(&conn);
        let str_job_files_list = String::from("job_files");
        let pk_directory_type_jobs = dbFsDirType::insert_fs_dir_type(&conn, &str_job_files_list);
        let vec_dir_type = dbFsDirType::list_fs_dir_type(&conn);
        let mut pk_dir_type = 0;
        let mut counter = 0;
        for dir_type in vec_dir_type {
            counter += 1;
            pk_dir_type = dir_type.id;
        }
        assert!(counter == 1);
        assert!(pk_dir_type == 1);
        let str_directory_path = String::from("directory_path");
        let insert_fs_dir_result = dbFsDir::insert_fs_dir(&conn, &pk_dir_type, &str_directory_path);
        let fs_dir_result = insert_fs_dir_result.unwrap();
        assert!(fs_dir_result == 0);
        let list_fs_dir_result =
            dbFsDir::list_fs_dir_by_all(&conn, &pk_dir_type, &str_directory_path);
        let vec_dir = list_fs_dir_result.unwrap();
        counter = 0;
        let mut pk_dir = 0;
        for dir in vec_dir {
            counter += 1;
            pk_dir = dir.id;
        }
        assert!(counter == 1);
        assert!(pk_dir == 1);
    }
}

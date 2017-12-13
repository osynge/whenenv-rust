use rusqlite::Connection;


#[derive(Debug)]
pub struct FsDir {
    pub id: i32,
    pub fk_type: i32,
    pub name: String,
}


pub fn table_create_fs_dir(conn: &Connection) -> &Connection {
    conn.execute(
        "CREATE TABLE FS_DIR (
                  id              INTEGER PRIMARY KEY ASC,
                  fk_type         INTEGER NOT NULL,
                  name            TEXT NOT NULL UNIQUE,
                  FOREIGN KEY(fk_type) REFERENCES FS_DIR_TYPE(id) ON UPDATE CASCADE
                  )",
        &[],
    ).unwrap();
    return conn;
}




pub fn insert_fs_dir(conn: &Connection, fk_type: &i32, name: &str) -> Result<i32, &'static str> {
    let fs_dir = FsDir {
        id: 0,
        name: name.to_string(),
        fk_type: fk_type.clone(),
    };
    let provider_instance = conn.execute(
        "INSERT INTO FS_DIR (fk_type, name)
                 VALUES (?1, ?2)",
        &[&fs_dir.fk_type, &fs_dir.name],
    );
    if provider_instance.is_err() {
        return Err("INSERT INTO FS_DIR_TYPE failed");
    }
    provider_instance.unwrap();
    return Ok(0);
}




pub fn list_fs_dir(conn: &Connection) -> Vec<FsDir> {
    let mut stmt = conn.prepare("SELECT id, fk_type, name FROM FS_DIR")
        .unwrap();
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




pub fn list_fs_dir_filter(conn: &Connection) -> Vec<FsDir> {
    let filter = "elephant".to_string();
    let mut stmt = conn.prepare("SELECT id, fk_type, name FROM FS_DIR WHERE name = elephant")
        .unwrap();
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



pub fn list_fs_dir_by_all(
    conn: &Connection,
    fk_fs_type: &i32,
    name: &str,
) -> Result<Vec<FsDir>, &'static str> {
    let mut stmt = conn.prepare(
        "SELECT FS_DIR.id, FS_DIR.fk_type, FS_DIR.name  FROM FS_DIR
        INNER JOIN FS_DIR_TYPE
        on FS_DIR.fk_type = FS_DIR_TYPE.id
        WHERE
        FS_DIR.fk_type = ?1",
    ).unwrap();
    let fs_dir_iter = stmt.query_map(&[fk_fs_type], |row| {
        FsDir {
            id: row.get(0),
            fk_type: row.get(1),
            name: row.get(2),
        }
    });
    if fs_dir_iter.is_err() {
        return Err("SELECT failed");
    }
    let result = fs_dir_iter.unwrap();
    let mut items = Vec::<FsDir>::new();
    let mut found = 0;
    for person in result {
        let dir = person.unwrap();
        items.push(dir);
        found = 1;
    }
    if found != 0 {
        return Ok(items);
    }
    return Err("None found");
}


#[cfg(test)]
mod tests {
    #[test]
    fn insert_fs_dir() {
        use db;
        use elephant;
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
        let freed = 1;
        let list_fs_dir_result =
            dbFsDir::list_fs_dir_by_all(&conn, &pk_dir_type, &str_directory_path);
        let vec_dir = list_fs_dir_result.unwrap();
        let mut counter2 = 0;
        let mut pk_dir = 0;
        for dir in vec_dir {
            counter2 += 1;
            pk_dir = dir.id;
        }
        assert!(counter2 == 1);
        assert!(pk_dir == 1);
    }
}

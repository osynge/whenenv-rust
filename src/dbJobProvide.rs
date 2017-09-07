use rusqlite::Connection;


#[derive(Debug)]
pub struct JobProvide {
    id: i32,
    fk_job: i32,
    fk_provider: i32,
}


pub fn table_create_job_provide(conn: &Connection)  -> &Connection  {
    let load_table = conn.execute("CREATE TABLE JOBPROVIDE (
                  id            INTEGER PRIMARY KEY ASC,
                  fk_job           INTEGER,
                  fk_provider      INTEGER,
                  FOREIGN KEY(fk_job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_provider) REFERENCES PROVIDER(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
    return conn;
}





pub fn insert_job_provide(conn: &Connection, fk_job: i32 , fk_provider: i32, name: String) {

    let me = JobProvide {
        id: 0,
        fk_job: fk_job,
        fk_provider: fk_provider,
    };
    let load_instance = conn.execute("INSERT INTO JOBPROVIDE (fk_job, fk_provider)
                  VALUES (?1, ?2)",
                 &[&me.fk_job, &me.fk_provider]);
    if load_instance.is_err() {
        return;
    }
    load_instance.unwrap();
}


pub fn list_job_provide(conn: &Connection)-> Vec<JobProvide> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_provider FROM JOBPROVIDE").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        JobProvide {
            id: row.get(0),
            fk_job: row.get(1),
            fk_provider: row.get(2),
        }
    });
    let mut items = Vec::<JobProvide>::new();
    if wraped_fs_file_iter.is_err() {
        return items;
    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {
        items.push(person.unwrap());
    }
    return items;
}

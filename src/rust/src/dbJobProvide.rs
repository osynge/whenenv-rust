use rusqlite::Connection;


#[derive(Debug)]
pub struct JobProvide {
    id: i32,
    fk_job: i32,
    fk_variable: i32,
}


pub fn table_create_job_provide(conn: &Connection) -> Result<(), &'static str> {
    let load_table = conn.execute_batch(
        "CREATE TABLE JOBPROVIDE (
                  id            INTEGER PRIMARY KEY ASC,
                  fk_job           INTEGER,
                  fk_variable      INTEGER,
                  UNIQUE (fk_job, fk_variable) ON CONFLICT REPLACE
                  FOREIGN KEY(fk_job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_variable) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )",
    );
    if load_table.is_err() {
        return Err("table_create_session Failed");
    }
    load_table.unwrap();
    return Ok(());
}


pub fn insert_job_provide(
    conn: &Connection,
    job: &i32,
    provider: &i32,
) -> Result<i32, &'static str> {

    let me = JobProvide {
        id: 0,
        fk_job: *job,
        fk_variable: *provider,
    };
    let load_instance = conn.execute(
        "INSERT INTO JOBPROVIDE (fk_job, fk_variable)
                  VALUES (?1, ?2)",
        &[&me.fk_job, &me.fk_variable],
    );
    if load_instance.is_err() {
        return Err("failed trying to INSERT INTO JOBPROVIDE");
    }
    load_instance.unwrap();
    return Ok(0);
}


pub fn list_job_provide(conn: &Connection) -> Vec<JobProvide> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable FROM JOBPROVIDE")
        .unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        JobProvide {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
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

pub fn pk_job_provide_by_all(
    conn: &Connection,
    fk_job: &i32,
    provider: &i32,
    pk: &mut i32,
) -> Result<i32, &'static str> {
    let mut stmt = conn.prepare("SELECT JOBPROVIDE.id, JOBPROVIDE.fk_job, JOBPROVIDE.fk_variable FROM JOBPROVIDE WHERE JOBPROVIDE.fk_job = ?1 AND JOBPROVIDE.fk_variable=?2").unwrap();
    let job_provide_iter = stmt.query_map(&[fk_job, provider], |row| {
        JobProvide {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
        }
    });
    if job_provide_iter.is_err() {
        return Err("Insert failed");
    }
    let result = job_provide_iter.unwrap();
    let mut found = 0;
    for person in result {
        let bill = person.unwrap();
        *pk = bill.id;
        found = 1;
    }
    if found != 0 {
        return Ok(found);
    }
    return Err("None found");
}

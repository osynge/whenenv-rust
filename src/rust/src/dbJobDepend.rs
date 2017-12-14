use rusqlite::Connection;


#[derive(Debug)]
pub struct JobDepend {
    pub id: i32,
    pub fk_job: i32,
    pub fk_variable: i32,
    pub sq_order: i32,
}


pub fn table_create_job_depend(conn: &Connection) -> &Connection {
    conn.execute(
        "CREATE TABLE JOBDEPEND (
                  id                INTEGER PRIMARY KEY ASC,
                  fk_job            INTEGER NOT NULL,
                  fk_variable       INTEGER NOT NULL,
                  sq_order          INTEGER NOT NULL,
                  UNIQUE (fk_job, fk_variable) ON CONFLICT REPLACE
                  FOREIGN KEY(fk_job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_variable) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )",
        &[],
    ).unwrap();
    return conn;
}


pub fn insert_job_depend(
    conn: &Connection,
    job: &i32,
    provider: &i32,
    sq_order: &i32,
) -> Result<i32, &'static str> {
    let me = JobDepend {
        id: 0,
        fk_job: *job,
        fk_variable: *provider,
        sq_order: *sq_order,
    };
    let job_depend_instance = conn.execute(
        "INSERT INTO JOBDEPEND (fk_job, fk_variable, sq_order)
                  VALUES (?1, ?2, ?3)",
        &[&me.fk_job, &me.fk_variable, &me.sq_order],
    );
    if job_depend_instance.is_err() {
        return Err("Failed trying to INSERT INTO JOBDEPEND");
    }
    job_depend_instance.unwrap();
    return Ok(0);
}


pub fn list_job_depend(conn: &Connection) -> Vec<JobDepend> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable, sq_order  FROM JOBDEPEND")
        .unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        JobDepend {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
            sq_order: row.get(3),
        }
    });
    let mut items = Vec::<JobDepend>::new();
    if wraped_fs_file_iter.is_err() {
        return items;

    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn job_depend_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable, sq_order FROM JOBDEPEND")
        .unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        JobDepend {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
            sq_order: row.get(3),
        }
    }).unwrap();

    for person in person_iter {
        info!("Found job_depend {:?}", person.unwrap());
    }
}

pub fn pk_job_depend_by_all(
    conn: &Connection,
    fk_job: &i32,
    provider: &i32,
    sq_order: &i32,
) -> Result<i32, &'static str> {
    let mut output = 0;
    let mut stmt = conn.prepare("SELECT JOBDEPEND.id, JOBDEPEND.fk_job, JOBDEPEND.fk_variable, JOBDEPEND.sq_order  FROM JOBDEPEND WHERE JOBDEPEND.fk_job = ?1 AND JOBDEPEND.fk_variable=?2 AND JOBDEPEND.sq_order=?3 ").unwrap();
    let job_depend_iter = stmt.query_map(&[fk_job, provider, sq_order], |row| {
        JobDepend {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
            sq_order: row.get(3),
        }
    });
    if job_depend_iter.is_err() {
        return Err("Insert failed");
    }
    let result = job_depend_iter.unwrap();
    let mut found = 0;
    for person in result {
        let bill = person.unwrap();
        output = bill.id;
        found = 1;
    }
    if found != 0 {
        return Ok(output);
    }
    return Err("None found");
}


pub fn job_depend_count(conn: &Connection, fk_job: &i32) -> Result<i32, &'static str> {
    let count_prep_rc = conn.prepare(
        "SELECT JOBDEPEND.id FROM JOBDEPEND WHERE JOBDEPEND.fk_job = ?1",
    );
    match count_prep_rc {
        Ok(mut result) => {
            let result_row = result.query(&[fk_job]);
            let mut rox = result_row.unwrap();
            let mut counter = 0;
            while let Some(_) = rox.next() {
                counter += 1;
            }
            return Ok(counter);
        }
        Err(_) => {
            return Err("Failed");
        }
    }
}

use rusqlite::Connection;


#[derive(Debug)]
pub struct JobRequireVariable {
    id: i32,
    fk_job: i32,
    fk_variable: i32,
}


pub fn table_create_job_require_variable(conn: &Connection) {
    conn.execute(
        "CREATE TABLE JOB_REQUIRE_VARIABLE (
                  id                    INTEGER PRIMARY KEY ASC,
                  fk_job                INTEGER NOT NULL,
                  fk_variable           INTEGER NOT NULL,
                  FOREIGN KEY(fk_job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_variable) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )",
        &[],
    ).unwrap();
}


pub fn insert_job_require_variable(
    conn: &Connection,
    job: &i32,
    variable: &i32,
) -> Result<i32, &'static str> {
    let me = JobRequireVariable {
        id: 0,
        fk_job: *job,
        fk_variable: *variable,
    };
    let job_require_variable_instance = conn.execute(
        "INSERT INTO JOB_REQUIRE_VARIABLE (fk_job, fk_variable)
                  VALUES (?1, ?2)",
        &[&me.fk_job, &me.fk_variable],
    );
    if job_require_variable_instance.is_err() {
        return Err("INSERT INTO JOB_REQUIRE_VARIABLE failed");
    }
    job_require_variable_instance.unwrap();
    return Ok(0);
}


pub fn list_job_require_variable(conn: &Connection) -> Vec<JobRequireVariable> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable  FROM JOB_REQUIRE_VARIABLE")
        .unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        JobRequireVariable {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
        }
    });
    let mut items = Vec::<JobRequireVariable>::new();
    if wraped_fs_file_iter.is_err() {
        return items;

    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn job_require_variable_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable  FROM JOB_REQUIRE_VARIABLE")
        .unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        JobRequireVariable {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
        }
    }).unwrap();

    for person in person_iter {
        info!("Found job_require_variable {:?}", person.unwrap());
    }
}


pub fn pk_job_require_variable_by_name(
    conn: &Connection,
    job: &i32,
    variable: &i32,
) -> Result<i32, &'static str> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable  FROM JOB_REQUIRE_VARIABLE WHERE fk_job = ?1 AND fk_variable = ?2").unwrap();
    let job_require_variable_iter = stmt.query_map(&[job, variable], |row| {
        JobRequireVariable {
            id: row.get(0),
            fk_job: row.get(1),
            fk_variable: row.get(2),
        }
    });
    if job_require_variable_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = job_require_variable_iter.unwrap();
    let mut found = 0;
    let mut output = 0;
    for person in result {
        let bill = person.unwrap();
        output = bill.id;
        found = 1;
        break;
    }
    if found != 0 {
        return Ok(output);
    }
    return Err("None found");
}

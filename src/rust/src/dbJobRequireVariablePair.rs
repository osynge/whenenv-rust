use rusqlite::Connection;

#[derive(Debug)]
pub struct JobRequireVariablePair {
    pub id: i32,
    pub fk_job: i32,
    pub fk_variable_pair: i32,
}

pub fn table_create_job_require_variable_pair(conn: &Connection) {
    let load_table = conn.execute(
        "CREATE TABLE JOB_REQUIRE_VALUE (
                  id            INTEGER PRIMARY KEY ASC,
                  fk_job           INTEGER NOT NULL,
                  fk_variable_pair INTEGER NOT NULL,
                  FOREIGN KEY(fk_job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_variable_pair) REFERENCES VARIABLE_PAIR(id) ON UPDATE CASCADE
                  )",
        &[],
    );
    if load_table.is_err() {
        error!(
            "table_create_job_require_variable_pair Failed {:?}",
            load_table
        );
        return;
    }
    load_table.unwrap();
}

pub fn insert_job_require_variable_pair(
    conn: &Connection,
    job: &i32,
    variable_pair: &i32,
) -> Result<i32, &'static str> {
    let me = JobRequireVariablePair {
        id: 0,
        fk_job: *job,
        fk_variable_pair: *variable_pair,
    };
    let variable_pair_instance = conn.execute(
        "INSERT INTO JOB_REQUIRE_VALUE (fk_job, fk_variable_pair)
                  VALUES (?1, ?2)",
        &[&me.fk_job, &me.fk_variable_pair],
    );
    if variable_pair_instance.is_err() {
        return Err("Insert failed");
    }
    variable_pair_instance.unwrap();
    return Ok(0);
}

pub fn list_job_require_variable_pair(conn: &Connection) -> Vec<JobRequireVariablePair> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable_pair  FROM JOB_REQUIRE_VALUE")
        .unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| JobRequireVariablePair {
        id: row.get(0),
        fk_job: row.get(1),
        fk_variable_pair: row.get(2),
    });
    let mut items = Vec::<JobRequireVariablePair>::new();
    if wraped_fs_file_iter.is_err() {
        return items;
    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {
        items.push(person.unwrap());
    }
    return items;
}

pub fn job_require_variable_pair_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_variable_pair FROM JOB_REQUIRE_VALUE")
        .unwrap();
    let person_iter = stmt.query_map(&[], |row| JobRequireVariablePair {
        id: row.get(0),
        fk_job: row.get(1),
        fk_variable_pair: row.get(2),
    }).unwrap();

    for person in person_iter {
        info!("Found variable_pair {:?}", person.unwrap());
    }
}

pub fn pk_job_require_variable_pair_by_all(
    conn: &Connection,
    job: &i32,
    variable_pair: &i32,
) -> Result<i32, &'static str> {
    let mut output = 0;
    let mut stmt = conn.prepare("SELECT JOB_REQUIRE_VALUE.id, JOB_REQUIRE_VALUE.fk_job, JOB_REQUIRE_VALUE.fk_variable_pair  FROM JOB_REQUIRE_VALUE WHERE JOB_REQUIRE_VALUE.fk_job = ?1 AND JOB_REQUIRE_VALUE.fk_variable_pair = ?2").unwrap();
    let variable_pair_iter = stmt.query_map(&[job, variable_pair], |row| JobRequireVariablePair {
        id: row.get(0),
        fk_job: row.get(1),
        fk_variable_pair: row.get(2),
    });
    if variable_pair_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = variable_pair_iter.unwrap();
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

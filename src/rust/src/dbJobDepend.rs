use rusqlite::Connection;
use std::result;


#[derive(Debug)]
pub struct JobDepend {
    pub id: i32,
    pub fk_job: i32,
    pub fk_provider: i32,
    pub sq_order: i32,
}


pub fn table_create_job_depend(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE JOBDEPEND (
                  id                INTEGER PRIMARY KEY ASC,
                  fk_job            INTEGER NOT NULL,
                  fk_provider       INTEGER NOT NULL,
                  sq_order          INTEGER NOT NULL,
                  UNIQUE (fk_job, fk_provider) ON CONFLICT REPLACE
                  FOREIGN KEY(fk_job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(fk_provider) REFERENCES PROVIDER(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
    return conn;
}


pub fn insert_job_depend(conn: &Connection, job: &i32, provider: &i32, sq_order: &i32) -> Result<i32, &'static str> {
    let me = JobDepend {
        id: 0,
        fk_job: *job,
        fk_provider: *provider,
        sq_order: *sq_order,
    };
    let job_depend_instance = conn.execute("INSERT INTO JOBDEPEND (fk_job, fk_provider, sq_order)
                  VALUES (?1, ?2, ?3)",
                 &[&me.fk_job, &me.fk_provider, &me.sq_order]);
    if job_depend_instance.is_err() {
        return Err("Insert failed");
    }
    job_depend_instance.unwrap();
    return Ok(0);
}


pub fn list_job_depend(conn: &Connection)-> Vec<JobDepend> {
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_provider, sq_order  FROM JOBDEPEND").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        JobDepend {
            id: row.get(0),
            fk_job: row.get(1),
            fk_provider: row.get(2),
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
    let mut stmt = conn.prepare("SELECT id, fk_job, fk_provider, sq_order FROM JOBDEPEND").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        JobDepend {
            id: row.get(0),
            fk_job: row.get(1),
            fk_provider: row.get(2),
            sq_order: row.get(3),
        }
    }).unwrap();

    for person in person_iter {
        println!("Found job_depend {:?}", person.unwrap());
    }
}

pub fn pk_job_depend_by_all(conn: &Connection, fk_job: &i32, provider: &i32, sq_order: &i32) -> Result<i32, &'static str>{
    let mut output = 0;
    let mut stmt = conn.prepare("SELECT JOBDEPEND.id, JOBDEPEND.fk_job, JOBDEPEND.fk_provider, JOBDEPEND.sq_order  FROM JOBDEPEND WHERE JOBDEPEND.fk_job = ?1 AND JOBDEPEND.fk_provider=?2 AND JOBDEPEND.sq_order=?3 ").unwrap();
    let job_depend_iter = stmt.query_map(&[fk_job, provider, sq_order], |row| {
        JobDepend {
            id: row.get(0),
            fk_job: row.get(1),
            fk_provider: row.get(2),
            sq_order: row.get(3),
        }
    });
    if job_depend_iter.is_err() {
        return Err("Insert failed");
    }
    let result = job_depend_iter.unwrap();
    let mut found = 0;
    let mut items = Vec::<i32>::new();
    for person in result {
        let bill= person.unwrap();
        output = bill.id;
        found = 1;
    }
    if found != 0 {
        return Ok(output);
    }
    return Err("None found");
}

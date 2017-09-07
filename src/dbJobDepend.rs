use rusqlite::Connection;

#[derive(Debug)]
pub struct JobDepend {
    pub id: i32,
    pub job: i32,
    pub provider: i32,
    pub sq_order: i32,
}


pub fn table_create_job_depend(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE JOBDEPEND (
                  id            INTEGER PRIMARY KEY ASC,
                  job           INTEGER NOT NULL,
                  provider      INTEGER NOT NULL,
                  sq_order      INTEGER NOT NULL UNIQUE,
                  FOREIGN KEY(job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(provider) REFERENCES PROVIDER(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
    return conn;
}



pub fn insert_job_depend(conn: &Connection, job: i32, provider: i32, sq_order: i32) {

    let me = JobDepend {
        id: 0,
        job: job,
        provider: provider,
        sq_order: sq_order,
    };
    conn.execute("INSERT INTO JOBDEPEND (job, provider, sq_order)
                  VALUES (?1, ?2, ?3)",
                 &[&me.job, &me.provider, &me.sq_order]).unwrap();
}

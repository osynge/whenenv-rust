use rusqlite::Connection;
use rusqlite::Error;
pub use dbFsDirType::FsDirType as FsDirType;
pub use dbFsDirType::table_create_fs_dir_type as table_create_fs_dir_type;
pub use dbFsDirType::insert_fs_dir_type as insert_fs_dir_type;
pub use dbFsDirType::list_fs_dir_type as list_fs_dir_type;
pub use dbFsDir::insert_fs_dir as insert_fs_dir;
pub use dbFsDir::FsDir as FsDir;
pub use dbFsDir::list_fs_dir as list_fs_dir;
pub use dbFsDir::table_create_fs_dir as table_create_fs_dir;
pub use dbFsFile::insert_fs_file as dbFsFile;
pub use dbFsFile::insert_fs_file as insert_fs_file;
pub use dbFsFile::list_fs_file as list_fs_file;
pub use dbFsFile::pk_fs_file_by_name as pk_fs_file_by_name;
pub use dbFsFile::table_create_fs_file as table_create_fs_file;
pub use dbProvider::Provider as Provider;
pub use dbProvider::table_create_provider as table_create_provider;
pub use dbProvider::insert_provider as insert_provider;
pub use dbProvider::list_provider as list_provider;
pub use dbProvider::pk_provider_by_name as pk_provider_by_name;
pub use dbJob::Job as Job;
pub use dbJob::table_create_job as table_create_job;
pub use dbJob::insert_job as insert_job;
pub use dbJob::list_job as list_job;
pub use dbJob::pk_job_by_name as pk_job_by_name;


pub use dbJobProvide::JobProvide as JobProvide;
pub use dbJobProvide::table_create_job_provide as table_create_job_provide;
pub use dbJobProvide::insert_job_provide as insert_job_type;
pub use dbJobProvide::list_job_provide as list_job_type;



pub use dbJobDepend::JobDepend as JobDepend;
pub use dbJobDepend::table_create_job_depend as table_create_job_depend;
pub use dbJobDepend::insert_job_depend as insert_job_depend;




#[derive(Debug)]
struct VariableName {
    id: i32,
    name: String,
}



fn table_create_variable_name(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE VARIABLE_NAME (
                  id            INTEGER PRIMARY KEY ASC,
                  name      TEXT
                  )", &[]).unwrap();
    return conn;
}


#[derive(Debug)]
struct JobRequireVariable {
    id: i32,
    job: i32,
    variable_id: i32,
}


fn table_create_require_variable(conn: &Connection)  {
    conn.execute("CREATE TABLE JOB_REQUIRE_VARIABLE (
                  id            INTEGER PRIMARY KEY ASC,
                  job           INTEGER NOT NULL,
                  variable_id   INTEGER NOT NULL,
                  FOREIGN KEY(job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(variable_id) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
}


#[derive(Debug)]
struct VariablePair {
    id: i32,
    variable_id: i32,
    variable_value: String
}




fn table_create_variable_pair(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE  VARIABLE_PAIR (
                  id            INTEGER PRIMARY KEY ASC,
                  variable_id   INTEGER NOT NULL,
                  variable_value  TEXT,
                  FOREIGN KEY(variable_id) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
       return conn;
}


#[derive(Debug)]
struct JobRequireVariablePair {
    id: i32,
    job: i32,
    variable_pair: i32,
}




fn table_create_require_variable_pair(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE JOB_REQUIRE_VALUE (
                  id            INTEGER PRIMARY KEY ASC,
                  job           INTEGER NOT NULL,
                  variable_pair INTEGER NOT NULL,
                  FOREIGN KEY(job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(variable_pair) REFERENCES VARIABLE_PAIR(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
    return conn;
}




pub fn connect() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", &[]).unwrap();
    return conn;
}


pub fn create_tables(conn: &Connection)  -> &Connection {
    table_create_fs_dir_type(&conn);
    table_create_fs_dir(&conn);
    table_create_fs_file(&conn);
    let newcon = table_create_provider(&conn);
    let newcon2 = table_create_job(&newcon);
    table_create_job_depend(&conn);
    table_create_job_provide(&conn);
    table_create_variable_name(&conn);
    table_create_variable_pair(&conn);
    table_create_require_variable(&conn);
    table_create_require_variable_pair(&conn);
    return &newcon;
}


pub fn insert_job_variable_name(conn: &Connection, name: String) {

    let me = VariableName {
        id: 0,
        name: name,
    };
    conn.execute("INSERT INTO VARIABLE_NAME (name)
                  VALUES (?1)",
                 &[&me.name]).unwrap();
}



pub fn insert_job_require_variable(conn: &Connection, job: i32,  require_id: i32) {

    let me = JobRequireVariable {
        id: 0,
        job: job,
        variable_id: require_id,

    };
    conn.execute("INSERT INTO JOB_REQUIRE_VARIABLE (job, variable_id)
                  VALUES (?1, ?2)",
                 &[&me.job, &me.variable_id]).unwrap();
}




pub fn insert_job_variable_pair(conn: &Connection, variable_id: i32, value: String) {

    let me = VariablePair {
        id: 0,
        variable_id: variable_id,
        variable_value: value,
    };
    conn.execute("INSERT INTO VARIABLE_PAIR (variable_id, variable_value)
                  VALUES (?1, ?2)",
                 &[&me.variable_id, &me.variable_value]).unwrap();
}




pub fn insert_job_require_variable_pair(conn: &Connection, job: i32,  require_id: i32) {

    let me = JobRequireVariablePair {
        id: 0,
        job: job,
        variable_pair: require_id,

    };
    conn.execute("INSERT INTO JOB_REQUIRE_VALUE (job, variable_pair)
                  VALUES (?1, ?2)",
                 &[&me.job, &me.variable_pair]).unwrap();
}


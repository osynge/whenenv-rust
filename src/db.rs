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
pub use dbJobProvide::insert_job_provide as insert_job_provide;
pub use dbJobProvide::list_job_provide as list_job_type;
pub use dbJobProvide::pk_job_provide_by_all as pk_job_provide_by_all;
pub use dbJobDepend::JobDepend as JobDepend;
pub use dbJobDepend::table_create_job_depend as table_create_job_depend;
pub use dbJobDepend::insert_job_depend as insert_job_depend;
pub use dbJobDepend::pk_job_depend_by_all as pk_job_depend_by_all;
pub use dbJobDepend::list_job_depend as list_job_depend;
pub use dbVariableName::VariableName as VariableName;
pub use dbVariableName::table_create_variable_name as table_create_variable_name;
pub use dbVariableName::insert_variable_name as insert_variable_name;
pub use dbVariableName::list_variable_name as list_variable_name;
pub use dbVariableName::variable_name_list as variable_name_list;
pub use dbVariableName::pk_variable_name_by_name as pk_variable_name_by_name;
pub use dbJobRequireVariable::JobRequireVariable as JobRequireVariable;
pub use dbJobRequireVariable::table_create_job_require_variable as table_create_job_require_variable;
pub use dbJobRequireVariable::insert_job_require_variable as insert_job_require_variable;
pub use dbJobRequireVariable::list_job_require_variable as list_job_require_variable;
pub use dbJobRequireVariable::job_require_variable_list as job_require_variable_list;
pub use dbJobRequireVariable::pk_job_require_variable_by_name as pk_job_require_variable_by_name;
pub use dbVariablePair::table_create_variable_pair as table_create_variable_pair;
pub use dbVariablePair::insert_variable_pair as insert_variable_pair;

pub use dbVariablePair::pk_variable_pair_by_name as pk_variable_pair_by_name;
pub use dbVariablePair::variable_pair_list as variable_pair_list;

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
    table_create_job_require_variable(&conn);
    table_create_require_variable_pair(&conn);
    return &newcon;
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

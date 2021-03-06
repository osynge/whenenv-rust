pub mod db_enviroment;
pub mod db_fs_dir;
pub mod db_fs_file;
pub mod db_job;
pub mod db_job_depend;
pub mod db_job_provide;
pub mod db_job_require_variable;
pub mod db_job_require_variable_pair;
pub mod db_provider;
pub mod db_session;
pub mod fs_dir_type;

pub use db_enviroment::insert_enviroment;
pub use db_enviroment::pk_enviroment_by_name;
pub use db_session::insert_session;
pub use db_session::pk_session_by_uuid;
use rusqlite::Connection;
use std::collections::HashSet;

pub use fs_dir_type::insert_fs_dir_type;
pub use fs_dir_type::list_fs_dir_type;
pub use fs_dir_type::pk_fs_dir_type_by_name;
pub use fs_dir_type::table_create_fs_dir_type;
pub use fs_dir_type::FsDirType;

pub use db_fs_dir::insert_fs_dir;
pub use db_fs_dir::list_fs_dir;
pub use db_fs_dir::list_fs_dir_by_all;
pub use db_fs_dir::table_create_fs_dir;
pub use db_fs_dir::FsDir;

pub use db_fs_file::insert_fs_file as dbFsFile;
pub use db_fs_file::insert_fs_file;
pub use db_fs_file::list_fs_file;
pub use db_fs_file::list_fs_file_type;
pub use db_fs_file::pk_fs_file_by_name;
pub use db_fs_file::table_create_fs_file;

pub use db_provider::insert_provider;
pub use db_provider::list_provider;
pub use db_provider::pk_provider_by_name;
pub use db_provider::table_create_provider;
pub use db_provider::Provider;

pub use db_job::insert_job;
pub use db_job::list_job;
pub use db_job::pk_job_by_name;
pub use db_job::table_create_job;
pub use db_job::Job;

pub use db_job_provide::insert_job_provide;
pub use db_job_provide::list_job_provide as list_job_type;
pub use db_job_provide::pk_job_provide_by_all;
pub use db_job_provide::table_create_job_provide;
pub use db_job_provide::JobProvide;

pub use db_job_depend::insert_job_depend;
pub use db_job_depend::list_job_depend;
pub use db_job_depend::pk_job_depend_by_all;
pub use db_job_depend::table_create_job_depend;
pub use db_job_depend::JobDepend;

pub use db_variable_name::insert_variable_name;
pub use db_variable_name::list_variable_name;
pub use db_variable_name::pk_variable_name_by_name;
pub use db_variable_name::table_create_variable_name;
pub use db_variable_name::variable_name_list;
pub use db_variable_name::VariableName;

pub use db_job_require_variable::insert_job_require_variable;
pub use db_job_require_variable::job_require_variable_list;
pub use db_job_require_variable::list_job_require_variable;
pub use db_job_require_variable::pk_job_require_variable_by_name;
pub use db_job_require_variable::table_create_job_require_variable;
pub use db_job_require_variable::JobRequireVariable;

pub use db_variable_pair::insert_variable_pair;
pub use db_variable_pair::pk_variable_pair_by_name;
pub use db_variable_pair::table_create_variable_pair;
pub use db_variable_pair::variable_pair_list;

pub use db_job_require_variable_pair::insert_job_require_variable_pair;
pub use db_job_require_variable_pair::job_require_variable_pair_list;
pub use db_job_require_variable_pair::list_job_require_variable_pair;
pub use db_job_require_variable_pair::pk_job_require_variable_pair_by_all;
pub use db_job_require_variable_pair::table_create_job_require_variable_pair;

pub fn connect() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", &[]).unwrap();
    return conn;
}

pub fn connect_file(filename: &str) -> Connection {
    let conn = Connection::open(filename).unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", &[]).unwrap();
    return conn;
}

fn list_tables(conn: &Connection) -> HashSet<String> {
    let mut output = HashSet::new();
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();
    //let wraped_table_iter = stmt.query_map(&[], |row| row.get::<int>(0));
    let wraped_table_iter = stmt.query_map(&[], |row| row.get::<_, String>(0));
    if wraped_table_iter.is_err() {
        return output;
    }
    let fs_file_iter = wraped_table_iter.unwrap();
    for person in fs_file_iter {
        let result = person.unwrap();
        let s = String::from(result);
        output.insert(s);
    }
    return output;
}

pub fn create_tables(conn: &Connection) -> &Connection {
    let tables_found = list_tables(&conn);
    if !tables_found.contains("FS_DIR_TYPE") {
        table_create_fs_dir_type(&conn);
    }
    if !tables_found.contains("FS_DIR") {
        table_create_fs_dir(&conn);
    }
    if !tables_found.contains("FS_FILE") {
        table_create_fs_file(&conn);
    }
    if !tables_found.contains("PROVIDER") {
        table_create_provider(&conn);
    }
    if !tables_found.contains("WHENENV_SESSION") {
        let _ = db_session::table_create_session(&conn);
    }
    if !tables_found.contains("JOB") {
        table_create_job(&conn);
    }
    if !tables_found.contains("JOBDEPEND") {
        table_create_job_depend(&conn);
    }
    if !tables_found.contains("JOBPROVIDE") {
        let _ = table_create_job_provide(&conn);
    }
    if !tables_found.contains("VARIABLE_NAME") {
        table_create_variable_name(&conn);
    }
    if !tables_found.contains("VARIABLE_PAIR") {
        table_create_variable_pair(&conn);
    }
    if !tables_found.contains("JOB_REQUIRE_VARIABLE") {
        table_create_job_require_variable(&conn);
    }
    if !tables_found.contains("JOB_REQUIRE_VALUE") {
        table_create_job_require_variable_pair(&conn);
    }
    if !tables_found.contains("WHENENV_ENVIROMENT") {
        db_enviroment::table_create_enviroment(&conn);
    }
    return &conn;
}

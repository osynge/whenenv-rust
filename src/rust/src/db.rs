use rusqlite::Connection;
use rusqlite::Error;
pub use dbSession::insert_session;
pub use dbSession::pk_session_by_uuid;
pub use dbEnviroment::pk_enviroment_by_name;
pub use dbEnviroment::insert_enviroment;
pub use dbFsDirType::FsDirType;
pub use dbFsDirType::table_create_fs_dir_type;
pub use dbFsDirType::insert_fs_dir_type;
pub use dbFsDirType::list_fs_dir_type;
pub use dbFsDirType::pk_fs_dir_type_by_name;
pub use dbFsDir::insert_fs_dir;
pub use dbFsDir::FsDir;
pub use dbFsDir::list_fs_dir;
pub use dbFsDir::table_create_fs_dir;
pub use dbFsDir::list_fs_dir_by_all;
pub use dbFsFile::list_fs_file_type;


pub use dbFsFile::insert_fs_file as dbFsFile;
pub use dbFsFile::insert_fs_file;
pub use dbFsFile::list_fs_file;
pub use dbFsFile::pk_fs_file_by_name;
pub use dbFsFile::table_create_fs_file;
pub use dbProvider::Provider;
pub use dbProvider::table_create_provider;
pub use dbProvider::insert_provider;
pub use dbProvider::list_provider;
pub use dbProvider::pk_provider_by_name;
pub use dbJob::Job;
pub use dbJob::table_create_job;
pub use dbJob::insert_job;
pub use dbJob::list_job;
pub use dbJob::pk_job_by_name;
pub use dbJobProvide::JobProvide;
pub use dbJobProvide::table_create_job_provide;
pub use dbJobProvide::insert_job_provide;
pub use dbJobProvide::list_job_provide as list_job_type;
pub use dbJobProvide::pk_job_provide_by_all;
pub use dbJobDepend::JobDepend;
pub use dbJobDepend::table_create_job_depend;
pub use dbJobDepend::insert_job_depend;
pub use dbJobDepend::pk_job_depend_by_all;
pub use dbJobDepend::list_job_depend;
pub use dbVariableName::VariableName;
pub use dbVariableName::table_create_variable_name;
pub use dbVariableName::insert_variable_name;
pub use dbVariableName::list_variable_name;
pub use dbVariableName::variable_name_list;
pub use dbVariableName::pk_variable_name_by_name;
pub use dbJobRequireVariable::JobRequireVariable;
pub use dbJobRequireVariable::table_create_job_require_variable as table_create_job_require_variable;
pub use dbJobRequireVariable::insert_job_require_variable;
pub use dbJobRequireVariable::list_job_require_variable;
pub use dbJobRequireVariable::job_require_variable_list;
pub use dbJobRequireVariable::pk_job_require_variable_by_name;
pub use dbVariablePair::table_create_variable_pair;
pub use dbVariablePair::insert_variable_pair;

pub use dbVariablePair::pk_variable_pair_by_name;
pub use dbVariablePair::variable_pair_list;

pub use dbJobRequireVariablePair::table_create_job_require_variable_pair as table_create_job_require_variable_pair;
pub use dbJobRequireVariablePair::insert_job_require_variable_pair as insert_job_require_variable_pair;
pub use dbJobRequireVariablePair::list_job_require_variable_pair;
pub use dbJobRequireVariablePair::job_require_variable_pair_list;
pub use dbJobRequireVariablePair::pk_job_require_variable_pair_by_all as pk_job_require_variable_pair_by_all;


use dbSession;
use dbEnviroment;

pub fn connect() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", &[]).unwrap();
    return conn;
}


pub fn create_tables(conn: &Connection) -> &Connection {
    dbSession::table_create_session(&conn);
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
    table_create_job_require_variable_pair(&conn);
    dbEnviroment::table_create_enviroment(&conn);
    return &newcon;
}

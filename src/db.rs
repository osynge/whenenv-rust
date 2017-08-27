use rusqlite::Connection;
use rusqlite::Error;

#[derive(Debug)]
pub struct FsDir {
    pub id: i32,
    pub name: String,
}


fn table_create_fs_dir(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE FS_DIR (
                  id              INTEGER PRIMARY KEY ASC,
                  name            TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}



#[derive(Debug)]
pub struct FsFile {
    pub id: i32,
    pub fk_fs_dir: i32,
    pub name: String,
}


fn table_create_fs_file(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE FS_FILE (
                  id                INTEGER PRIMARY KEY ASC,
                  fk_fs_dir         INTEGER NOT NULL,
                  name              TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}


#[derive(Debug)]
pub struct Provider {
    id: i32,
    name: String,
}


fn table_create_provider(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE PROVIDER (
                  id              INTEGER PRIMARY KEY ASC,
                  name            TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}

#[derive(Debug)]
struct Job {
    id: i32,
    name: String,
    shell: String,
}


fn table_create_job(conn: &Connection)  -> &Connection {
    conn.execute("CREATE TABLE JOB (
                  id              INTEGER PRIMARY KEY ASC,
                  name            TEXT NOT NULL UNIQUE,
                  shell           TEXT NOT NULL
                  )", &[]).unwrap();
    return conn;
}


#[derive(Debug)]
struct JobProvide {
    id: i32,
    job: i32,
    provider: i32,
}


fn table_create_job_provide(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE JOBPROVIDE (
                  id            INTEGER PRIMARY KEY ASC,
                  job           INTEGER,
                  provider      INTEGER,
                  FOREIGN KEY(job) REFERENCES JOB(id) ON UPDATE CASCADE
                  FOREIGN KEY(provider) REFERENCES PROVIDER(id) ON UPDATE CASCADE
                  )", &[]).unwrap();
    return conn;
}


#[derive(Debug)]
struct JobDepend {
    id: i32,
    job: i32,
    provider: i32,
    sq_order: i32,
}


fn table_create_job_depend(conn: &Connection)  -> &Connection  {
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


pub fn create_tables(conn: &Connection) -> &Connection  {
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
    return newcon2;
}

pub fn insert_fs_dir(conn: &Connection, name: String) {

    let me = FsDir {
        id: 0,
        name: name,
    };
    conn.execute("INSERT INTO FS_DIR (name)
                  VALUES (?1)",
                 &[&me.name]).unwrap();
}

pub fn list_fs_dir(conn: &Connection) -> Vec<FsDir> {
    let mut stmt = conn.prepare("SELECT id, name FROM FS_DIR").unwrap();
    let fs_dir_iter = stmt.query_map(&[], |row| {
        FsDir {
            id: row.get(0),
            name: row.get(1)
        }
    }).unwrap();
    let mut items = Vec::<FsDir>::new();
    for person in fs_dir_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn insert_fs_file(conn: &Connection, fk_fs_dir: i32, name: String) {

    let me = FsFile {
        id: 0,
        name: name,
        fk_fs_dir : fk_fs_dir,
    };
    conn.execute("INSERT INTO FS_FILE (name, fk_fs_dir)
                  VALUES (?1, ?2)",
                 &[&me.name, &me.fk_fs_dir]).unwrap();
}

pub fn list_fs_file(conn: &Connection) -> Vec<FsFile> {
    let mut stmt = conn.prepare("SELECT id, fk_fs_dir,name  FROM FS_FILE").unwrap();
    let fs_file_iter = stmt.query_map(&[], |row| {
        FsFile {
            id: row.get(0),
            fk_fs_dir: row.get(1),
            name: row.get(2),
        }
    }).unwrap();
    let mut items = Vec::<FsFile>::new();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn insert_provider(conn: &Connection, name: String) {

    let me = Provider {
        id: 0,
        name: name,
    };
    let george = conn.execute("INSERT INTO PROVIDER (name)
                  VALUES (?1)",
                 &[&me.name]);
    println!("Found provider {:?}", george.is_err());
    if george.is_err() {
        return;

    }
    george.unwrap();

}

pub fn list_provider(conn: &Connection)-> Vec<Provider> {
    let mut stmt = conn.prepare("SELECT id, name  FROM PROVIDER").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        Provider {
            id: row.get(0),
            name: row.get(1),
        }
    });
    let mut items = Vec::<Provider>::new();
    if wraped_fs_file_iter.is_err() {
        return items;

    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn insert_job(conn: &Connection, name: String, shell: String) {

    let me = Job {
        id: 0,
        name: name,
        shell: shell,
    };
    conn.execute("INSERT INTO JOB (name, shell)
                  VALUES (?1, ?2)",
                 &[&me.name, &me.shell]).unwrap();
}


pub fn insert_job_provide(conn: &Connection, job: i32, provider: i32) {

    let me = JobProvide {
        id: 0,
        job: job,
        provider: provider,
    };
    conn.execute("INSERT INTO JOBPROVIDE (job, provider)
                  VALUES (?1, ?2)",
                 &[&me.job, &me.provider]).unwrap();
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


pub fn provider_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name FROM PROVIDER").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        Provider {
            id: row.get(0),
            name: row.get(1)
        }
    }).unwrap();

    for person in person_iter {
        println!("Found provider {:?}", person.unwrap());
    }
}

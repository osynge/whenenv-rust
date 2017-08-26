use rusqlite::Connection;



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
    conn.execute("CREATE TABLE require_variable (
                  id            INTEGER PRIMARY KEY ASC,
                  job           INTEGER,
                  variable_id   INTEGER
                  )", &[]).unwrap();
}


#[derive(Debug)]
struct VariablePair {
    id: i32,
    variable_id: i32,
    variable_value: String
}




fn table_create_variable_pair(conn: &Connection)  {
    conn.execute("CREATE TABLE VariablePair (
                  id            INTEGER PRIMARY KEY ASC,
                  variable_id   INTEGER,
                  variable_value  TEXT
                  )", &[]).unwrap();
    
}


#[derive(Debug)]
struct JobRequireVariablePair {
    id: i32,
    variable_pair: i32,
}




fn table_create_require_variable_pair(conn: &Connection)  -> &Connection  {
    conn.execute("CREATE TABLE VariableId (
                  id            INTEGER PRIMARY KEY ASC,
                  job           INTEGER,
                  variable_pair INTEGER
                  )", &[]).unwrap();
    return conn;
}




pub fn connect() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", &[]).unwrap();
    return conn;
}


pub fn create_tables(conn: &Connection) -> &Connection  {
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


pub fn insert_provider(conn: &Connection, name: String) {

    let me = Provider {
        id: 0,
        name: name,
    };
    conn.execute("INSERT INTO PROVIDER (name)
                  VALUES (?1)",
                 &[&me.name]).unwrap();
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

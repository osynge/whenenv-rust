use rusqlite::Connection;
use std::result;

#[derive(Debug)]
pub struct VariablePair {
    id: i32,
    variable_id: i32,
    variable_value: String
}



pub fn table_create_variable_pair(conn: &Connection) {
    let load_table = conn.execute("CREATE TABLE  VARIABLE_PAIR (
                  id            INTEGER PRIMARY KEY ASC,
                  variable_id   INTEGER NOT NULL,
                  variable_value  TEXT,
                  FOREIGN KEY(variable_id) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )", &[]);
    if load_table.is_err() {
        println!("table_create_job Failed {:?}", load_table);
        return;
    }
    load_table.unwrap();
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

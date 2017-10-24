use rusqlite::Connection;
use std::result;

#[derive(Debug)]
pub struct VariablePair {
    id: i32,
    fk_variable: i32,
    variable_value: String
}


pub fn table_create_variable_pair(conn: &Connection) {
    let load_table = conn.execute("CREATE TABLE  VARIABLE_PAIR (
                  id            INTEGER PRIMARY KEY ASC,
                  fk_variable   INTEGER NOT NULL,
                  variable_value  TEXT,
                  FOREIGN KEY(fk_variable) REFERENCES VARIABLE_NAME(id) ON UPDATE CASCADE
                  )", &[]);
    if load_table.is_err() {
        println!("table_create_job Failed {:?}", load_table);
        return;
    }
    load_table.unwrap();
}


pub fn insert_variable_pair(conn: &Connection, fk_variable :&i32, name: &str) -> Result<i32, &'static str> {
    let bill = fk_variable;
    let mut john = String::from(name);
    let me = VariablePair {
        id : 0,
        fk_variable: bill.clone(),
        variable_value: john,
    };
    let variable_pair_instance = conn.execute("INSERT INTO VARIABLE_PAIR (fk_variable, variable_value)
                  VALUES (?1 ?2)",
                 &[&me.fk_variable, &me.variable_value]);
    if variable_pair_instance.is_err() {
        return Err("Insert failed");
    }
    variable_pair_instance.unwrap();
    return Ok(0);
}


pub fn list_variable_pair(conn: &Connection)-> Vec<VariablePair> {
    let mut stmt = conn.prepare("SELECT id, fk_variable, variable_value  FROM VARIABLE_PAIR").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        VariablePair {
            id: row.get(0),
            fk_variable: row.get(1),
            variable_value: row.get(2),
        }
    });
    let mut items = Vec::<VariablePair>::new();
    if wraped_fs_file_iter.is_err() {
        return items;

    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn variable_pair_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, fk_variable, variable_value FROM VARIABLE_PAIR").unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        VariablePair {
            id: row.get(0),
            fk_variable: row.get(1),
            variable_value: row.get(2),
        }
    }).unwrap();

    for person in person_iter {
        println!("Found variable_pair {:?}", person.unwrap());
    }
}


pub fn pk_variable_pair_by_name(conn: &Connection, fk_variable :&i32, name: &str) -> Result<i32, &'static str>{
    let mut output = 0;
    let bill = String::from(name);
    let mut stmt = conn.prepare("SELECT VARIABLE_PAIR.id, VARIABLE_PAIR.fk_variable, VARIABLE_PAIR.variable_value  FROM VARIABLE_PAIR
		WHERE VARIABLE_PAIR.fk_variable = ?1 AND VARIABLE_PAIR.variable_value = ?2").unwrap();
    let variable_pair_iter = stmt.query_map(&[fk_variable,&bill], |row| {
        VariablePair {
            id: row.get(0),
            fk_variable: row.get(1),
            variable_value: row.get(2),
        }
    });
    if variable_pair_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = variable_pair_iter.unwrap();
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

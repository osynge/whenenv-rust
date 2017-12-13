use rusqlite::Connection;


#[derive(Debug)]
pub struct VariableName {
    pub id: i32,
    pub name: String,
}



pub fn table_create_variable_name(conn: &Connection) -> &Connection {
    conn.execute(
        "CREATE TABLE VARIABLE_NAME (
                  id            INTEGER PRIMARY KEY ASC,
                  name      TEXT NOT NULL UNIQUE
                  )",
        &[],
    ).unwrap();
    return conn;
}


pub fn insert_variable_name(conn: &Connection, name: &String) -> Result<i32, &'static str> {
    let me = VariableName {
        id: 0,
        name: name.clone(),
    };
    let variable_name_instance = conn.execute(
        "INSERT INTO VARIABLE_NAME (name)
                  VALUES (?1)",
        &[&me.name],
    );
    if variable_name_instance.is_err() {
        return Err("Insert failed");
    }
    variable_name_instance.unwrap();
    return Ok(0);
}


pub fn list_variable_name(conn: &Connection) -> Vec<VariableName> {
    let mut stmt = conn.prepare("SELECT id, name  FROM VARIABLE_NAME").unwrap();
    let wraped_fs_file_iter = stmt.query_map(&[], |row| {
        VariableName {
            id: row.get(0),
            name: row.get(1),
        }
    });
    let mut items = Vec::<VariableName>::new();
    if wraped_fs_file_iter.is_err() {
        return items;

    }
    let fs_file_iter = wraped_fs_file_iter.unwrap();
    for person in fs_file_iter {

        items.push(person.unwrap());
    }
    return items;
}


pub fn variable_name_list(conn: &Connection) {
    let mut stmt = conn.prepare("SELECT id, name, fk_provider, sq_order FROM VARIABLE_NAME")
        .unwrap();
    let person_iter = stmt.query_map(&[], |row| {
        VariableName {
            id: row.get(0),
            name: row.get(1),
        }
    }).unwrap();

    for person in person_iter {
        info!("Found variable_name {:?}", person.unwrap());
    }
}

pub fn pk_variable_name_by_name(conn: &Connection, name: &str) -> Result<i32, &'static str> {
    let mut output = 0;
    let bill = String::from(name);
    let mut stmt = conn.prepare("SELECT id, name  FROM VARIABLE_NAME WHERE name = ?1")
        .unwrap();
    let variable_name_iter = stmt.query_map(&[&name], |row| {
        VariableName {
            id: row.get(0),
            name: row.get(1),
        }
    });
    if variable_name_iter.is_err() {
        return Err("Insert failed dfdf");
    }
    let result = variable_name_iter.unwrap();
    let mut found = 0;
    for person in result {
        let bill = person.unwrap();
        output = bill.id;
        found = 1;
    }
    if found != 0 {
        return Ok(output);
    }
    return Err("None found");
}

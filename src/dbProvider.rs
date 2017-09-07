use rusqlite::Connection;
#[derive(Debug)]
pub struct Provider {
    id: i32,
    name: String,
}


pub fn table_create_provider(conn: &Connection) -> &Connection {
    conn.execute("CREATE TABLE PROVIDER (
                  id              INTEGER PRIMARY KEY ASC,
                  name            TEXT NOT NULL UNIQUE
                  )", &[]).unwrap();
    return conn;
}

pub fn insert_provider(conn: &Connection, name: String) {

    let me = Provider {
        id: 0,
        name: name,
    };
    let provider_instance = conn.execute("INSERT INTO PROVIDER (name)
                  VALUES (?1)",
                 &[&me.name]);
    if provider_instance.is_err() {
        return;
    }
    provider_instance.unwrap();

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

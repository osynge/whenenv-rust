use std::collections::BTreeMap;
use rusqlite::Connection;
use db_variable_name;
use dbJob;
use dbFsFile;
use serde_yaml;
use std::collections::HashSet;

fn job_provide_list_by_job(conn: &Connection) -> Vec<db_variable_name::VariableName> {
    let mut items = Vec::<db_variable_name::VariableName>::new();
    let provide_list_prep_rc = conn.prepare(
        "SELECT DISTINCT
            VARIABLE_NAME.id,
            VARIABLE_NAME.name
            FROM JOBPROVIDE , VARIABLE_NAME
        WHERE
            VARIABLE_NAME.id = JOBPROVIDE.fk_variable
            order by VARIABLE_NAME.name",
    );
    if provide_list_prep_rc.is_err() {
        error!("SQL issue in job_provide_list");
        return items;
    }
    let mut provide_list_prep = provide_list_prep_rc.unwrap();

    let result_row = provide_list_prep.query(&[]);
    let mut rox = result_row.unwrap();

    while let Some(row_query) = rox.next() {
        let row = row_query.unwrap();
        let bill = db_variable_name::VariableName {
            id: row.get(0),
            name: row.get(1),
        };
        debug!("bill:{}", bill.id);
        items.push(bill);
    }
    return items;
}

fn job_depend_list_by_job(conn: &Connection) -> Vec<db_variable_name::VariableName> {
    let mut items = Vec::<db_variable_name::VariableName>::new();
    let provide_list_prep_rc = conn.prepare(
        "SELECT DISTINCT
            VARIABLE_NAME.id,
            VARIABLE_NAME.name
            FROM JOBDEPEND , VARIABLE_NAME
        WHERE
            VARIABLE_NAME.id = JOBDEPEND.fk_variable
            order by VARIABLE_NAME.name",
    );
    if provide_list_prep_rc.is_err() {
        error!("SQL issue in job_provide_list");
        return items;
    }
    let mut provide_list_prep = provide_list_prep_rc.unwrap();

    let result_row = provide_list_prep.query(&[]);
    let mut rox = result_row.unwrap();

    while let Some(row_query) = rox.next() {
        let row = row_query.unwrap();
        let bill = db_variable_name::VariableName {
            id: row.get(0),
            name: row.get(1),
        };
        debug!("bill:{}", bill.id);
        items.push(bill);
    }
    return items;
}

fn job_targets_list_by_job(conn: &Connection) -> Vec<db_variable_name::VariableName> {
    let mut items = Vec::<db_variable_name::VariableName>::new();

    let mut hs_depend: HashSet<i32> = HashSet::new();
    let list_dep = job_depend_list_by_job(&conn);
    for item in list_dep {
        hs_depend.insert(item.id);
    }
    for item in job_provide_list_by_job(&conn) {
        let foo = hs_depend.contains(&item.id);
        if foo == false {
            items.push(item)
        }
    }
    return items;
}

fn list_job(conn: &Connection, fk_variable_name: &i32) -> Vec<dbJob::Job> {
    let mut items = Vec::<dbJob::Job>::new();
    let provide_list_prep_rc = conn.prepare(
        "SELECT DISTINCT
            JOB.id,
            JOB.name,
            JOB.fk_file
            FROM JOB, JOBPROVIDE , VARIABLE_NAME
        WHERE
            JOB.id = JOBPROVIDE.fk_job
            AND
            VARIABLE_NAME.id = JOBPROVIDE.fk_variable
            AND
            VARIABLE_NAME.id = ?1
            order by JOB.name",
    );
    if provide_list_prep_rc.is_err() {
        error!("SQL issue in job_provide_list");
        return items;
    }
    let mut provide_list_prep = provide_list_prep_rc.unwrap();

    let result_row = provide_list_prep.query(&[fk_variable_name]);
    let mut rox = result_row.unwrap();

    while let Some(row_query) = rox.next() {
        let row = row_query.unwrap();
        let bill = dbJob::Job {
            id: row.get(0),
            name: row.get(1),
            fk_file: row.get(2),
        };
        debug!("bill:{}", bill.id);
        items.push(bill);
    }
    return items;
}

fn list_file(conn: &Connection, fk_variable_name: &i32) -> Vec<dbFsFile::FsFile> {
    let mut items = Vec::<dbFsFile::FsFile>::new();
    let provide_list_prep_rc = conn.prepare(
        "SELECT DISTINCT
            FS_FILE.id,
            FS_FILE.fk_fs_dir,
            FS_FILE.name
        FROM
            FS_FILE
        WHERE
            FS_FILE.id = ?1
            order by FS_FILE.name",
    );
    if provide_list_prep_rc.is_err() {
        error!("SQL issue in job_provide_list");
        return items;
    }
    let mut provide_list_prep = provide_list_prep_rc.unwrap();

    let result_row = provide_list_prep.query(&[fk_variable_name]);
    let mut rox = result_row.unwrap();

    while let Some(row_query) = rox.next() {
        let row = row_query.unwrap();
        let bill = dbFsFile::FsFile {
            id: row.get(0),
            fk_fs_dir: row.get(1),
            name: row.get(2),
        };
        debug!("bill:{}", bill.id);
        items.push(bill);
    }
    return items;
}

fn shared_foo(conn: &Connection, variables: &Vec<db_variable_name::VariableName>) -> String {
    let mut map = BTreeMap::new();
    for var_name in variables {
        let mut v_new = Vec::<BTreeMap<&str, String>>::new();
        let doop = list_job(&conn, &var_name.id);
        for item in doop {
            let mut map2 = BTreeMap::new();
            map2.insert("name", item.name.to_string());
            let woop = list_file(&conn, &item.fk_file);
            for op in woop {
                map2.insert("file_name", op.name.to_string());
            }
            v_new.push(map2);
        }
        map.insert(var_name.name.to_string(), v_new);
    }
    return serde_yaml::to_string(&map).unwrap();
}

pub fn process_list_provides(conn: &Connection) {
    let henry = job_provide_list_by_job(&conn);
    println!("{}", shared_foo(&conn, &henry));
}

pub fn process_list_targets(conn: &Connection) {
    let henry = job_targets_list_by_job(&conn);
    println!("{}", shared_foo(&conn, &henry));
}

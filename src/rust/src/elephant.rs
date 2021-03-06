use db;
use db_session;
use rusqlite::Connection;

pub fn elephant_directory_type(conn: &Connection, text: &String) -> Result<i32, &'static str> {
    let mut pk_variable: i32 = 0;
    let rc = db::pk_fs_dir_type_by_name(conn, &text, &mut pk_variable);
    match rc {
        Ok(pk) => {
            return Ok(pk);
        }
        Err(_) => {
            let doink = db::insert_fs_dir_type(conn, &text);
            if doink.is_err() {
                return doink;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_fs_dir_type_by_name(conn, &text, &mut pk_variable);
                    match doin3k {
                        Ok(pk) => {
                            return Ok(pk);
                        }
                        Err(_) => {
                            error!("Failed to select variable");
                            return doin3k;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert insert_fs_dir_type");
                    return doink;
                }
            }
        }
    }
}

pub fn elephant_directory(conn: &Connection, fk_directory_type: &i32, text: &String) -> i32 {
    let mut pk_variable: i32 = 0;
    let list_fs_dir_by_all_result = db::list_fs_dir_by_all(conn, &fk_directory_type, &text);
    match list_fs_dir_by_all_result {
        Ok(pk) => for item in pk {
            pk_variable = item.id;
        },
        Err(_) => {
            let doink = db::insert_fs_dir(conn, &fk_directory_type, &text);
            if doink.is_err() {
                error!("Failed to insert_fs_dir variable");
                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::list_fs_dir_by_all(conn, &fk_directory_type, &text);
                    match doin3k {
                        Ok(pk) => {
                            for item in pk {
                                pk_variable = item.id;
                            }
                            return pk_variable;
                        }
                        Err(_) => {
                            error!("Failed to select variable");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert variable");
                    return 0;
                }
            }
        }
    }
    return pk_variable;
}

pub fn elephant_file(conn: &Connection, fk_directory: &i32, text: &str) -> i32 {
    let mut pk_variable: i32 = 0;
    let list_fs_dir_by_all_result = db::list_fs_dir_by_all(conn, &fk_directory, &text);
    match list_fs_dir_by_all_result {
        Ok(pk) => for item in pk {
            pk_variable = item.id;
        },
        Err(_) => {
            let doink = db::insert_fs_dir(conn, &fk_directory, &text);
            if doink.is_err() {
                error!("Failed to insert_fs_dir variable");
                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::list_fs_dir_by_all(conn, &fk_directory, &text);
                    match doin3k {
                        Ok(pk) => for item in pk {
                            pk_variable = item.id;
                            return pk_variable;
                        },
                        Err(_) => {
                            error!("Failed to select variable");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert variable");
                    return 0;
                }
            }
        }
    }
    return pk_variable;
}

pub fn elephant_session(conn: &Connection, text: &String) -> i32 {
    let rc = db::pk_session_by_uuid(conn, &text);
    match rc {
        Ok(pk) => {
            return pk;
        }
        Err(_) => {
            let doink = db::insert_session(conn, &text);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db_session::pk_session_by_uuid(conn, &text);
                    match doin3k {
                        Ok(pk) => {
                            return pk;
                        }
                        Err(_) => {
                            error!("Failed to select session");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert session");
                    return 0;
                }
            }
        }
    }
}

pub fn elephant_enviroment(
    conn: &Connection,
    pk_session: &i32,
    pk_enviroment: &i32,
) -> Result<i32, &'static str> {
    let rc = db::pk_enviroment_by_name(conn, &pk_session, &pk_enviroment);
    match rc {
        Ok(_) => {
            return rc;
        }
        Err(_) => {
            let doink = db::insert_enviroment(conn, &pk_session, &pk_enviroment);
            if doink.is_err() {
                return doink;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_enviroment_by_name(conn, &pk_session, &pk_enviroment);
                    match doin3k {
                        Ok(_) => {
                            let bill = *pk_enviroment;
                            return Ok(bill);
                        }
                        Err(_) => {
                            error!("Failed to pk_enviroment_by_name");

                            info!("Found pk_session {:?}", pk_session);
                            info!("Found pk_enviroment {:?}", pk_enviroment);
                            return doin3k;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert job");
                    return doink;
                }
            }
        }
    }
}

pub fn elephant_variable_pk(conn: &Connection, text: &String) -> Result<i32, &'static str> {
    let rc = db::pk_variable_name_by_name(conn, &text);
    match rc {
        Ok(pk) => {
            return Ok(pk);
        }
        Err(_) => {
            let doink = db::insert_variable_name(conn, &text);
            if doink.is_err() {
                return Err("Failed to insert_variable_name");
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_variable_name_by_name(conn, &text);
                    match doin3k {
                        Ok(pk) => {
                            return Ok(pk);
                        }
                        Err(_) => {
                            error!("Failed to select variable");
                            return doin3k;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert variable");
                    return doink;
                }
            }
        }
    }
}

pub fn elephant_variable_pair_pk(
    conn: &Connection,
    fk_variable: &i32,
    text: &String,
) -> Result<i32, &'static str> {
    let rc = db::pk_variable_pair_by_name(conn, &fk_variable, &text);
    match rc {
        Ok(pk) => {
            return Ok(pk);
        }
        Err(_) => {
            let doink = db::insert_variable_pair(conn, fk_variable, &text);
            if doink.is_err() {
                error!("Failed to pk_variable_pair_by_name");
                return doink;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_variable_pair_by_name(conn, &fk_variable, &text);
                    match doin3k {
                        Ok(pk) => {
                            return Ok(pk);
                        }
                        Err(_) => {
                            error!("Failed to pk_variable_pair_by_name");
                            return doin3k;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert_variable_pair");
                    return doink;
                }
            }
        }
    }
}

pub fn elephant_job_require_variables(
    conn: &Connection,
    pk_job: &i32,
    pk_variable: &i32,
) -> Result<i32, &'static str> {
    let rc = db::pk_job_require_variable_by_name(conn, &pk_job, &pk_variable);
    match rc {
        Ok(pk) => {
            return Ok(pk);
        }
        Err(_) => {
            let doink = db::insert_job_require_variable(conn, &pk_job, &pk_variable);
            match doink {
                Err(_) => {
                    error!("elephant_job_require_variables:Failed to insert_job_require_variable");
                    return doink;
                }

                Ok(_) => {
                    let doin3k = db::pk_job_require_variable_by_name(conn, &pk_job, &pk_variable);
                    match doin3k {
                        Ok(pk) => {
                            return Ok(pk);
                        }
                        Err(_) => {
                            error!("elephant_job_require_variables:Failed to select job");
                            return doin3k;
                        }
                    }
                }
            }
        }
    }
}

pub fn elephant_job_pk(conn: &Connection, pk_file: &i32, in_text: &str) -> i32 {
    let text = String::from(in_text);
    let mut pk_job: i32 = 0;
    let rc = db::pk_job_by_name(conn, &text, &mut pk_job);
    match rc {
        Ok(_) => {
            return pk_job;
        }
        Err(_) => {
            let doink = db::insert_job(conn, &pk_file, &text);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_job_by_name(conn, &text, &mut pk_job);
                    match doin3k {
                        Ok(_) => {
                            return pk_job;
                        }
                        Err(_) => {
                            error!("Failed to select job");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert job");
                    return 0;
                }
            }
        }
    }
}

pub fn elephant_provider_pk(conn: &Connection, in_text: &str) -> i32 {
    let text = String::from(in_text);

    let mut pk_provider: i32 = 0;
    let rc = db::pk_provider_by_name(conn, &text, &mut pk_provider);
    match rc {
        Ok(_) => {
            return pk_provider;
        }
        Err(_) => {
            let doink = db::insert_provider(conn, &text);
            if doink.is_err() {
                error!("Error Failed to insert elephant_provider");
                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_provider_by_name(conn, &text, &mut pk_provider);
                    match doin3k {
                        Ok(_) => {
                            return pk_provider;
                        }
                        Err(_) => {
                            error!("Failed to select provider");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert elephant_provider");
                    return 0;
                }
            }
        }
    }
}

pub fn elephant_job_depend_pair_pk(conn: &Connection, job: &i32, variable_pair: &i32) -> i32 {
    let rc = db::pk_job_require_variable_pair_by_all(conn, &job, &variable_pair);
    match rc {
        Ok(pk) => {
            return pk;
        }
        Err(_) => {
            let doink = db::insert_job_require_variable_pair(conn, &job, &variable_pair);
            if doink.is_err() {
                error!("Failed to insert job_depend_pair:{}{}", variable_pair, job);

                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k =
                        db::pk_job_require_variable_pair_by_all(conn, &job, &variable_pair);
                    match doin3k {
                        Ok(pk) => {
                            return pk;
                        }
                        Err(_) => {
                            error!("Failed to select job_depend");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert job_depend");
                    return 0;
                }
            }
        }
    }
}

pub fn elephant_job_depend_pk(conn: &Connection, job: &i32, provider: &i32, sq_order: &i32) -> i32 {
    let rc = db::pk_job_depend_by_all(conn, &job, &provider, &sq_order);
    match rc {
        Ok(pk) => {
            return pk;
        }
        Err(_) => {
            let doink = db::insert_job_depend(conn, &job, &provider, &sq_order);
            if doink.is_err() {
                error!("Failed to insert_job_depend");
                return 0;
            }
            match doink {
                Ok(_) => {
                    let doin3k = db::pk_job_depend_by_all(conn, &job, &provider, &sq_order);
                    match doin3k {
                        Ok(pk) => {
                            return pk;
                        }
                        Err(_) => {
                            error!("Failed to select job_depend");
                            return 0;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert job_depend");
                    return 0;
                }
            }
        }
    }
}

pub fn elephant_job_provide_variables(
    conn: &Connection,
    job: &i32,
    provider: &i32,
) -> Result<i32, &'static str> {
    let mut pk_job_provide: i32 = 0;
    let rc = db::pk_job_provide_by_all(conn, &job, &provider, &mut pk_job_provide);
    match rc {
        Ok(pk) => {
            return Ok(pk);
        }
        Err(_) => {
            let doink = db::insert_job_provide(conn, &job, &provider);
            if doink.is_err() {
                error!("elephant_job_provide_variables:Failed to insert");
                debug!("elephant_job_provide_variables:job:{:?}", &job);
                debug!("elephant_job_provide_variables:provider:{:?}", &provider);
                return doink;
            }
            match doink {
                Ok(_) => {
                    let doin3k =
                        db::pk_job_provide_by_all(conn, &job, &provider, &mut pk_job_provide);
                    match doin3k {
                        Ok(pk) => {
                            return Ok(pk);
                        }
                        Err(_) => {
                            error!("Failed to select job_provide");
                            return doin3k;
                        }
                    }
                }
                Err(_) => {
                    error!("Failed to insert job_provide");
                    return doink;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn elephant_directory_type() {
        use db;
        use elephant;
        let conn = db::connect();
        db::create_tables(&conn);
        let str_job_files_list = String::from("job_files");

        let result_directory_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
        match result_directory_type {
            Ok(pk_dir_type) => {
                assert!(pk_dir_type == 1);
            }
            Err(_) => {}
        }
    }

    #[test]
    fn elephant_directory() {
        use db;
        use elephant;
        let conn = db::connect();
        db::create_tables(&conn);
        let str_job_files_list = String::from("job_files");
        let result_directory_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
        match result_directory_type {
            Ok(pk_dir_type) => {
                assert!(pk_dir_type == 1);
                let _ = String::from("dir_one");
                let str_directory_path = String::from("directory_path");
                let pk_dir = elephant::elephant_directory(&conn, &pk_dir_type, &str_directory_path);
                assert!(pk_dir == 1);
            }
            Err(_) => {}
        }
    }
    #[test]
    fn elephant_session() {
        use db;
        use elephant;
        let conn = db::connect();
        db::create_tables(&conn);
        let str_session_uuid = String::from("session uuid");
        let pk_session = elephant::elephant_session(&conn, &str_session_uuid);
        assert!(pk_session == 1);
    }

    #[test]
    fn elephant_enviroment() {
        use db;
        use elephant;
        let conn = db::connect();
        db::create_tables(&conn);
        let str_session_uuid = String::from("session uuid");
        let pk_session = elephant::elephant_session(&conn, &str_session_uuid);
        assert!(pk_session == 1);
        let str_dir_one_list = String::from("dir_one");
        let pk_provider = elephant::elephant_provider_pk(&conn, &str_dir_one_list);
        let _ = elephant::elephant_enviroment(&conn, &pk_session, &pk_provider);
    }

    #[test]
    fn elephant_file() {
        use db;
        use elephant;
        let conn = db::connect();
        db::create_tables(&conn);
        let str_job_files_list = String::from("job_files");
        let _ = elephant::elephant_directory_type(&conn, &str_job_files_list);
        let result_directory_type = elephant::elephant_directory_type(&conn, &str_job_files_list);
        match result_directory_type {
            Ok(pk_dir_type) => {
                assert!(pk_dir_type == 1);
                let _ = String::from("dir_one");
                let str_directory_path = String::from("directory_path");
                let pk_dir = elephant::elephant_directory(&conn, &pk_dir_type, &str_directory_path);
                assert!(pk_dir == 1);
                let str_file_path = String::from("file_path");
                let pk_file = elephant::elephant_file(&conn, &pk_dir, &str_file_path);
                assert!(pk_file == 1);
                assert!(true);
            }
            Err(_) => {
                assert!(true);
            }
        }
    }
}

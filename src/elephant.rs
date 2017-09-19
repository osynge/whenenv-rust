use rusqlite::Connection;
use db;

pub fn elephant_variable_pk(conn: &Connection, text :&String) -> i32 {
    let mut pk_variable :i32 = 0;
    let rc = db::pk_variable_name_by_name(conn, &text, &mut pk_variable);
    match rc {
        Ok(pk) => {
            return pk_variable;
        }
        Err(_) => {
            let doink = db::insert_variable_name(conn, &text);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_variable_name_by_name(conn, &text, &mut pk_variable);
                    match doin3k {
                        Ok(pk) => {
                            return pk_variable;
                            }
                        Err(_) => {
                                println!("Failed to select variable");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert variable");
                    return 0;
                }
            }

        }
    }
    return pk_variable;
}


pub fn elephant_job_require_variables(conn: &Connection, pk_job :&i32, pk_variable: &i32) -> i32 {
    let mut pk_job_require_variables :i32 = 0;
    let rc = db::pk_job_require_variable_by_name(conn, &pk_job, &pk_variable, &mut pk_job_require_variables);
    match rc {
        Ok(pk) => {
            return pk_job_require_variables;
        }
        Err(_) => {
            let doink = db::insert_job_require_variable(conn, &pk_job, &pk_variable);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_require_variable_by_name(conn, &pk_job, &pk_variable, &mut pk_job_require_variables);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job_require_variables;
                            }
                        Err(_) => {
                                println!("Failed to select job");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job");
                    return 0;
                }
            }

        }
    }
    return pk_job_require_variables;
}




pub fn elephant_job_pk(conn: &Connection, pk_file :&i32, in_text :&str) -> i32 {
    let text = String::from(in_text);
    let mut pk_job :i32 = 0;
    let rc = db::pk_job_by_name(conn, &text, &mut pk_job);
    match rc {
        Ok(pk) => {
            return pk_job;
        }
        Err(_) => {
            let doink = db::insert_job(conn, &pk_file, &text);
            if doink.is_err() {
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_by_name(conn, &text, &mut pk_job);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job;
                            }
                        Err(_) => {
                                println!("Failed to select job");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job");
                    return 0;
                }
            }

        }
    }
    return pk_job;
}


pub fn elephant_provider_pk(conn: &Connection, in_text :&str) -> i32 {
    let text = String::from(in_text);

    let mut pk_provider :i32 = 0;
    let rc = db::pk_provider_by_name(conn, &text, &mut pk_provider);
    match rc {
        Ok(pk) => {
            return pk_provider;
        }
        Err(_) => {
            let doink = db::insert_provider(conn, &text);
            if doink.is_err() {
                println!("Failed to insert");
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_provider_by_name(conn, &text, &mut pk_provider);
                    match doin3k {
                        Ok(pk) => {
                            return pk_provider;
                            }
                        Err(_) => {
                                println!("Failed to select provider");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert provider");
                    return 0;
                }
            }
        }
    }
    return pk_provider;
}


pub fn elephant_job_depend_pk(conn: &Connection, job: i32, provider: i32, sq_order: i32) -> i32 {
    let mut pk_job_depend :i32 = 0;
    let rc = db::pk_job_depend_by_all(conn, &job, &provider, &sq_order, &mut pk_job_depend);
    match rc {
        Ok(pk) => {
            return pk_job_depend;
        }
        Err(_) => {
            let doink = db::insert_job_depend(conn, &job, &provider, &sq_order);
            if doink.is_err() {
                println!("Failed to insert");
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_depend_by_all(conn, &job, &provider, &sq_order, &mut pk_job_depend);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job_depend;
                            }
                        Err(_) => {
                                println!("Failed to select job_depend");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job_depend");
                    return 0;
                }
            }
        }
    }
    return pk_job_depend;
}

pub fn elephant_job_provide_variables(conn: &Connection, job: i32, provider: i32) -> i32 {
    let mut pk_job_provide :i32 = 0;
    let rc = db::pk_job_provide_by_all(conn, &job, &provider,  &mut pk_job_provide);
    match rc {
        Ok(pk) => {
            return pk_job_provide;
        }
        Err(_) => {
            let doink = db::insert_job_provide(conn, &job, &provider);
            if doink.is_err() {
                println!("Failed to insert");
                return 0;
            }
            match doink {
                Ok(pk) => {
                    let doin3k = db::pk_job_provide_by_all(conn, &job, &provider, &mut pk_job_provide);
                    match doin3k {
                        Ok(pk) => {
                            return pk_job_provide;
                            }
                        Err(_) => {
                                println!("Failed to select job_provide");
                                return 0;
                            }
                        }
                    }
                Err(_) => {
                    println!("Failed to insert job_provide");
                    return 0;
                }
            }
        }
    }
    return pk_job_provide;
}

use cfg;
use clap::ArgMatches;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

pub fn actions_get(matches: &ArgMatches) -> HashSet<String> {
    let mut hs_actions = HashSet::<String>::new();

    if let Some(_) = matches.values_of("list-provides") {
        let bill = String::from("load-jobs");
        hs_actions.insert(bill);
        let list_provides = String::from("list-provides");
        hs_actions.insert(list_provides);
    }
    if let Some(_) = matches.values_of("list-target") {
        let bill = String::from("load-jobs");
        hs_actions.insert(bill);
        let list_provides = String::from("list-target");
        hs_actions.insert(list_provides);
    }
    if let Some(_) = matches.values_of("session") {
        let bill = String::from("load-jobs");
        hs_actions.insert(bill);
        let str_load_scripts = String::from("load-scripts");
        hs_actions.insert(str_load_scripts);
        let session = String::from("session");
        hs_actions.insert(session);
    }
    return hs_actions;
}

pub fn cfg_actions_update_clap(freds: &Arc<Mutex<cfg::Config>>, matches: &ArgMatches) {
    let mut fred = freds.lock().unwrap();
    if let Some(_) = matches.values_of("list-provides") {
        fred.actions.insert(cfg::Action::DbConnect);
        fred.actions.insert(cfg::Action::LoadJobs);
        fred.actions.insert(cfg::Action::ListProvides);
    }
    if let Some(_) = matches.values_of("list-target") {
        fred.actions.insert(cfg::Action::DbConnect);
        fred.actions.insert(cfg::Action::LoadJobs);
        fred.actions.insert(cfg::Action::ListTarget);
    }
    if let Some(_) = matches.values_of("session") {
        fred.actions.insert(cfg::Action::DbConnect);
        fred.actions.insert(cfg::Action::LoadJobs);
        fred.actions.insert(cfg::Action::LoadScripts);
        fred.actions.insert(cfg::Action::SessionStart);
    }
    if let Some(_) = matches.values_of("request-run") {
        fred.actions.insert(cfg::Action::DbConnect);
        fred.actions.insert(cfg::Action::LoadJobs);
        fred.actions.insert(cfg::Action::LoadScripts);
        fred.actions.insert(cfg::Action::SessionStart);
        fred.actions.insert(cfg::Action::RequestRun);
    }
    if let Some(jobs_dir_it) = matches.values_of("dir-jobs") {
        for jobs_dir in jobs_dir_it {
            fred.path_jobs.push(jobs_dir.to_string());
        }
    }
    if let Some(jobs_dir_it) = matches.values_of("dir-sh") {
        for jobs_dir in jobs_dir_it {
            fred.path_shell.push(jobs_dir.to_string());
        }
    }

    if let Some(jobs_dir_it) = matches.values_of("dir-py") {
        for jobs_dir in jobs_dir_it {
            fred.path_python.push(jobs_dir.to_string());
        }
    }
    if fred.actions.contains(&cfg::Action::LoadScripts) {}

    if let Some(in_v) = matches.values_of("rdbms") {
        for enviroment_variable in in_v {
            fred.rdbms_connection_uri = Some(enviroment_variable.to_string());
        }
    } else {
        fred.rdbms_connection_uri = None;
    }
}

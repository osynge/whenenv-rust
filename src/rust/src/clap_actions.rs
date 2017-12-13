use clap::ArgMatches;
use std::collections::HashSet;


pub fn actions_get(matches: &ArgMatches) -> HashSet<String> {
    let mut hs_actions = HashSet::<String>::new();

    if let Some(_) = matches.values_of("list-provides") {
        let bill = String::from("load-jobs");
        hs_actions.insert(bill);
        let list_provides = String::from("list-provides");
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

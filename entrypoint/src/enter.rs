/// admin inits database
///     -> command dbinit
///
use std::{env, fs, process, sync::Once};
use voting::voting_day::VOTING_DATE;
use {after_voting, data, voting};

static DB_ONCE: Once = Once::new();

fn collect_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please enter arguments");
        process::exit(1);
    } else if args.len() > 2 {
        eprintln!("takes only one argument");
        process::exit(1);
    }
    return args;
}

fn db_available() -> bool {
    let files = fs::read_dir("./").unwrap();

    let mut filenames = files.filter_map(|f| f.ok().map(|f| f.file_name()));
    let db_exists = filenames.any(|db| db == "cracy.db");
    return db_exists;
}

fn db_init() {
    let db_exists = db_available();

    if db_exists {
        eprintln!("All systems go, ready to vote");
        process::exit(1)
    } else {
        DB_ONCE.call_once(|| {
            data::db_init();
        });
    }
}

fn start_vote() {
    voting::vote_init()
}

fn split_args() {
    let args: Vec<String> = collect_args();
    if args[1] == "dbinit".to_string() {
        db_init()
    } else if args[1] == "vote".to_string() {
        start_vote()
    } else {
        eprintln!("Not a command");
    }
}

pub fn entry() {
    split_args()
}

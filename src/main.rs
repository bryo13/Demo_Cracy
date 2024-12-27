use std::{fs, sync::Once};
use voting::voting_day::VOTING_DATE;
use {after_voting, data, voting};

static DB_ONCE: Once = Once::new();

// ensure database is only initialized once
// by checking SQLite db exists and the init_db runs only once
// prevents violation of UNIQUE property inserting seed data
fn db_init() {
    let files = fs::read_dir("./").unwrap();

    let mut filenames = files.filter_map(|f| f.ok().map(|f| f.file_name()));
    let db_exists = filenames.any(|db| db == "cracy.db");

    if db_exists {
        println!("Welcome to vote");
        voting::vote_init();
    } else {
        DB_ONCE.call_once(|| {
            data::db_init();
        });
    }
}

// API call from timeserver to confirm date == const VOTING_DATE
fn confirm_current_date() -> bool {
    return true;
}

// confirm voting is done to start vote count
fn confirm_time_after_1830() -> bool {
    return true;
}

// // call after voting is done
// // add to only call after voting is done i.e const VOTING_DATE >= 1830hrs
fn pick_winner() {
    if confirm_current_date() && confirm_time_after_1830() {
        after_voting::count();
    }
}

fn main() {
    //db_init();
    pick_winner();
}

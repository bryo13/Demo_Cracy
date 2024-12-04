use data;

use std::{fs, sync::Once};

static DB_ONCE: Once = Once::new();

// ensure database is only initialized once
// by checking SQLite db exists and the init_db runs only once
// prevents violation of UNIQUE property inserting seed data
fn init_tables() {
    let files = fs::read_dir("./").unwrap();

    let mut filenames = files.filter_map(|f| f.ok().map(|f| f.file_name()));
    let db_exists = filenames.any(|db| db == "cracy.db");

    if db_exists {
        println!("Welcome to vote");
    } else {
        DB_ONCE.call_once(|| {
            data::db_init();
        });
    }
}

fn main() {
    init_tables();
}

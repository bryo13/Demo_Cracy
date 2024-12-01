mod database;

fn main() {
    // database::create_database::create_db();
    database::electorate_table::create_electorate_table()
        .expect("Couldnt create or insert data to electorate");
}

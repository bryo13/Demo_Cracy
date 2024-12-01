mod database;

fn db_init() {
    database::electorate_table::create_electorate_table().unwrap();

    database::candidate_table::create_candidate_table().unwrap();

}

fn main() {
    db_init()
}

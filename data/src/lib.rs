mod candidate_table;
pub mod create_database;
mod electorate_seed;
mod electorate_table;
mod results_table;
mod voting_table;

pub fn db_init() {
    electorate_table::create_electorate_table().unwrap();
    candidate_table::create_candidate_table().unwrap();
    voting_table::create_votes_table().unwrap();
    results_table::create_results_table().unwrap();
}

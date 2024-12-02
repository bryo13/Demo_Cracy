mod voting_table; 

pub fn voting_init() {
    voting_table::create_votes_table().unwrap();
}

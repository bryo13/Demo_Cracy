pub mod voting_day;
use data::electorate_table;

pub fn vote_init() {
    // let electorate_count = electorate_table::Electorate_count();

    voting_day::vote();
}

// rem unique value of voter_ID causes a panic!
// to deal with this

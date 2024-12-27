pub mod voting_day;
use data::electorate_table;

pub fn vote_init() {
    let electorate_count = electorate_table::Electorate_count();

    for i in 0..electorate_count {
        println!("you are electorate number: {:?}", i + 1);
        voting_day::vote();
    }
}

// rem unique value of voter_ID causes a panic!
// to deal with this

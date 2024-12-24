pub mod voting_day;

pub fn vote_init() {
    voting_day::vote();
}

// keep voting open for all members
// rem unique value of voter_ID causes a panic!

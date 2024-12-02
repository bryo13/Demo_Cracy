use data;
use voting;

fn main() {
    data::db_init();
    voting::voting_init();
}

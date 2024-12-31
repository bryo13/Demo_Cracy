/// admin inits database
///     -> command dbinit
///
use std::{env, fs, process, sync::Once};
use voting::voting_day::VOTING_DATE;
use {after_voting, data, voting};

static DB_ONCE: Once = Once::new();

fn collect_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        no_args();
        process::exit(1);
    } else if args.len() > 2 {
        eprintln!("takes only one argument");
        process::exit(1);
    }
    return args;
}

fn db_available() -> bool {
    let files = fs::read_dir("./").unwrap();

    let mut filenames = files.filter_map(|f| f.ok().map(|f| f.file_name()));
    let db_exists = filenames.any(|db| db == "cracy.db");
    return db_exists;
}

fn db_init() {
    let db_exists = db_available();

    if db_exists {
        eprintln!("All systems go, ready to vote");
        process::exit(1)
    } else {
        DB_ONCE.call_once(|| {
            data::db_init();
        });
    }
}

fn no_args() {
    println!("\x1b[31mNo args read\x1b[0m");
    println!("\x1b[4mAdmin args\x1b[0m :");
    println!("\x1b[34mdbinit - initializes db and nessesary tables
    i.e ./demo_cracy dbinit
vote - accepts user votes, through a server
    i.e ./demo_cracy vote
count_votes - counts the votes each candidate got and checks if any candidate passed the threshold
    i.e ./demo_cracy count_votes
\x1b[0m");
}

fn wrong_args() {
    println!("\x1b[31mWrong args read\x1b[0m");
    println!("\x1b[4mAdmin args\x1b[0m :");
    println!("\x1b[34mdbinit - initializes db and nessesary tables
    i.e ./demo_cracy dbinit
vote - accepts user votes, through a server
    i.e ./demo_cracy vote
count_votes - counts the votes each candidate got and checks if any candidate passed the threshold
    i.e ./demo_cracy count_votes
\x1b[0m");
}
// // API call from timeserver to confirm date == const VOTING_DATE
// fn confirm_current_date() -> bool {
//     return true;
// }

// // confirm voting is done to start vote count
// fn confirm_time_after_1830() -> bool {
//     return true;
// }

// // // call after voting is done
// // // add to only call after voting is done i.e const VOTING_DATE >= 1830hrs
// fn pick_winner() {
//     if confirm_current_date() && confirm_time_after_1830() {
//         after_voting::count();
//     }
// }

fn split_args() {
    let args: Vec<String> = collect_args();
    if args[1] == "dbinit".to_string() {
        db_init()
    } else if args[1] == "vote".to_string() {
        voting::vote_init()
    } else if args[1] == "count_votes".to_string() {
        after_voting::count()
    } else {
        wrong_args()
    }
}

pub fn entry() {
    split_args()
}

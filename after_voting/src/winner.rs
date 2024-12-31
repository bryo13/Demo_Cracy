use data::create_database::DB_PATH;
use data::electorate_table;
/// The winner is the first candidate gets to greater than
/// (all electorate / 2) * 5
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

// read results_table in desc order
#[tokio::main]
async fn read_results() -> Result<Vec<SqliteRow>, sqlx::Error> {
    let winner_pool = SqlitePool::connect(DB_PATH).await?;

    let res_query = sqlx::query(
        "SELECT candidate_name, voter_sum 
    FROM results 
    ORDER BY voter_sum desc;",
    )
    .fetch_all(&winner_pool)
    .await?;

    Ok(res_query)
}

// calc winning threshold -  (all electorate / 2) * 5
fn calc_threshold(count: i64) -> i64 {
    let mut threshold: i64 = 0;
    let half = (count / 2) as i64;
    threshold = half * 5;
    return threshold;
}

// gets the winner
// fn get_highest(res: Vec<SqliteRow>) {
//     if let Some(first_row) = res.get(0) {
//         let name: String = first_row.get("candidate_name");
//         let score: i64 = first_row.get("voter_sum");
//         println!("{:?} - {:?}", name, score);
//     } else {
//         println!("No candidate found");
//     }
// }

// highest and passes the threshold
// returns those that pass as a vec
fn get_highest(res: Vec<SqliteRow>, count: i64) -> Vec<String> {
    let mut past_threshold: Vec<String> = Vec::new();
    let threshold = calc_threshold(count);

    for row in res {
        let sum: i64 = row.get("voter_sum");
        let name: String = row.get("candidate_name");

        if sum > threshold {
            past_threshold.push(name);
        };
    }
    return past_threshold;
}

pub fn pronounce_winner() {
    let count = electorate_table::Electorate_count();
    match read_results() {
        Ok(row) => println!("{:?}", get_highest(row, count)),
        Err(e) => eprintln!("{:?}", e),
    };
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
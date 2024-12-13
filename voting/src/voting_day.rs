/// On voting day:
///  -> The electorate will be asked to enter their ID number
///     the ID will be verified if he/she is present in the DB
///     If he/she is above 18.
///  -> Once this parameters are met, they will be asked to choose
///     their prefered candidate, this updates the votes table.
///     Then their less prefered candidate until all candidates are captured.
///  -> For the prefered candidate, they recieve a total of the number of candidates,
///     i.e if there are three candidates, an electorate's fav candidate will recieve
///     3 points, the 2nd prefered candidate 2 and the least prefered candidate 1.
use chrono::NaiveDate;
use data::create_database::DB_PATH;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use std::io;

const VOTING_DATE: &str = "2024-11-05";

#[tokio::main]
async fn electorate_details() -> Result<SqliteRow, sqlx::Error> {
    let voting_pool = SqlitePool::connect(DB_PATH).await?;

    let mut id_number: String = String::new();
    io::stdin()
        .read_line(&mut id_number)
        .expect("couldnt read user ID");

    let query_user = sqlx::query(
        "SELECT * FROM electorate_table 
        WHERE ID_number = ?;",
    )
    .bind(id_number.trim())
    .fetch_one(&voting_pool)
    .await?;

    Ok(query_user)
}

fn valid_age(details: Result<SqliteRow, sqlx::Error>) -> bool {
    match details {
        Ok(user_info) => {
            let dob: String = user_info.get("DOB");
            let user_dob =
                NaiveDate::parse_from_str(dob.as_str(), "%Y-%m-%d").expect("Invalid date format");
            let voting_date =
                NaiveDate::parse_from_str(VOTING_DATE, "%Y-%m-%d").expect("Invalid date format");

            if (voting_date.signed_duration_since(user_dob).num_weeks()) / 54 >= 18 {
                return true;
            }
        }
        Err(e) => fprintln!("{:?}", err),
    }
    return false;
}

// confirm the voter chooses a valid candidate
// checking if they are part of the candidates' table
async fn candidate_present(firstname: String) -> bool {
    let query_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Could not create get candidate pool");

    let cnds = sqlx::query(
        "SELECT et.First_name, ct.Electorate_ID_number
    FROM electorate_table as et
    INNER JOIN candidates_table as ct
    ON ct.Electorate_ID_number = et.ID_number
    where et.First_name = ?;",
        firstname,
    )
    .fetch_one(&query_pool)
    .await;

    match cnds {
        Ok(_) => return true,
        Err(_) => return false,
    };
}
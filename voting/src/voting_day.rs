/// Computation that handles results of a voting day and logic
/// involved i.e age verification

const VOTING_DATE: &str = "2024-11-05";

#[tokio::main]
async fn electorate_details() -> Result<SqliteRow, sqlx::Error> {}

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

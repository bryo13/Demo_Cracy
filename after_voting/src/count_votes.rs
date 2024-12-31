/// basically calling SUM() on all columns of votes_table
/// Calc percentage of all candidates
/// -> create results table that holds vote count
use data::create_database::DB_PATH;
use errors;
use serde::Serialize;
use serde_json::Value;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};

// holds results afer summation
#[derive(Debug, Serialize)]
pub struct ElectionResult {
    mannix: i64,
    cleon: i64,
    rashelle: i64,
}

// fn called by external mods
pub fn returning_officer() {
    let results = get_results();
    let sum = sum_votes(results);
    insert_results(sum).unwrap();
}

// query returning summed votes for each candidate
#[tokio::main]
async fn get_results() -> SqliteRow {
    let sum_pool = SqlitePool::connect(DB_PATH).await.expect("cant connect db");

    let sum_query = sqlx::query(
        "SELECT SUM(Rashelle) as Rashelle, SUM(Cleon) as Cleon, SUM(Mannix) as Mannix 
        FROM votes_table;",
    )
    .fetch_one(&sum_pool)
    .await
    .expect("couldnt get sum of the votes");

    return sum_query;
}

// returns updated ElectionResult type from get_results()
fn sum_votes(votes_sum: SqliteRow) -> ElectionResult {
    let mut cleon_sum: i64 = votes_sum.get("Cleon");
    let mut mannix_sum: i64 = votes_sum.get("Mannix");
    let mut rashelle_sum: i64 = votes_sum.get("Rashelle");

    return ElectionResult {
        mannix: mannix_sum,
        cleon: cleon_sum,
        rashelle: rashelle_sum,
    };
}

// insert results count to results table
#[tokio::main]
async fn insert_results(results: ElectionResult) -> Result<String, sqlx::Error> {
    let insert_pool = SqlitePool::connect(DB_PATH).await?;

    let ser = serde_json::to_value(&results).expect("couldnt serialize preference struct");
    if let Value::Object(flds) = ser {
        for (k, v) in flds {
            match sqlx::query("INSERT INTO results(candidate_name, voter_sum) VALUES(?,?);")
                .bind(k)
                .bind(v)
                .execute(&insert_pool)
                .await
            {
                Ok(_) => println!("Recorded results"),
                Err(e) => errors::unique_rows::unique_constraint_failed(
                    e,
                    "results records have already been recorded",
                ),
            };
        }
    };
    Ok(String::from("Inserted results"))
}

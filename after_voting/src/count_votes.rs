/// basically calling SUM() on all columns of votes_table
/// Calc percentage of all candidates
/// -> create results table that holds vote count

/// The winner is the first candidate gets to greater than 
/// (all electorate / 2) * 5
use data::create_database::DB_PATH;
use sqlx::{Row, SqlitePool};

#[derive(Debug)]
pub struct ElectionResult {
    mannix: i64,
    cleon: i64,
    rashelle: i64,
}

#[tokio::main]
async fn create_results_table() -> Result<String, sqlx::Error> {
    let results_pool = SqlitePool::connect(DB_PATH).await?;

    let _results_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS results(
            ID integer PRIMARY KEY AUTOINCREMENT,
            candidate_name text UNIQUE,
            voter_sum integer);",
    )
    .execute(&results_pool)
    .await?;

    Ok(String::from("Created results table"))
}

#[tokio::main]
async fn count_votes() -> ElectionResult {
    let count_pool = SqlitePool::connect(DB_PATH)
        .await
        .expect("cant create voter count pool");

    let sum_query = sqlx::query(
        "SELECT SUM(Rashelle) as rashelle, SUM(Mannix) as mannix, SUM(Cleon) as cleon
        FROM votes_table;",
    )
    .fetch_all(&count_pool)
    .await
    .expect("cant get sum of candidates");

    let mut cleon_sum: i64 = 0;
    let mut mannix_sum: i64 = 0;
    let mut rashelle_sum: i64 = 0;

    for r in sum_query {
        cleon_sum = r.get("cleon");
        rashelle_sum = r.get("rashelle");
        mannix_sum = r.get("mannix");
    }
    return ElectionResult {
        mannix: mannix_sum,
        cleon: cleon_sum,
        rashelle: rashelle_sum,
    };
}

// insert count results to results table
#[tokio::main]
async fn insert_results(res: ElectionResult) -> Result<String, sqlx::Error> {
    let results = count_votes();
    let insert_pool = SqlitePool::connect(DB_PATH).await?;

    let ser = serde_json::to_value(&res).expect("couldnt serialize preference struct");
    if let Value::Object(flds) = ser {
        for (k, v) in flds {
            let _insert_query = sqlx::query("
            INSERT INTO results(candidate_name, voter_sum) VALUES(?,?);
            ")
            .bind(k).bind(v)
            .execute(&insert_pool)
            .await?;
        }
    };
    Ok(String::from("Inserted results"))
}

// how many pple voted?
#[tokio::main]
async fn electorate_count() -> f64 {
    let count_pool = SqlitePool::connect(DB_PATH).await?;

    // find count of electorate
    let count = sqlx::query("
    SELECT COUNT(ID_number) AS count 
    FROM "electorate_table"; 
    ")
    .fetch(&count_pool)
    .await
    .expect("couldnt count all electorate");

    let count: f64 = count.get("count");
    return count
}

fn calc_threshold() {
    let count = electorate_count();
    let half = (count / 2.0).ceil() as i64;
    return half * 5 
}

// find winner
// read results table in asc order
// check the first candidate crossed the threshold

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_results_table() {
        let rt = tokio::task::spawn_blocking(|| 
            create_results_table())
            .await
            .expect("Couldnt init electorate table");


        assert!(rt.is_ok(), "Failed due to: {:?}",rt);
    }
}

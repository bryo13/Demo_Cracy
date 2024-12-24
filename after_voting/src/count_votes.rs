/// basically calling SUM() on all columns of votes_table
/// Calc percentage of all candidates
/// -> create results table that holds vote count
/// ->  and percentages
use data::create_database::DB_PATH;
use sqlx::{SqlitePool, Row};

#[derive(Debug)]
pub struct ElectionResult {
    mannix_result: i64,
    cleon_result: i64,
    rashelle_result: i64,
}

#[tokio::main]
async fn create_results_table() -> Result<String, sqlx::Error> {
    let results_pool = SqlitePool::connect(DB_PATH).await?;

    let _results_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS results(
            ID integer PRIMARY KEY AUTOINCREMENT,
            votes_sum integer,
            votes_percentage integer,
        );",
    )
    .execute(&results_pool)
    .await?;
    Ok(String::from("Created results table"))
}

#[tokio::main]
pub async fn count() -> ElectionResult {
    let count_pool = SqlitePool::connect(DB_PATH)
        .await
        .expect("cant create voter count pool");

    let sum_query = sqlx::query(
        "
        SELECT SUM(Rashelle) as rashelle, SUM(Mannix) as mannix, SUM(Cleon) as cleon
        FROM votes_table;
    ",
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
        mannix_result: mannix_sum,
        cleon_result: cleon_sum,
        rashelle_result: rashelle_sum,
    };
}

// fn calc_percentage()

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_results_table() {
        let rt = tokio::task::spawn_blocking(|| {
            data::create_database::create_db();
            create_results_table();
        })
        .await
        .expect("error init result tests");
    }
}

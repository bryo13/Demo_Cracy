/// basically calling SUM() on all columns of votes_table
/// Calc percentage of all candidates
/// -> create results table that holds vote count
/// ->  and percentages
use data::create_database::DB_PATH;
use sqlx::SqlitePool;

struct ElectionResult {
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
async fn count() -> ElectionResult {
    let res = ElectionResult {
        mannix_result: 0,
        cleon_result: 0,
        rashelle_result: 0,
    };

    return res;
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

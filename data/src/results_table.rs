use crate::create_database;
use sqlx::SqlitePool;

#[tokio::main]
pub async fn create_results_table() -> Result<String, sqlx::Error> {
    let results_pool = SqlitePool::connect(create_database::DB_PATH).await?;

    let _results_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS results(
            ID integer PRIMARY KEY AUTOINCREMENT,
            candidate_name text UNIQUE,
            voter_sum integer);",
    )
    .execute(&results_pool)
    .await?;
    println!("--> Created results table");
    Ok(String::from("Created results table"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_results_table() {
        let rt = tokio::task::spawn_blocking(|| create_results_table())
            .await
            .expect("Couldnt init electorate table");

        assert!(rt.is_ok(), "Failed due to: {:?}", rt);
    }
}

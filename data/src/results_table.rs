use crate::create_database;
use sqlx::SqlitePool;

pub fn create_results_table() -> Result<String, String> {
    let _ = match create_database::create_db() {
        Ok(_) => results_table(),
        Err(error) => {
            if error == "Database exists" {
                results_table()
            } else {
                Err(String::from("Error met when creating votes table"))
            }
        }
    };

    Ok(String::from("--> Created results table successfully"))
}

#[tokio::main]
async fn results_table() -> Result<String, String> {
    let results_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Cant create results pool");

    let _results_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS results(
            ID integer PRIMARY KEY AUTOINCREMENT,
            candidate_name text UNIQUE,
            voter_sum integer);",
    )
    .execute(&results_pool)
    .await
    .expect("Error creating results table");
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

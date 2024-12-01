/// connects to database
///     -> cracy.db from database/create_table.rs
///
/// Connect to electorate table
///     -> electorate table database/electorate_table.rs
///
/// Create candidates table
///
/// Inner join candidates_table electorate_table
///
/// Choose candidates max 3, have to be in the electorate_table
///     -> inner join candidates_table and electorate_table
///
/// Candidate will include electorate details, 1st prefered count,
///     2nd prefered count and 3rd prefered count
///
use super::create_database;
use sqlx::{Row, SqlitePool};

// pub fn create_candidate_table() -> Result<String, String> {}

#[tokio::main]
async fn candidate_table() -> Result<String, String> {
    let cd_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Couldnt create candidate pool");

    let _candidate_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS candidates_table(
            ID integer PRIMARY KEY AUTOINCREMENT,
            Electorate_ID_number integer);",
    )
    .execute(&cd_pool)
    .await
    .expect("Couldnt exec create candidate table");

    println!("--> created candidate table");
    Ok(String::from("--> Created candidate table successfully"))
}

// #[tokio::main]
// async fn insert_candidate() -> Result<String, String> {}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_candidate_table_created() {
        let tb = tokio::task::spawn_blocking(|| {
            candidate_table()
        })
        .await
        .expect("Couldnt init candidates table");
        
        assert!(tb.is_ok());
        let test_candidate_pool = SqlitePool::connect(create_database::DB_PATH)
            .await
            .expect("couldnt create test candidate pool");

        let tables = sqlx::query(
            "SELECT name
                FROM sqlite_master
                WHERE type='table' AND name='candidates_table';",
        )
        .fetch_one(&test_candidate_pool)
        .await
        .expect("error getting candidates_table");

        let name = tables.get::<String, &str>("name");
        assert_eq!(name, "candidates_table");
    }

}

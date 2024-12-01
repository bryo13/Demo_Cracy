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
use sqlx::SqlitePool;

pub fn create_candidate_table() -> Result<String, String> {
    let _ = match create_database::create_db() {
        Ok(_) => candidates_table(),

        Err(error) => {
            if error == "Database exists" {
                candidates_table()
            } else {
                Err(String::from("Error met when creating candidates table"))
            }
        }
    };
    Ok(String::from("--> created candidates table")) 
}

#[tokio::main]
async fn candidates_table() -> Result<String, String> {
    let cd_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Couldnt create candidate pool");

    let _candidates_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS candidates_table(
            ID integer PRIMARY KEY AUTOINCREMENT,
            Electorate_ID_number integer);",
    )
    .execute(&cd_pool)
    .await
    .expect("Couldnt exec create candidates table");

    println!("--> created candidates table");
    Ok(String::from("--> Created candidates table successfully"))
}

// #[tokio::main]
// async fn insert_candidate() -> Result<String, String> {}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_create_candidates_table_created() {
        let tb = tokio::task::spawn_blocking(|| {
            create_candidate_table();
            candidates_table()
        })
        .await
        .expect("Couldnt init candidates table");
        
        assert!(tb.is_ok());
        let test_candidates_pool = SqlitePool::connect(create_database::DB_PATH)
            .await
            .expect("couldnt create test candidates pool");

        let tables = sqlx::query(
            "SELECT name
                FROM sqlite_master
                WHERE type='table' AND name='candidates_table';",
        )
        .fetch_one(&test_candidates_pool)
        .await
        .expect("error getting candidates_table");

        let name = tables.get::<String, &str>("name");
        assert_eq!(name, "candidates_table");
    }

}

/// create_database creates the cracy.db sqlite database
use sqlx::{migrate::MigrateDatabase, Sqlite};

const DB_PATH: &str = "sqlite://cracy.db";

#[tokio::main]
pub fn create_db() -> Result<String, Err> {
    if !Sqlite::database_exists(DB_PATH).await.unwrap_or(false) {
        Sqlite::create_database(DB_PATH).await?;
    } else {
        Err(String::from("Database exists"));
    }
    Ok(String::from("Database ready"))
}

mod tests {
    use super::*;

    #[test]
    fn test_create_db_ok() {
        // test Ok value
        let result = create_db();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("Database ready"));
    }
}

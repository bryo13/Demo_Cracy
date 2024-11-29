/// create_database creates the cracy.db sqlite database
use sqlx::{migrate::MigrateDatabase, Sqlite};

pub const DB_PATH: &str = "sqlite://cracy.db";

#[tokio::main]
pub async fn create_db() -> Result<String, String> {
    if !Sqlite::database_exists(DB_PATH).await.unwrap_or(false) {
        Sqlite::create_database(DB_PATH)
            .await
            .expect("Couldnt create db");
    } else {
        println!("--> Db exists");
    }
    Ok(String::from("Database ready"))
}

#[cfg(test)]
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

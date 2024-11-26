use sqlx::{migrate::MigrateDatabase, Sqlite};

const DB_URL: &str = "sqlite://cracy.db";

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating {} database", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created db successful"),
            Err(error) => panic!("Error {}", error),
        }
    } else {

        println!("DB exists");
    }
}

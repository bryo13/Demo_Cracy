/// Handles all electorate data ops and test
///     -> creating electorate_table
///     -> insert seed data into electorate_table
use super::{create_database, electorate_seed};
use sqlx::SqlitePool;

pub fn create_electorate_table() -> Result<String, String> {
    let _ = match create_database::create_db() {
        Ok(_) => elec_table(),

        Err(error) => {
            if error == "Database exists" {
                elec_table()
            } else {
                Err(String::from("Error met when creating electorate table"))
            }
        }
    };

    match insert_electorate() {
        Ok(res) => Ok(String::from(res)),
        Err(res) => Err(String::from(res)),
    }
}

#[tokio::main]
async fn elec_table() -> Result<String, String> {
    let db_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("couldnt create pool");

    let _elect_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS electorate_table(
            ID integer PRIMARY KEY AUTOINCREMENT,
            DOB integer,
            First_name varchar(255),
            Last_name varchar(255),
            ID_number long UNIQUE,
            County varchar(256));",
    )
    .execute(&db_pool)
    .await
    .expect("Couldnt exec create electorate table query");

    println!("--> Created electorate table");
    Ok(String::from("--> Created electorate table successfully"))
}

#[tokio::main]
async fn insert_electorate() -> Result<String, String> {
    let insert_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Could not create insert pool");

    let seed_data = electorate_seed::seed();

    for sd in seed_data {
        sqlx::query(
        "INSERT INTO electorate_table(DOB,First_name,Last_name, ID_number, County) VALUES(?,?,?,?,?);")
        .bind(sd.dob).bind(sd.first_name).bind(sd.last_name).bind(sd.id_number).bind(sd.county)
        .execute(&insert_pool)
        .await
        .expect("Couldnt exec insert query");
    }
    println!("--> Inserted seed data into electorate");
    Ok(String::from("--> Inserted electorate table successfully"))
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_create_electorate_table() {
        let elt = tokio::task::spawn_blocking(|| create_electorate_table())
            .await
            .expect("Couldnt init electorate table");

        assert!(elt.is_ok());
    }

    // test electorate table exists in db
    #[tokio::test]
    async fn test_elector_table() {
        let db_pool = SqlitePool::connect(create_database::DB_PATH)
            .await
            .expect("couldnt create test pool");

        let table = sqlx::query(
            "SELECT name 
            FROM sqlite_master 
            WHERE type='table' AND name='electorate_table';",
        )
        .fetch_one(&db_pool)
        .await
        .expect("Error querying table in db");

        let name = table.get::<String, &str>("name");

        assert_eq!(name, "electorate_table");
    }

    #[tokio::test]
    async fn test_insert_data() {
        let db_pool = SqlitePool::connect(create_database::DB_PATH)
            .await
            .expect("couldnt create test pool");

        let get_db_data = sqlx::query(
            "SELECT First_name 
            FROM electorate_table 
            WHERE First_name='Dors';",
        )
        .fetch_one(&db_pool)
        .await
        .expect("Error querying table in db");

        let name = get_db_data.get::<String, &str>("First_name");

        assert_eq!(name, "Dors");
    }
}

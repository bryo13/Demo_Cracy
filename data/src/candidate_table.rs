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
use crate::create_database;
use errors::unique_rows;
use sqlx::{Row, SqlitePool};

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
    insert_candidate().unwrap();

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
            Electorate_ID_number text UNIQUE);",
    )
    .execute(&cd_pool)
    .await
    .expect("Couldnt exec create candidate table");
    println!("--> Created candidate table successfully");
    Ok(String::from("--> Created candidate table successfully"))
}

/*
#[tokio::main]
async fn get_candidates() -> Result<Vec<i32>, String> {
    // There are three main power players
    // Rashelle, Cleon and Mannix
    let cd_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Couldnt create candidate pool");

    let cnds = sqlx::query(
        "SELECT ID_number FROM 'electorate_table'
        WHERE First_name in ('Rashelle','Cleon','Mannix')
        LIMIT=3;",
    )
    .fetch_all(&cd_pool)
    .await
    .expect("Couldnt get candidates ID numbers");

    let mut cands: Vec<i32> = Vec::new();
    for c in cnds {
        let id: i32 = c.get("ID_number");
        cands.push(id)
    }
    Ok(cands)
}
*/
#[tokio::main]
async fn insert_candidate() -> Result<String, String> {
    let candidate_insert_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Could not create candidates insert pool");

    let cnds = sqlx::query(
        "SELECT ID_number FROM electorate_table
            WHERE First_name in ('Rashelle','Cleon','Mannix')
            LIMIT 3;",
    )
    .fetch_all(&candidate_insert_pool)
    .await
    .expect("Couldnt get candidates ID numbers");

    let mut cands: Vec<String> = Vec::new();
    for c in cnds {
        let id: String = c.get("ID_number");
        cands.push(id)
    }

    for c in cands {
        match sqlx::query(
            "INSERT OR IGNORE INTO candidates_table(Electorate_ID_number)
                VALUES(?);",
        )
        .bind(c)
        .execute(&candidate_insert_pool)
        .await
        {
            Ok(_) => println!("--> Inserted candidate "),
            Err(e) => unique_rows::unique_constraint_failed(e, "Dublicate candidates"),
        };
    }

    Ok(String::from("Inserted candidates"))
}

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

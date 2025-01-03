/// create voting table with the columns holding
/// the candidates and voters_ID
use crate::create_database;
use sqlx::SqlitePool;

pub fn create_votes_table() -> Result<String, String> {
    let _ = match create_database::create_db() {
        Ok(_) => voting_table(),
        Err(error) => {
            if error == "Database exists" {
                voting_table()
            } else {
                Err(String::from("Error met when creating votes table"))
            }
        }
    };

    Ok(String::from("--> Created votes table successfully"))
}

/* gets the IDs of the candidates into a vector
// will be concatinated into the voters table query
#[tokio::main]
async fn get_candidates_ID() -> Result<Vec<i32>, String> {
    let candidate_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Couldnt create voter pool");

    let candidates = sqlx::query(
        "
        SELECT Electorate_ID_number
        FROM candidates_table;",
    )
    .fetch_all(&candidate_pool)
    .await
    .expect("Couldnt get candidates for voters table");

    let mut cands: Vec<i32> = Vec::new();
    for c in candidates {
        let id: i32 = c.get("Electorate_ID_number");
        cands.push(id)
    }

    Ok(cands)
}

async fn create_query() -> String {
    match get_candidates_ID().await {
        Ok(cands) => {
            let query = format!(
                "CREATE TABLE IF NOT EXISTS votes_table(ID integer PRIMARY KEY AUTOINCREMENT, {} integer, {} integer, {} integer);",
                cands[0], cands[1], cands[2]
            );
            return query;
        },
        Err(_) => todo!()
    }
}
*/
#[tokio::main]
async fn voting_table() -> Result<String, String> {
    let votes_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Couldnt create votes pool");

    let _create_votes_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS votes_table(
                ID integer PRIMARY KEY AUTOINCREMENT,
                voter_ID text  UNIQUE,
                Rashelle integer,
                Mannix integer,
                Cleon integer,
                County varchar(256));",
    )
    .execute(&votes_pool)
    .await
    .expect("Couldnt exec create votes table query");
    println!("--> Created votes table");
    Ok(String::from("--> Created votes table successfully"))
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::Row;

    // test votes table exists in db
    #[tokio::test]
    async fn test_votes_table() {
        let db_pool = SqlitePool::connect(create_database::DB_PATH)
            .await
            .expect("couldnt create test pool");

        let table = sqlx::query(
            "SELECT name 
            FROM sqlite_master 
            WHERE type='table' AND name='votes_table';",
        )
        .fetch_one(&db_pool)
        .await
        .expect("Error querying votes table in db");

        let name = table.get::<String, &str>("name");

        assert_eq!(name, "votes_table");
    }
}

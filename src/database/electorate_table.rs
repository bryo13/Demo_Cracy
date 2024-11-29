/// electorate_table mod creates the tables that will holCreated electorate table
/// successfullCreated electorate table successfullyyd
///     voter data
/// The data will include:
///     - DOB
///     - ID number(defaults to a random 8 digit number)
///     - First name
///     - Last name
///     - Registered county
///
/// Additional logic such as the age to vote, set at 18
///     and the candidate vying for post being present in
///     of the electorate table will either be dealt here
///     or in other modules!
///
/// Inserting data will also be handled in this module via sample seed
///     data
use super::create_database;
use sqlx::SqlitePool;

struct Electorate {
    dob: String,
    first_name: String,
    last_name: String,
    id_number: u64,
    county: String,
}

pub fn create_electorate_table() -> Result<String, String> {
    let db = create_database::create_db();

    match db {
        Ok(_) => elec_table(),

        Err(error) => {
            if error == "Database exists" {
                elec_table()
            } else {
                Err(String::from("Error met when creating electorate table"))
            }
        }
    }
}

#[tokio::main]
async fn elec_table() -> Result<String, String> {
    let db_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("couldnt create pool");

    let elect_table = sqlx::query(
        "CREATE TABLE IF NOT EXISTS electorate_table(
            ID integer PRIMARY KEY AUTOINCREMENT,
            DOB integer,
            First_name varchar(255),
            Last_name varchar(255),
            ID_number long,
            County varchar(256));",
    )
    .execute(&db_pool)
    .await
    .expect("Couldnt exec create table query");

    Ok(String::from("--> Created electorate table successfully"))
}


/*
#[tokio::main]
async fn insert_electorate() -> Result<String, String> {
    let insert_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Could not create insert pool");

    let insert_query = sqlx::query(
        "INSERT INTO electorate_table(DOB,First_name,Last_name, ID_number, County) VALUES(?,?,?,?,?);")
        .bind().bind().bind().bind().bind()
        .execute(&insert_pool)
        .await
        .expect("Couldnt exec insert query");

    Ok(String::from("--> Inserted electorate table successfully"))
}
*/
fn seed() -> Vec<Electorate> {
    let citizens = vec![
        Electorate {
            dob: "1943-02-22".to_string(),
            first_name: "Dors".to_string(),
            last_name: "Venabili".to_string(),
            id_number: 23422345,
            county: "Uni".to_string(),
        },
        Electorate {
            dob: "1943-11-21".to_string(),
            first_name: "Eto".to_string(),
            last_name: "Demerizel".to_string(),
            id_number: 35343463,
            county: "Terminus".to_string(),
        },
        Electorate {
            dob: "1982-01-02".to_string(),
            first_name: "Rashelle".to_string(),
            last_name: "I".to_string(),
            id_number: 6546347,
            county: "Wye".to_string(),
        },
        Electorate {
            dob: "1929-12-04".to_string(),
            first_name: "Mannix".to_string(),
            last_name: "IV".to_string(),
            id_number: 90242523,
            county: "Wye".to_string(),
        },
        Electorate {
            dob: "1974-02-22".to_string(),
            first_name: "Emmer".to_string(),
            last_name: "Thalus".to_string(),
            id_number: 82344869,
            county: "Wye".to_string(),
        },
        Electorate {
            dob: "2017-10-17".to_string(),
            first_name: "Raych".to_string(),
            last_name: "I".to_string(),
            id_number: 9934521,
            county: "Dahl".to_string(),
        },
        Electorate {
            dob: "1978-02-12".to_string(),
            first_name: "Davan".to_string(),
            last_name: "I".to_string(),
            id_number: 45238903,
            county: "Dahl".to_string(),
        },
        Electorate {
            dob: "1969-02-22".to_string(),
            first_name: "Cleon".to_string(),
            last_name: "I".to_string(),
            id_number: 6743434,
            county: "Terminus".to_string(),
        },
        Electorate {
            dob: "1968-02-22".to_string(),
            first_name: "Hari".to_string(),
            last_name: "Seldon".to_string(),
            id_number: 31415926,
            county: "Terminus".to_string(),
        },
    ];

    citizens
}

#[cfg(test)]
mod test {
    use sqlx::Row;
    use super::*;

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
}

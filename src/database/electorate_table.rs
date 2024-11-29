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
<<<<<<< HEAD
use sqlx::SqlitePool;
=======
use sqlx::{SqlitePool, Row};

struct Electorate {
    dob: String,
    first_name: String,
    last_name: String,
    id_number: u64,
    county: String,
}
>>>>>>> database

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
<<<<<<< HEAD
            ID integer not null primary key,
=======
            ID integer PRIMARY KEY AUTOINCREMENT,
>>>>>>> database
            DOB integer,
            First_name varchar(255),
            Last_name varchar(255),
            ID_number long,
            County varchar(256));",
    )
    .execute(&db_pool)
    .await
    .expect("Couldnt exec create table query");

<<<<<<< HEAD
    println!("--> Create table query result: {:?}", elect_table);
    Ok(String::from("--> Created electorate table successfully"))
}

=======
    Ok(String::from("--> Created electorate table successfully"))
}

/*
>>>>>>> database
#[tokio::main]
async fn insert_electorate() -> Result<String, String> {
    let insert_pool = SqlitePool::connect(create_database::DB_PATH)
        .await
        .expect("Could not create insert pool");
<<<<<<< HEAD
=======

    let insert_query = sqlx::query(
        "INSERT INTO electorate_table(DOB,First_name,Last_name, ID_number, County) VALUES(?,?,?,?,?);")
        .bind().bind().bind().bind().bind()
        .execute(&insert_pool)
        .await
        .expect("Couldnt exec insert query");

    Ok(String::from("--> Inserted electorate table successfully"))
}
*/
/*
fn seed() -> Vec<Electorate> {
    let citizens = vec![
        Electorate {dob: "1943-02-22".to_string(),first_name: "Dors", last_name:"Venabili", county:"Uni"},
        Electorate {dob: "1943-11-21",first_name: "Eto", last_name:"Demerzel", county:"Terminus"},
        Electorate {dob: "1982-01-02",first_name: "Rashelle", last_name:"I", county:"Wye"},
        Electorate {dob: "1929-12-04",first_name: "Mannix", last_name:"IV", county:"Wye"},
        Electorate {dob: "1974-02-22",first_name: "Emmer", last_name:"Thalus", county:"Wye"},
        Electorate {dob: "2017-10-17",first_name: "Raych", last_name:"I", county:"Dahl"},
        Electorate {dob: "1978-02-12",first_name: "Davan", last_name:"I", county:"Dahl"},
        Electorate {dob: "1969-02-22",first_name: "Cleon", last_name:"I", County:"Terminus"},
        Electorate {dob: "1968-02-22",first_name: "Hari", last_name:"Seldon", county:"Terminus"},
    ];
    
    citizens
}
*/
#[cfg(test)]
mod test {
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
            WHERE type='table' AND name='electorate_table';"
        )
        .fetch_one(&db_pool)
        .await
        .expect("Error querying table in db");

        let name = table.get::<String, &str>("name");

        assert_eq!(name, "electorate_table");
    }
>>>>>>> database
}

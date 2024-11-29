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
    DOB: String,
    First_name: String,
    Last_name: String,
    ID_number: u64,
    County: String,
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


fn seed() -> Vec<Electorate> {
    let citizens = vec![
        Electorate {DOB: '1943-02-22',First_name: "Dors", Last_name:"Venabili", County:"Uni"},
        Electorate {DOB: '1943-11-21',First_name: "Eto", Last_name:"Demerzel", County:"Terminus"},
        Electorate {DOB: '1982-01-02',First_name: "Rashelle", Last_name:"I", County:"Wye"},
        Electorate {DOB: '1929-12-04',First_name: "Mannix", Last_name:"IV", County:"Wye"},
        Electorate {DOB: '1974-02-22',First_name: "Emmer", Last_name:"Thalus", County:"Wye"},
        Electorate {DOB: '2017-10-17',First_name: "Raych", Last_name:"I", County:"Dahl"},
        Electorate {DOB: '1978-02-12',First_name: "Davan", Last_name:"I", County:"Dahl"},
        Electorate {DOB: '1969-02-22',First_name: "Cleon", Last_name:"I", County:"Terminus"},
        Electorate {DOB: '1968-02-22',First_name: "Hari", Last_name:"Seldon", County:"Terminus"},
    ];
    
    citizens
}


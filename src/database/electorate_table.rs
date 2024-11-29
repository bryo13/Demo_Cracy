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

pub fn create_electorate_table() -> Result<String, String> {
    let db = create_database::create_db();

    match db {
        Ok(_) => elec_table(),

        Err(error) => {
            if !db {
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
            ID integer not null primary key,
            DOB integer,
            First_name varchar(255),
            Last_name varchar(255),
            ID_number long,
            County varchar(256));",
    )
    .execute(&db_pool)
    .await
    .expect("Couldnt exec create table query");

    println!("--> Create table query result: {:?}", elect_table);
    Ok(String::from("Created electorate table successfully"))
}

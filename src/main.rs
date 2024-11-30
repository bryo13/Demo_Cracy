mod database;

fn main() {
<<<<<<< HEAD
    match database::electorate_table::create_electorate_table() {
        Ok(ready) => println!("{ready}"),
        Err(err) => println!("{:?}",err),
    }

=======
    // database::create_database::create_db();
    database::electorate_table::create_electorate_table()
    .expect("Couldnt create or insert data to electorate");
>>>>>>> database
}

mod database;

fn main() {
    match database::electorate_table::create_electorate_table() {
        Ok(ready) => println!("{ready}"),
        Err(err) => println!("{:?}",err),
    }

}

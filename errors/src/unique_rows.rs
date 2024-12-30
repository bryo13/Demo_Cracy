use sqlx::Error;

pub fn unique_constraint_failed(e: Error, msg: &str) {
    if let Some(se) = e.as_database_error() {
        if let Some(code) = se.code() {
            if code == "2067" {
                println!("{:?}", msg);
            }
        }
    } else {
        eprintln!("{:?}", e);
    }
}

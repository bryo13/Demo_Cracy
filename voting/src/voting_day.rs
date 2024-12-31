/// On voting day:
///  -> The electorate will be asked to enter their ID number
///     the ID will be verified if he/she is present in the DB
///     If he/she is above 18.
///  -> Once this parameters are met, they will be asked to choose
///     their prefered candidate, this updates the votes table.
///     Then their less prefered candidate until all candidates are captured.
///  -> For the prefered candidate, they recieve a total of the number of candidates,
///     i.e if there are three candidates, an electorate's fav candidate will recieve
///     5 points, the 2nd prefered candidate 3 and the least prefered candidate 1.
use chrono::NaiveDate;
use data::create_database::DB_PATH;
use errors::unique_rows;
use serde::Serialize;
use serde_json::Value;
use sqlx::{sqlite::SqliteRow, Row, SqlitePool};
use std::{collections::HashMap, io};
use tokio::runtime::Runtime;

pub const VOTING_DATE: &str = "2024-11-05";

#[derive(Debug, Serialize)]
struct Preference {
    first_pref: String,
    sec_pref: String,
    third_pref: String,
}

// holds votes per preference
#[derive(Debug)]
struct Votes {
    Mannix: i8,
    Cleon: i8,
    Rashelle: i8,
}

#[tokio::main]
async fn electorate_details(id_number: String) -> Result<SqliteRow, sqlx::Error> {
    let voting_pool = SqlitePool::connect(DB_PATH).await?;

    let query_user = sqlx::query(
        "SELECT * FROM electorate_table 
        WHERE ID_number = ?;",
    )
    .bind(id_number.trim())
    .fetch_one(&voting_pool)
    .await?;

    Ok(query_user)
}

fn valid_age(details: Result<&SqliteRow, sqlx::Error>) -> bool {
    match details {
        Ok(user_info) => {
            let dob: String = user_info.get("DOB");
            let user_dob =
                NaiveDate::parse_from_str(dob.as_str(), "%Y-%m-%d").expect("Invalid date format");
            let voting_date =
                NaiveDate::parse_from_str(VOTING_DATE, "%Y-%m-%d").expect("Invalid date format");

            if (voting_date.signed_duration_since(user_dob).num_weeks()) / 54 >= 18 {
                return true;
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
    return false;
}

// map containing candidates
// for removing already selected candidate
fn candidate_map() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert(String::from("1"), String::from("Mannix"));
    map.insert(String::from("2"), String::from("Rashelle"));
    map.insert(String::from("3"), String::from("Cleon"));
    return map;
}

// get prefered candidates
// to refactor
fn get_pref() -> Preference {
    let mut candidates = candidate_map();
    let mut preference: Preference = Preference {
        first_pref: String::new(),
        sec_pref: String::new(),
        third_pref: String::new(),
    };
    println!("Enter your prefered candidate");
    println!("\x1b[31m{:#?}\x1b[0m", candidates);

    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Cant read first pref");

    let pref = first.trim();

    if !candidates.contains_key(pref) {
        println!("Please choose from the numbers 1,2,3");
        get_pref();
    }
    if let Some(rem) = candidates.remove(pref) {
        println!("Your first prefered candidate is {:#?}", rem);
        preference.first_pref = rem
    }

    println!("Enter your prefered candidate");
    println!("\x1b[31m{:#?}\x1b[0m", candidates);
    let mut two = String::new();
    io::stdin()
        .read_line(&mut two)
        .expect("Cant read second prefered");

    let pref_two = two.trim();

    if !candidates.contains_key(pref_two) {
        println!("Please choose a candidate from the list");
        get_pref();
    }
    if let Some(rem) = candidates.remove(pref_two) {
        println!("your second prefered candidate is: {:#?} ", rem);
        preference.sec_pref = rem
    }

    for val in candidates.values() {
        preference.third_pref = val.to_string();
    }
    println!("\x1b[31mYour {:#?} \x1b[0m", preference);
    return preference;
}

fn get_votes(cnds: Preference) -> Votes {
    let mut votes = Votes {
        Cleon: 0,
        Mannix: 0,
        Rashelle: 0,
    };

    let ser = serde_json::to_value(&cnds).expect("couldnt serialize preference struct");
    if let Value::Object(flds) = ser {
        for (k, v) in flds {
            if k == "first_pref" && v == "Cleon" {
                votes.Cleon = 5;
            } else if k == "sec_pref" && v == "Cleon" {
                votes.Cleon = 3;
            } else if k == "third_pref" && v == "Cleon" {
                votes.Cleon = 1;
            }
            if k == "first_pref" && v == "Rashelle" {
                votes.Rashelle = 5;
            } else if k == "sec_pref" && v == "Rashelle" {
                votes.Rashelle = 3;
            } else if k == "third_pref" && v == "Rashelle" {
                votes.Rashelle = 1;
            }
            if k == "first_pref" && v == "Mannix" {
                votes.Mannix = 5;
            } else if k == "sec_pref" && v == "Mannix" {
                votes.Mannix = 3;
            } else if k == "third_pref" && v == "Mannix" {
                votes.Mannix = 1;
            }
        }
    };
    return votes;
}

// update votes table per voter
#[tokio::main]
async fn update_votes(electorate_details: Result<SqliteRow, sqlx::Error>) {
    let voting_pool = SqlitePool::connect(DB_PATH)
        .await
        .expect("Cant create voting pool");
    let mut ID_number: String = String::new();
    let mut county: String = String::new();
    let voters_pref = get_pref();
    let votes_per_pref = get_votes(voters_pref);

    match electorate_details {
        Ok(details) => {
            ID_number = details.get("ID_number");
            county = details.get("County");
        }
        Err(e) => eprintln!("{}", e),
    };

    match sqlx::query(
        "INSERT INTO votes_table(voter_ID,County,Rashelle,Mannix,Cleon) VALUES(?,?,?,?,?);",
    )
    .bind(ID_number)
    .bind(county)
    .bind(votes_per_pref.Rashelle)
    .bind(votes_per_pref.Mannix)
    .bind(votes_per_pref.Cleon)
    .execute(&voting_pool)
    .await
    {
        Ok(_) => println!("Successfully voted!"),
        Err(e) => unique_rows::unique_constraint_failed(e, "Already voted"),
    }
}

pub fn vote() {
    println!("Enter your ID number:");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Cant read id number");

    let mut trimed_id: String = id.trim().to_string();
    if !voted(&mut trimed_id) {
        let person = electorate_details(trimed_id);
        match person {
            Ok(ref electorate) => {
                if valid_age(Ok(electorate)) {
                    update_votes(person);
                }
            }
            Err(e) => eprintln!("Error {:#?}", e),
        }
    } else {
        eprintln!("\x1b[31mAlready voted\x1b[0m");
    }
}

// check already votes
#[tokio::main]
async fn check_voted(id: &mut String) -> SqliteRow {
    let voting_pool = SqlitePool::connect(DB_PATH)
        .await
        .expect("Cant create voting pool");

    let rw = sqlx::query("SELECT EXISTS(SELECT 1  from votes_table WHERE voter_ID = ?);")
        .bind(id.to_string())
        .fetch_one(&voting_pool)
        .await
        .expect("couldnt run check voter query");

    return rw;
}

fn voted(id: &mut String) -> bool {
    let exists: bool = check_voted(id).get(0);
    return exists;
}

/// Holds part of foundation electorate data

pub struct Electorate {
    pub dob: String,
    pub first_name: String,
    pub last_name: String,
    pub id_number: i32,
    pub county: String,
}

pub fn seed() -> Vec<Electorate> {
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

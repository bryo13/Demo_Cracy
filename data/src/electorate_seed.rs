/// Holds part of foundation electorate data

pub struct Electorate {
    pub dob: String,        // holds date of birth
    pub first_name: String, // Electorate's first name
    pub last_name: String,  // Electorate's last name
    pub id_number: String,     // Electorate's ID no
    pub county: String,     // Electorate's county
}

pub fn seed() -> Vec<Electorate> {
    let citizens = vec![
        Electorate {
            dob: "1943-02-22".to_string(),
            first_name: "Dors".to_string(),
            last_name: "Venabili".to_string(),
            id_number: "23422345".to_string(),
            county: "Uni".to_string(),
        },
        Electorate {
            dob: "1943-11-21".to_string(),
            first_name: "Eto".to_string(),
            last_name: "Demerizel".to_string(),
            id_number: "35343463".to_string(),
            county: "Terminus".to_string(),
        },
        Electorate {
            dob: "1982-01-02".to_string(),
            first_name: "Rashelle".to_string(),
            last_name: "I".to_string(),
            id_number: "65463474".to_string(),
            county: "Wye".to_string(),
        },
        Electorate {
            dob: "1929-12-04".to_string(),
            first_name: "Mannix".to_string(),
            last_name: "IV".to_string(),
            id_number: "90242523".to_string(),
            county: "Wye".to_string(),
        },
        Electorate {
            dob: "1974-02-22".to_string(),
            first_name: "Emmer".to_string(),
            last_name: "Thalus".to_string(),
            id_number: "82344869".to_string(),
            county: "Wye".to_string(),
        },
        Electorate {
            dob: "2017-10-17".to_string(),
            first_name: "Raych".to_string(),
            last_name: "I".to_string(),
            id_number: "99934521".to_string(),
            county: "Dahl".to_string(),
        },
        Electorate {
            dob: "1978-02-12".to_string(),
            first_name: "Davan".to_string(),
            last_name: "I".to_string(),
            id_number: "45238903".to_string(),
            county: "Dahl".to_string(),
        },
        Electorate {
            dob: "1969-02-22".to_string(),
            first_name: "Cleon".to_string(),
            last_name: "I".to_string(),
            id_number: "67434345".to_string(),
            county: "Terminus".to_string(),
        },
        Electorate {
            dob: "1968-02-22".to_string(),
            first_name: "Hari".to_string(),
            last_name: "Seldon".to_string(),
            id_number: "31415926".to_string(),
            county: "Terminus".to_string(),
        },
    ];

    citizens
}

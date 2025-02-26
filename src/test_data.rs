use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use prost::Message;

// Include the generated code from the Protocol Buffers
include!(concat!(env!("OUT_DIR"), "/test.rs"));

// Create a module for the evolved schema
pub mod evolved {
    // Include the generated code for the evolved schema
    include!("generated/test_evolved.rs");
}

// Serde-compatible data structures for JSON (mirroring the Protocol Buffers structs)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonPerson {
    pub name: String,
    pub id: i32,
    pub email: String,
    pub phones: Vec<JsonPhoneNumber>,
    pub addresses: Vec<JsonAddress>,
    pub metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonPhoneNumber {
    pub number: String,
    pub type_: i32, // 0=MOBILE, 1=HOME, 2=WORK
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

// Evolved JSON structure (with new fields)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonPersonEvolved {
    pub name: String,
    pub id: i32,
    pub email: String,
    pub phones: Vec<JsonPhoneNumberEvolved>,
    pub addresses: Vec<JsonAddressEvolved>,
    pub metadata: HashMap<String, String>,
    pub additional_field: Option<String>,
    pub priority: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonPhoneNumberEvolved {
    pub number: String,
    pub type_: i32, // 0=MOBILE, 1=HOME, 2=WORK, 3=OTHER
    pub is_primary: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JsonAddressEvolved {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub additional_info: Option<String>,
}

// Function to generate test data
pub fn generate_test_data(size: usize) -> (JsonPerson, Person) {
    // For JSON
    let mut json_person = JsonPerson {
        name: "Test Person".to_string(),
        id: 12345,
        email: "test@example.com".to_string(),
        phones: Vec::new(),
        addresses: Vec::new(),
        metadata: HashMap::new(),
    };

    // For Protocol Buffers
    let mut proto_person = Person {
        name: "Test Person".to_string(),
        id: 12345,
        email: "test@example.com".to_string(),
        phones: Vec::new(),
        addresses: Vec::new(),
        metadata: HashMap::new(),
    };

    // Add phone numbers based on size
    for i in 0..size {
        let phone_type = (i % 3) as i32; // Cycle through MOBILE, HOME, WORK
        
        json_person.phones.push(JsonPhoneNumber {
            number: format!("555-{}", 1000 + i),
            type_: phone_type,
        });

        proto_person.phones.push(person::PhoneNumber {
            number: format!("555-{}", 1000 + i),
            r#type: phone_type,
        });
    }

    // Add addresses based on size
    for i in 0..std::cmp::max(1, size / 2) {
        json_person.addresses.push(JsonAddress {
            street: format!("{} Main St", 100 + i),
            city: format!("City {}", i),
            state: format!("State {}", i),
            zip: format!("{}", 10000 + i),
            country: "Country".to_string(),
        });

        proto_person.addresses.push(person::Address {
            street: format!("{} Main St", 100 + i),
            city: format!("City {}", i),
            state: format!("State {}", i),
            zip: format!("{}", 10000 + i),
            country: "Country".to_string(),
        });
    }

    // Add metadata based on size
    for i in 0..size {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        json_person.metadata.insert(key.clone(), value.clone());
        proto_person.metadata.insert(key, value);
    }

    (json_person, proto_person)
}

// Function to generate evolved test data
pub fn generate_evolved_test_data(size: usize) -> (JsonPersonEvolved, evolved::Person) {
    let (json_basic, _) = generate_test_data(size);
    
    // Convert to evolved JSON with additional fields
    let json_evolved = JsonPersonEvolved {
        name: json_basic.name.clone(),
        id: json_basic.id,
        email: json_basic.email.clone(),
        phones: json_basic.phones.iter().map(|p| JsonPhoneNumberEvolved {
            number: p.number.clone(),
            type_: p.type_,
            is_primary: Some(p.type_ == 0), // Make MOBILE phones primary
        }).collect(),
        addresses: json_basic.addresses.iter().map(|a| JsonAddressEvolved {
            street: a.street.clone(),
            city: a.city.clone(),
            state: a.state.clone(),
            zip: a.zip.clone(),
            country: a.country.clone(),
            additional_info: Some("Extra address details".to_string()),
        }).collect(),
        metadata: json_basic.metadata.clone(), // Clone the HashMap
        additional_field: Some("New information".to_string()),
        priority: Some(5),
    };

    // For Protocol Buffers evolved schema
    let mut proto_evolved = evolved::Person {
        name: json_basic.name,
        id: json_basic.id,
        email: json_basic.email,
        phones: Vec::new(),
        addresses: Vec::new(),
        metadata: HashMap::new(),
        additional_field: "New information".to_string(),
        priority: 5,
    };
    
    // Add phone numbers with the new is_primary field
    for i in 0..size {
        let phone_type = (i % 3) as i32;
        proto_evolved.phones.push(evolved::person::PhoneNumber {
            number: format!("555-{}", 1000 + i),
            r#type: phone_type,
            is_primary: phone_type == 0, // Make MOBILE phones primary
        });
    }
    
    // Add addresses with the new additional_info field
    for i in 0..std::cmp::max(1, size / 2) {
        proto_evolved.addresses.push(evolved::person::Address {
            street: format!("{} Main St", 100 + i),
            city: format!("City {}", i),
            state: format!("State {}", i),
            zip: format!("{}", 10000 + i),
            country: "Country".to_string(),
            additional_info: "Extra address details".to_string(),
        });
    }
    
    // Add metadata
    for i in 0..size {
        let key = format!("key{}", i);
        let value = format!("value{}", i);
        proto_evolved.metadata.insert(key, value);
    }

    (json_evolved, proto_evolved)
}
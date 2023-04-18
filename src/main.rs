use serde::{Deserialize, Serialize};
fn main() {
    // Print the count of how many people are in space
    let people_in_space = deserialize_people(get_people());
    println!("There are {} people in space", people_in_space.people.len());

    // Print the name of each person in space
    for person in people_in_space.people {
        println!("{}", person.name);
    }
}

// Create struct to match the json response from get_people function
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    craft: String,
}

// Create a struct that contains a string for updated_time and a collection of Person structs
#[derive(Debug, Serialize, Deserialize)]
struct PeopleInSpace {
    update_time: String,
    people: Vec<Person>,
}

// Send a get request to https://api.spacebits.net/people and return the response JSON string
fn get_people() -> String {
    let url = "https://api.spacebits.net/people";
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send().unwrap();
    response.text().unwrap()
}

// Parse the response JSON string into a PeopleInSpace struct
fn deserialize_people(json: String) -> PeopleInSpace {
    serde_json::from_str(&json).unwrap()
}

mod tests {
    // Test get_people function to ensure it returns a non-empty string
    #[test]
    fn test_get_people() {
        let result = super::get_people();
        assert!(!result.is_empty());
    }

    // Test that deserialize_people returns a people_in_space_struct with at least 1 person
    #[test]
    fn test_deserialize_people() {
        let result = super::deserialize_people(super::get_people());
        assert!(!result.people.is_empty());
    }
}

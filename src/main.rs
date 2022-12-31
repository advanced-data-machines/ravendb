#[macro_use] extern crate serde_derive;

use ravendb::raven::{RavenClient};

fn main() {
    println!("RavenDB - Rust client test\n");
    let client = RavenClient::new("http://localhost:8080","Demo", "");

    let oren = Person{ 
        first_name: "Oren".to_string(), 
        last_name: "Ayende".to_string(),
        emails: vec!("ayende@ayende.com".to_string(), "oren@ravendb.net".to_string()),
        age: 29, 
    };

    let rslt = client.put("users/ayende", &oren);
    println!("{:?}",rslt);

    let rslt = client.get::<Person>("users/ayende").unwrap();
    println!("{:#?}",rslt);

    let rslt = client.del("users/ayende").unwrap();
    println!("{:#?}",rslt);
}

// TODO: 
// Need to figure out how to decorate this so we 
// can store "entities" in a designated collection
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person{
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub emails: Vec<String>
}


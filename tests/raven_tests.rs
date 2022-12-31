#[macro_use] extern crate serde_derive;
use ravendb::client::{RavenClient, RavenQuery};

const URL: &'static str = "http://localhost:8080";
const DATABASE: &'static str = "Demo";
const PEM: &'static str = "";


#[test]
fn test_query(){
    let client = RavenClient::new(URL, DATABASE, PEM);
    let mut rq = RavenQuery{
        query: "from @empty where emails in ($emails)".to_string(),
        query_params: std::collections::HashMap::new(),
    };
    rq.query_params.insert("emails".to_string(), vec!("oren@ravendb.net".to_string(),"ayende@ravendb.cloud".to_string() ));

    let rslt = client.raw_query::<TestPerson>(&rq);
    println!("Query Result: \n{:?}",rslt);
}

#[test]
fn test_insert(){
    let client = RavenClient::new(URL, DATABASE, PEM);

    for n in 0..1000{
        let p = TestPerson{ 
            first_name: n.to_string(), 
            last_name: "ludzie".to_string(),
            age: 29,
            emails: vec!(format!("{}@ayende.com",n)),
        };

        let rslt = client.put(&format!("temp/{}", n.to_string()), &p);
        println!("{:?}",rslt);
    }
}

#[test]
fn test_delete(){
    let client = RavenClient::new(URL, DATABASE, PEM);
    for n in 0..1000{
        let rslt = client.del(&format!("temp/{}", n.to_string()));
        println!("{:?}",rslt);
    }
}


// TODO: 
// Need to figure out how to decorate this so we 
// can store "entities" in a designated collection
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestPerson{
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub emails: Vec<String>
}

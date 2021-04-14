
mod raven;
#[macro_use] extern crate serde_derive;

fn main() {
    println!("RavenDB - Rust client test\n");
    let client = raven::RavenClient::new("http://localhost:8080","Demo", "");

    let oren = Person{ 
        first_name: "Oren".to_string(), 
        last_name: "Ayende".to_string(),
        emails: vec!("ayende@ayende.com".to_string(), "oren@ravendb.net".to_string()),
        age: 29, // age of Batman
    };

    let rslt = client.put("users/ayende", &oren);
    println!("{:?}",rslt);


    let rslt = client.get::<Person>("users/ayende").unwrap();
    println!("{:#?}",rslt);

    test_query(&client);
    test_insert(&client);
    test_delete(&client);
}

fn test_query(client: &raven::RavenClient){
    let mut rq = raven::RavenQuery{
        query: "from @empty where emails in ($emails)".to_string(),
        query_params: std::collections::HashMap::new(),
    };
    rq.query_params.insert("emails".to_string(), vec!("oren@ravendb.net".to_string(),"ayende@ravendb.cloud".to_string() ));

    let rslt = client.raw_query::<Person>(&rq);
    println!("Query Result: \n{:?}",rslt);


}

fn test_insert(client: &raven::RavenClient){
    for n in 0..1000{
        let p = Person{ 
            first_name: n.to_string(), 
            last_name: "ludzie".to_string(),
            age: 29,
            emails: vec!(format!("{}@ayende.com",n)),
        };

        let rslt = client.put(&format!("temp/{}", n.to_string()), &p);
        println!("{:?}",rslt);
    }
}

fn test_delete(client: &raven::RavenClient){
    for n in 0..1000{
        let rslt = client.del(&format!("temp/{}", n.to_string()));
        println!("{:?}",rslt);
    }
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



mod raven;
#[macro_use] extern crate serde_derive;

fn main() {
    println!("RavenDB - Rust client test\n");
    let client = raven::RavenClient::new("http://localhost:8080","Demo", "");

    let rslt = client.get("users/ayende");
    println!("{:?}",rslt);
    test_query(&client);
    test_insert(&client);
    test_delete(&client);
}

fn test_query(client: &raven::RavenClient){
    let mut rq = raven::RavenQuery{
        query: "from Users where Emails in ($emails)".to_string(),
        query_params: std::collections::HashMap::new(),
    };
    rq.query_params.insert("emails".to_string(), vec!("oren@ravendb.net".to_string(),"ayende@ravendb.cloud".to_string() ));

    client.raw_query(&rq);

}

fn test_insert(client: &raven::RavenClient){
    for n in 0..1000{
        let p = Person{ 
            first_name: n.to_string(), 
            last_name: "ludzie".to_string(),
            age: 29,
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


#[derive(Debug, Serialize,Deserialize,Clone)]
struct Person{
    first_name: String,
    last_name: String,
    age: u8,
}
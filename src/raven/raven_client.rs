use reqwest;
use super::RavenError;
use serde::ser::{Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize)]
pub struct RavenQuery{
    #[serde(rename = "Query")]
    pub query: String,

    #[serde(rename = "QueryParameters")]
    pub query_params: HashMap<String, Vec<String>>,
}

pub struct RavenClient{
    pub server: String, 
    pub database: String, 
    pub pem: String,
    client: reqwest::blocking::Client
}

impl RavenClient {
    pub fn new(server: &str, database: &str, pem: &str ) -> Self{
        Self{
            server: server.to_string(),
            database: database.to_string(),
            pem: pem.to_string(), 
            client: reqwest::blocking::Client::new()
        }    
    }

    fn url(&self, path: &str) -> String {
        format!("{}/databases/{}/{}", self.server, self.database, path.to_string())
    }

    pub fn raw_query(&self, query: &RavenQuery) -> Result<(), RavenError>  {
        let url = &self.url("queries");
        let rslt = self.client.post(url)
            .json(&query)
            .send();      

        let json  = rslt?.text();
        println!("{:#?}",json) ;
        Ok(())
    }

    pub fn get(&self, id: &str) -> Result<String, RavenError> {
        let resp = reqwest::blocking::get(&self.url(&format!("docs?id={}",id.to_string())))?
                    .json::<serde_json::Value>();                   
        Ok(resp?.to_string()) // return $this->_exec("GET", $url, 200, NULL)->Results[0];
    }

    pub fn put<T>(&self, id: &str, doc: T) -> Result<(), RavenError>  where T: Serialize{
        let rslt = self.client.put(&self.url(&format!("docs?id={}",id.to_string())))
            .json(&doc)
            .send();        
        println!("\n\n{:#?}",rslt);
        Ok(()) //return $this->_exec("PUT", $url, 201, $body);
    }

    pub fn del(&self, id: &str) -> Result<(), RavenError> {
        let rslt = self.client.delete(&self.url(&format!("docs?id={}",id.to_string())))
                    .send();
        println!("deleted: {:?}",rslt);
        Ok(())
    }    
}


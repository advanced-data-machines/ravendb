#![allow(dead_code)]
use reqwest;
use super::{RavenQuery, QueryResult, RavenError};
use serde::ser::{Serialize};


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

    pub fn raw_query<T: serde::de::DeserializeOwned>(&self, query: &RavenQuery) -> Result<QueryResult<T>, RavenError>  {
        let rslt: QueryResult<T> = self.client.post(&self.url("queries"))
            .json(&query)
            .send()?
            .json()?;
        Ok(rslt)
    }

    pub fn get<'a, T: serde::de::DeserializeOwned>(&self, id: &str) -> Result<QueryResult<T>, reqwest::Error> {
        let resp = reqwest::blocking::get(&self.url(&format!("docs?id={}",id.to_string())))?
                    .json::<QueryResult<T>>()?;    
        Ok(resp)
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


pub mod raven_client;
pub mod error;
pub use raven_client::{RavenClient};
pub use error::{RavenError};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryResult<T>{
    #[serde(rename = "Results")]
    pub results: Vec<T>,
}

//pub type RavenResult<T> = Result<T,RavenError>;

#[derive(Serialize, Deserialize)]
pub struct RavenQuery{
    #[serde(rename = "Query")]
    pub query: String,

    #[serde(rename = "QueryParameters")]
    pub query_params: HashMap<String, Vec<String>>,
}

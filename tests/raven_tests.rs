#[macro_use] extern crate serde_derive;
extern crate ravendb;

#[test]
fn test_unit_test_work_cheeky_smile() {
    assert_eq!(4,4);
}

#[test]
fn test_connection(){

    assert_eq!(true, true);
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

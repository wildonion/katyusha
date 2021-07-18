








use serde::{Serialize, Deserialize};
use uuid::Uuid;




#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct MetaData(Option<(Uuid, chrono::NaiveDateTime)>);
impl MetaData{
    pub fn new() -> Self{
        MetaData(Some((Uuid::new_v4(), chrono::Local::now().naive_local())))
    }
}





#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Burn{ //-- Burn will be stored on the stack due to its static data types thus it's bounded to the Copy trait and we can take a reference to self
    pub info: MetaData,
}
impl Burn{
    pub async fn start() -> Self{
        // TODO - start burning CPU using multithreading round robin algorithm
        // ...
        Burn{info: MetaData::new()}
    }
}






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
pub struct Inject{ //-- Burn will be stored on the stack due to its static data types thus it's bounded to the Copy trait and we can take a reference from self
    pub info: MetaData,
}
impl Inject{
    pub async fn start() -> Self{
        // TODO - inject your arbitrary codes into the target RAM
        // ...
        Inject{info: MetaData::new()}
    }
}
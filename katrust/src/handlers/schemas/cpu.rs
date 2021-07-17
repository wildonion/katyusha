








use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;




pub struct Id(Uuid);
impl Id{
    pub fn new() -> Self{
        Id(Uuid::new_v4())
    }
}


#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Burn{ //-- Burn will be stored on the stack due to its static data types thus it's bounded to the Copy trait and we can take a reference from self
    pub id: Uuid,
}
impl Burn{
    pub async fn start() -> Self{
        // TODO - start burning cpu using multithreading round robin algorithm
        // ...
        Burn{id: Uuid::new_v4()}
    }
}









use serde::{Serialize, Deserialize};
use std::sync::Arc;
use uuid::Uuid;




#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct GPSData{ //-- GPSData will be stored on the stack due to its static data types thus it's bounded to the Copy trait and we can take a reference from self
    pub id: Uuid,
    pub imei: i64,
    pub lat: i32,
    pub lon: i32,
    pub speed: f32,
    pub alt: i16,
    pub angle: i16,
    pub satellites: i8,
    pub devicetime: i64,
}






impl GPSData{
    pub async fn last() -> Self{
        GPSData{id: Uuid::new_v4(), imei: 351756051523999, lat: 0, lon: 0, alt: 0, angle: 0, satellites: 0, speed: 0.0, devicetime: 0}
    }
}

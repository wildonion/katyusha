



/*


NOTE - get the len, capacity and the pointer of a variable which contains the unique address 
       for each variable’s value’s bytes (saved in stack or heap) inside the stack and change 
       its offset to change the value inside the variable in runtime, finally it’ll point 
       to another location inside the memory stack.
       
       

-------------
unsafe tricks
-------------
https://gist.github.com/wildonion/4f1956d9908e348a74b4381458e474e1#file-unsafer-rs
https://github.com/wildonion/aravl/blob/master/microservices/device/src/schemas/device.rs
https://pramode.in/2016/09/13/using-unsafe-tricks-in-rust/
    
    
*/





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
pub struct Inject{ //-- Inject will be stored on the stack due to its static data types thus it's bounded to the Copy trait and we can take a reference from self
    pub info: MetaData,
}
impl Inject{
    pub async fn start() -> Self{
        // TODO - inject your arbitrary codes into the target RAM
        // ...
        Inject{info: MetaData::new()}
    }
}

use serde::{
    Deserialize, 
    Serialize, 
};

#[derive(Serialize, Deserialize,  PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
}

#[derive(Serialize, Deserialize,  PartialEq, Debug, Clone)]
pub struct GreetResp{
    pub message:String,
}

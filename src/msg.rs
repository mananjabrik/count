use cosmwasm_std::Addr;
use serde::{
    Deserialize, 
    Serialize, 
};

#[derive(Serialize, Deserialize,  PartialEq, Debug, Clone)]
pub enum QueryMsg {
    Greet {},
    AdminsList {},
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstantiateMessage{
    pub admins:Vec<String>,
    pub donation_denom:String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ExcuteMsg {
    AddMembers { admins: Vec<String>},
    Leave {},
    Donate {}, 
}

#[derive(Serialize, Deserialize,  PartialEq, Debug, Clone)]
pub struct GreetResp{
    pub message:String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AdminsListResp  {
    pub admins: Vec<Addr>,
}
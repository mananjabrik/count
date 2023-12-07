use cosmwasm_std::{Addr};

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GreetResp)]
    Greet {},
    #[returns(AdminsListResp)]
    AdminsList {},
    #[returns(CountResp)]
    Count {}
}

#[cw_serde]
pub struct InstantiateMessage{
    pub admins:Vec<String>,
    pub donation_denom:String,
}
#[cw_serde]
pub enum ExcuteMsg {
    AddMembers { admins: Vec<String>},
    Leave {},
    Donate {},
    Increment {}, 
}

// response json 
#[cw_serde]
pub struct GreetResp{
    pub message:String,
}
#[cw_serde]
pub struct AdminsListResp  {
    pub admins: Vec<Addr>,
}

#[cw_serde]
pub struct CountResp{
    pub value: u128,
}
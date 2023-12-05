use cosmwasm_std::{
    entry_point,
    Binary, 
    Deps, 
    DepsMut, 
    Env, 
    MessageInfo, 
    Response, 
    StdResult, 
};
use error::ContractError;
use msg::{InstantiateMessage, ExcuteMsg};
mod contract;
mod msg;
mod state;
mod error;

#[entry_point]
pub fn instantiate(
    deps:DepsMut, 
    env:Env, 
    info: MessageInfo, 
    msg: InstantiateMessage
    )-> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExcuteMsg) -> Result<Response, ContractError>{
    contract::execute(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps:Deps, env:Env, msg: msg::QueryMsg)-> StdResult<Binary> {
    contract::query(deps, env, msg)
}


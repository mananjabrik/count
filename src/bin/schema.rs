use cosmwasm_schema::write_api;
use count::msg::{InstantiateMessage, ExcuteMsg, QueryMsg};


fn main() {
    write_api! {
        instantiate: InstantiateMessage,
        execute: ExcuteMsg,
        query: QueryMsg
    }
}

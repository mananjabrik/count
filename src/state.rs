use cosmwasm_std::{Addr};
use cw_storage_plus::Item;

pub const ADMINS: Item<Vec<Addr>> = Item::new("admins");
pub const DONATION_DENOM: Item<String> = Item::new("donatione_denom");
pub const COUNT:Item<u128> = Item::new("COUNT_NUMBER");
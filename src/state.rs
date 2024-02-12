use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{ Addr, Uint128 };
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub fee: Uint128,
    pub owner: Addr,
    pub deployer: Addr
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub fee: Uint128,
    pub owner: Addr,
}


pub const CONFIG: Item<Config> = Item::new("state");

pub const STAKING: Map<Addr, Uint128> = Map::new("staking");

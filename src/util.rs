use cosmwasm_std::{ Response, Uint128, Storage, Addr };
use crate::error::ContractError;
use crate::state::CONFIG;


pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_ORDER: u64 = 10;

pub fn multiple() -> Uint128 { Uint128::from(100u128) }
pub fn decimal() -> Uint128 { Uint128::from(1000000u128) }

pub fn check_owner(
    storage: &mut dyn Storage,
    address: Addr
) -> Result<Response, ContractError> {
    let cfg = CONFIG.load(storage)?;
    
    if address != cfg.owner {
        return Err(ContractError::Unauthorized {})
    }
    Ok(Response::new())
}

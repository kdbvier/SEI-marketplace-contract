use std::str::FromStr;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, from_binary, Uint128, CosmosMsg, BankMsg, Coin };
use cw2::{ set_contract_version, get_contract_version };
use cw721::{ Cw721ReceiveMsg };

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg };
use crate::state::{ Config, CONFIG, STAKING, ConfigResponse };
use crate::util::{ check_owner };

// version info for migration info
const CONTRACT_NAME: &str = "SEITIZEN_MARKETPLACE";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const DENOM: &str = "usei";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        fee: msg.fee.clone(),
        owner: info.sender.clone(),
        deployer: info.sender.clone()
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("fee", msg.fee.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwner { owner } => execute_update_owner(deps, info, owner),
        ExecuteMsg::UpdateFee { fee } => execute_update_fee(deps, info, fee),
        ExecuteMsg::Stake {} => execute_stake(deps, info),
        ExecuteMsg::WithdrawSei { amount } => execute_withdraw(deps, info, amount),
    }
}

pub fn execute_update_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner: Addr,
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.owner = owner.clone();
        Ok(exists)
    })?;
    Ok(Response::new().add_attribute("action", "update_owner").add_attribute("owner", owner))
}

pub fn execute_update_fee(
    deps: DepsMut,
    info: MessageInfo,
    fee: Uint128
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender)?;
    CONFIG.update(deps.storage, |mut exists| -> StdResult<_> {
        exists.fee = fee.clone();
        Ok(exists)
    })?;
    Ok(Response::new().add_attribute("action", "change_fee").add_attribute("fee", fee.to_string()))
}

pub fn execute_stake(
    deps: DepsMut,
    info: MessageInfo
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let (_index, coin) = info.funds.iter().enumerate().find(| (_i, c) | c.denom == DENOM.to_string() ).unwrap();
    if coin.clone().amount < config.fee {
        return Err(ContractError::Insufficient {});
    }
    STAKING.save(deps.storage, info.sender.clone(), &coin.amount)?;
    Ok(Response::new().add_attribute("action", "stake").add_attribute("user", info.sender.to_string()))
}

pub fn execute_withdraw(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128
) -> Result<Response, ContractError> {
    check_owner(deps.storage, info.sender.clone())?;
    let msg: CosmosMsg = BankMsg::Send { 
        to_address: info.sender.clone().to_string(), 
        amount: vec![Coin {
            denom: DENOM.to_string(),
            amount
            }]
        }.into();
    Ok(Response::new().add_message(msg))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetStaking { address } => to_binary(&query_staking_info(deps, address)?)
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let response = ConfigResponse {
        owner: config.owner,
        fee: config.fee   
    };
    Ok(response)
}

fn query_staking_info(deps: Deps, address: Addr) -> StdResult<Uint128> {
    let staking_info = STAKING.load(deps.storage, address)?;
    Ok(staking_info)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, crate::ContractError> {
    let version = get_contract_version(deps.storage)?;
    if version.contract != CONTRACT_NAME {
        return Err(ContractError::CannotMigrate {
            previous_contract: version.contract,
        });
    }
    Ok(Response::default())
}

#[test]
fn test() {
    let mut string_array: Vec<String> = Vec::new();
    string_array.push ("result1".to_string());
    string_array.push ("result2".to_string());
    string_array.push ("result3".to_string());

    string_array.retain_mut(|x| x!=&String::from("result1"));
    println!("{:?}", string_array)
}
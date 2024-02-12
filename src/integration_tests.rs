use cw_multi_test::{ App, Contract, ContractWrapper, Executor, BankSudo, SudoMsg };
use cosmwasm_std::{ Empty, testing::mock_env, Addr, StdResult, Timestamp, BlockInfo, to_binary, Uint128, Coin };
use crate::{msg::{ InstantiateMsg, QueryMsg, ExecuteMsg }, state::ConfigResponse};
use crate::state::Config;

pub type Extension = Option<Empty>;

fn mock_app() -> App {
    App::default()
}

pub fn staking_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::execute,
        crate::contract::instantiate,
        crate::contract::query
    );
    Box::new(contract)
}

fn native_token_balance(router: &mut App, address: &Addr) -> Uint128 {
    let native_token_balance = router
        .wrap()
        .query_balance(address.clone(), "usei".to_string())
        .unwrap();
    native_token_balance.amount
}

fn init_staking_contract(router: &mut App) -> Addr {
    let msg = InstantiateMsg {
        fee: Uint128::new(10000),
    };
    let staking_id = router.store_code(staking_contract());
    let staking_address = router
        .instantiate_contract(
            staking_id, 
            Addr::unchecked("admin"),
            &msg, 
            &[], 
            "STAKING", 
            Some("admin".to_string())
        ).unwrap();
    staking_address
}

#[test]
fn test_staking() {
    let mut router = mock_app();
    let mut env = mock_env();
    let admin = Addr::unchecked("admin");
    let user = Addr::unchecked("user");
    let staking_contract = init_staking_contract(&mut router);
    let ts = Timestamp::from_seconds(1);
    router.set_block(BlockInfo {
        height: 1,
        time: ts,
        chain_id: "1".to_string()
    });
    
    router
        .sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: admin.clone().to_string(),
            amount: vec![Coin {
                denom: "usei".to_string(),
                amount: Uint128::new(10000000),
            }],
        }))
        .unwrap();
    router
        .sudo(SudoMsg::Bank(BankSudo::Mint {
            to_address: user.clone().to_string(),
            amount: vec![Coin {
                denom: "usei".to_string(),
                amount: Uint128::new(10000000),
            }],
        }))
        .unwrap();
    let balance = native_token_balance(&mut router, &admin);
    println!("---------native-balance: {:?}", balance);
    router.execute_contract(
        admin.clone(), 
        staking_contract.clone(), 
        &ExecuteMsg::Stake {  },
        &[Coin {
            denom: "usei".to_string(),
            amount: Uint128::new(20000),
        }]
    ).unwrap();
    let balance = native_token_balance(&mut router, &admin);
    println!("---------native-balance: {:?}", balance);
    router.execute_contract(
        user.clone(), 
        staking_contract.clone(), 
        &ExecuteMsg::Stake {  },
        &[Coin {
            denom: "usei".to_string(),
            amount: Uint128::new(20000),
        }]
    ).unwrap();

    let config: ConfigResponse = router.wrap().query_wasm_smart(
        staking_contract.clone(), 
        &QueryMsg::GetConfig {}
    ).unwrap();
    println!("-----------{:?}----------{:?}", config.owner, config.fee.to_string());
    let stake_info: Uint128 = router.wrap().query_wasm_smart(
        staking_contract.clone(), 
        &QueryMsg::GetStaking { address: admin.clone() }
    ).unwrap();
    println!("----------stake_info: {:?}", stake_info.to_string());

    router.execute_contract(
        admin.clone(), 
        staking_contract.clone(), 
        &ExecuteMsg::WithdrawSei { amount: Uint128::new(40000) }, 
        &[]
    ).unwrap();
    let balance = native_token_balance(&mut router, &admin);
    println!("---------native-balance: {:?}", balance);
}

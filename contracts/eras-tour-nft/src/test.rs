// swift_fan related data: persistent because swift_fan must pay for himself
// authorizations: temporary because it should not be restored
// metadata: instance because the contract "admin" should pay

// must have functionalities
// 1. nft token admin minting 10 tickets as a token
// 2. transferring the token to a ticket purchaser
// 2.1 metadata to reflect the concert seating
// 3. burning the unsold tokens

#![cfg(test)]
extern crate std;

use crate::{contract::ErasNftContract, ErasNftContractClient};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};

fn create_token<'a>(env: &Env, admin: &Address) -> ErasNftContractClient<'a> {
    let token = ErasNftContractClient::new(env, &env.register_contract(None, ErasNftContract {}));
    token.initialize(admin, &"Eras Tour".into_val(env), &"Eras".into_val(env));
    token
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::random(&env);

    let swift_fan_1 = Address::random(&env);
    let swift_fan_2 = Address::random(&env);
    let swift_fan_3 = Address::random(&env);
    let eras_token = create_token(&env, &admin);

    // @todo:
    // mint 50 nfts with a different metadata to indicate a seat number
    // eras_token.mint(&500);

    assert_eq!(
        env.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    eras_token.address.clone(),
                    symbol_short!("mint"),
                    (&swift_fan_1, 1000_i128).into_val(&env),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // assert_eq!(eras_token.balance(), 1000);
}
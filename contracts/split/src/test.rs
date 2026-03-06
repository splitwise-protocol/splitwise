#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String, vec};

#[test]
fn test_create_and_join_group() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitwiseContract, ());
    let client = SplitwiseContractClient::new(&env, &contract_id);

    // Generate addresses for testing
    let creator = Address::generate(&env);
    let member = Address::generate(&env);

    let group_name = String::from_str(&env, "Trip to Paris");
    
    // 1. Test create_group
    let group_id = client.create_group(&group_name, &creator);
    assert_eq!(group_id, 1);

    let group = client.get_group(&group_id);
    assert_eq!(group.name, group_name);
    assert_eq!(group.members, vec![&env, creator.clone()]);

    // 2. Test join_group
    client.join_group(&group_id, &member);

    let updated_group = client.get_group(&group_id);
    assert_eq!(updated_group.members, vec![&env, creator.clone(), member.clone()]);
}

#[test]
#[should_panic(expected = "Group not found")]
fn test_join_non_existent_group() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(SplitwiseContract, ());
    let client = SplitwiseContractClient::new(&env, &contract_id);

    let member = Address::generate(&env);

    // Trying to join group 999 which doesn't exist
    client.join_group(&999, &member);
}

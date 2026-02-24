use super::test_utils::{create_test_group, mint_tokens, setup_test_env};
use crate::base::types::GroupMember;
use crate::AutoShareContractClient;
use soroban_sdk::{testutils::Address as _, Address, Vec};

#[test]
fn test_get_member_earnings_tracks_cumulative_totals() {
    let test_env = setup_test_env();
    let env = test_env.env;
    let contract = test_env.autoshare_contract;
    let token = test_env.mock_tokens.get(0).unwrap().clone();
    let client = AutoShareContractClient::new(&env, &contract);

    let member1 = Address::generate(&env);
    let member2 = Address::generate(&env);

    let mut members = Vec::new(&env);
    members.push_back(GroupMember {
        address: member1.clone(),
        percentage: 70,
    });
    members.push_back(GroupMember {
        address: member2.clone(),
        percentage: 30,
    });

    let creator = test_env.users.get(0).unwrap().clone();
    let id = create_test_group(&env, &contract, &creator, &members, 10u32, &token);

    let sender = test_env.users.get(1).unwrap().clone();

    // Initial earnings should be 0
    assert_eq!(client.get_member_earnings(&member1, &id), 0);
    assert_eq!(client.get_member_earnings(&member2, &id), 0);

    // First distribution: 1000 tokens
    mint_tokens(&env, &token, &sender, 1000);
    client.distribute(&id, &token, &1000, &sender);

    // Verify first distribution earnings
    assert_eq!(client.get_member_earnings(&member1, &id), 700);
    assert_eq!(client.get_member_earnings(&member2, &id), 300);

    // Second distribution: 500 tokens
    mint_tokens(&env, &token, &sender, 500);
    client.distribute(&id, &token, &500, &sender);

    // Verify cumulative earnings
    assert_eq!(client.get_member_earnings(&member1, &id), 700 + 350);
    assert_eq!(client.get_member_earnings(&member2, &id), 300 + 150);

    // Third distribution with different member
    let member3 = Address::generate(&env);
    assert_eq!(client.get_member_earnings(&member3, &id), 0);
}

#[test]
fn test_get_member_earnings_returns_zero_for_non_existent_group() {
    let test_env = setup_test_env();
    let env = test_env.env;
    let contract = test_env.autoshare_contract;
    let client = AutoShareContractClient::new(&env, &contract);

    let member = Address::generate(&env);
    let id = crate::BytesN::<32>::from_array(&env, &[0u8; 32]);

    assert_eq!(client.get_member_earnings(&member, &id), 0);
}

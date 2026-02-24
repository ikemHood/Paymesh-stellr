use crate::test_utils::{create_test_group, create_test_members, setup_test_env};
use crate::AutoShareContractClient;
use soroban_sdk::testutils::Events;

#[test]
fn test_start_fundraising_success() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let members = create_test_members(&test_env.env, 2);
    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    let target_amount = 5000i128;
    client.start_fundraising(&group_id, &creator, &target_amount);

    let status = client.get_fundraising_status(&group_id);
    assert_eq!(status.target_amount, target_amount);
    assert_eq!(status.total_raised, 0);
    assert!(status.is_active);

    // Verify event
    let events = test_env.env.events().all();
    if !events.is_empty() {
        let last_event = events.last().unwrap();
        // FundraisingStarted { group_id, target_amount }
        // topics: [FundraisingStarted, group_id]
        // data: target_amount
        let event_amount: i128 = soroban_sdk::FromVal::from_val(&test_env.env, &last_event.2);
        assert_eq!(event_amount, target_amount);
    }
}

#[test]
#[should_panic]
fn test_start_fundraising_fail_not_creator() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let non_creator = test_env.users.get(1).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let members = create_test_members(&test_env.env, 2);
    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    let target_amount = 5000i128;
    client.start_fundraising(&group_id, &non_creator, &target_amount);
}

#[test]
#[should_panic]
fn test_start_fundraising_fail_already_active() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let members = create_test_members(&test_env.env, 2);
    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    let target_amount = 5000i128;
    client.start_fundraising(&group_id, &creator, &target_amount);

    // Try to start again
    client.start_fundraising(&group_id, &creator, &target_amount);
}

#[test]
#[should_panic]
fn test_start_fundraising_fail_group_inactive() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let members = create_test_members(&test_env.env, 2);
    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    client.deactivate_group(&group_id, &creator);

    let target_amount = 5000i128;
    client.start_fundraising(&group_id, &creator, &target_amount);
}

#[test]
#[should_panic]
fn test_start_fundraising_fail_invalid_amount() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let members = create_test_members(&test_env.env, 2);
    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    client.start_fundraising(&group_id, &creator, &0i128);
}

#[test]
#[should_panic]
fn test_start_fundraising_fail_paused() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let members = create_test_members(&test_env.env, 2);
    let group_id = create_test_group(
        &test_env.env,
        &test_env.autoshare_contract,
        &creator,
        &members,
        10,
        &token,
    );

    client.pause(&test_env.admin);

    let target_amount = 5000i128;
    client.start_fundraising(&group_id, &creator, &target_amount);
}

use crate::autoshare_logic::DataKey;
use crate::base::types::FundraisingConfig;
use crate::test_utils::setup_test_env;
use crate::AutoShareContractClient;
use soroban_sdk::BytesN;

#[test]
fn test_get_fundraising_status_default() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let group_id = BytesN::from_array(&test_env.env, &[1u8; 32]);

    let status = client.get_fundraising_status(&group_id);

    assert_eq!(status.target_amount, 0);
    assert_eq!(status.total_raised, 0);
    assert!(!status.is_active);
}

#[test]
fn test_get_fundraising_status_existing() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let group_id = BytesN::from_array(&test_env.env, &[2u8; 32]);

    // Manually set fundraising data in storage to simulate it being populated by start_fundraising
    let config = FundraisingConfig {
        target_amount: 1000,
        total_raised: 500,
        is_active: true,
    };

    let key = DataKey::GroupFundraising(group_id.clone());
    test_env.env.as_contract(&test_env.autoshare_contract, || {
        test_env.env.storage().persistent().set(&key, &config);
    });

    let status = client.get_fundraising_status(&group_id);

    assert_eq!(status.target_amount, 1000);
    assert_eq!(status.total_raised, 500);
    assert!(status.is_active);
}

#[test]
fn test_get_contributions_empty() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let group_id = BytesN::from_array(&test_env.env, &[3u8; 32]);
    let user = test_env.users.get(0).unwrap();

    let group_contributions = client.get_group_contributions(&group_id);
    let user_contributions = client.get_user_contributions(&user);

    assert_eq!(group_contributions.len(), 0);
    assert_eq!(user_contributions.len(), 0);
}

#[test]
fn test_get_contributions_populated() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let group_id = BytesN::from_array(&test_env.env, &[4u8; 32]);
    let contributor = test_env.users.get(0).unwrap();
    let token = test_env.mock_tokens.get(0).unwrap();
    let amount = 1000i128;
    let timestamp = 123456u64;

    let contribution = crate::base::types::FundraisingContribution {
        group_id: group_id.clone(),
        contributor: contributor.clone(),
        token: token.clone(),
        amount,
        timestamp,
    };

    // Simulate contribution by manually setting storage
    let group_key = DataKey::GroupContributions(group_id.clone());
    let user_key = DataKey::UserContributions(contributor.clone());

    test_env.env.as_contract(&test_env.autoshare_contract, || {
        let mut group_list = soroban_sdk::Vec::new(&test_env.env);
        group_list.push_back(contribution.clone());
        test_env
            .env
            .storage()
            .persistent()
            .set(&group_key, &group_list);

        let mut user_list = soroban_sdk::Vec::new(&test_env.env);
        user_list.push_back(contribution.clone());
        test_env
            .env
            .storage()
            .persistent()
            .set(&user_key, &user_list);
    });

    let group_contributions = client.get_group_contributions(&group_id);
    assert_eq!(group_contributions.len(), 1);
    let gc = group_contributions.get(0).unwrap();
    assert_eq!(gc.amount, amount);
    assert_eq!(gc.contributor, contributor);

    let user_contributions = client.get_user_contributions(&contributor);
    assert_eq!(user_contributions.len(), 1);
    let uc = user_contributions.get(0).unwrap();
    assert_eq!(uc.group_id, group_id);
    assert_eq!(uc.amount, amount);
}

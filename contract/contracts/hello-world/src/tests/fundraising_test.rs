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

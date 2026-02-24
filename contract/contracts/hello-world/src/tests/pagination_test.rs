use crate::test_utils::{create_test_group, setup_test_env};
use crate::AutoShareContractClient;
use soroban_sdk::{testutils::Address as _, Address, Vec};

#[test]
fn test_get_groups_paginated() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let creator = test_env.users.get(0).unwrap().clone();
    let token = test_env.mock_tokens.get(0).unwrap().clone();

    let mut members = Vec::new(&test_env.env);
    members.push_back(crate::base::types::GroupMember {
        address: Address::generate(&test_env.env),
        percentage: 100,
    });

    // Create 25 groups
    for i in 1..=25 {
        create_test_group(
            &test_env.env,
            &test_env.autoshare_contract,
            &creator,
            &members,
            i, // unique usages -> unique ID
            &token,
        );
    }

    // Test first page
    let page1 = client.get_groups_paginated(&0, &10);
    assert_eq!(page1.groups.len(), 10);
    assert_eq!(page1.total, 25);
    assert_eq!(page1.offset, 0);
    assert_eq!(page1.limit, 10);

    // Test second page
    let page2 = client.get_groups_paginated(&10, &10);
    assert_eq!(page2.groups.len(), 10);
    assert_eq!(page2.offset, 10);

    // Test third page (remaining 5)
    let page3 = client.get_groups_paginated(&20, &10);
    assert_eq!(page3.groups.len(), 5);
    assert_eq!(page3.offset, 20);

    // Test limit cap (should cap at 20)
    let page_capped = client.get_groups_paginated(&0, &50);
    assert_eq!(page_capped.groups.len(), 20);
    assert_eq!(page_capped.limit, 20);

    // Test offset out of bounds
    let page_empty = client.get_groups_paginated(&30, &10);
    assert_eq!(page_empty.groups.len(), 0);
    assert_eq!(page_empty.total, 25);
}

#[test]
fn test_get_groups_paginated_empty() {
    let test_env = setup_test_env();
    let client = AutoShareContractClient::new(&test_env.env, &test_env.autoshare_contract);

    let page = client.get_groups_paginated(&0, &10);
    assert_eq!(page.groups.len(), 0);
    assert_eq!(page.total, 0);
}

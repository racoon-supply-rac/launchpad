use cosmwasm_std::{coins, Timestamp};
use cw_multi_test::Executor;
use sg_std::{GENESIS_MINT_START_TIME, NATIVE_DENOM};

use open_edition_minter::msg::ConfigResponse;
use open_edition_minter::msg::{ExecuteMsg, QueryMsg};

use crate::common_setup::setup_accounts_and_block::setup_block_time;
use crate::common_setup::setup_minter::common::constants::MAX_TOKEN_LIMIT;
use crate::common_setup::templates::{
    open_edition_minter_custom_template, OpenEditionMinterCustomParams,
};

const MINT_PRICE: u128 = 100_000_000;

#[test]
fn check_max_tokens_limit_init() {
    let vt = open_edition_minter_custom_template(
        None,
        Some(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_000)),
        None,
        Some(10),
        Some(2),
        None,
        Some(MAX_TOKEN_LIMIT + 1),
        OpenEditionMinterCustomParams::default(),
        None,
        None,
    );
    // if the number of tokens to be minted exceed to max, should error
    assert!(vt.is_err());

    // Should work otherwise
    let vt = open_edition_minter_custom_template(
        None,
        Some(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_000)),
        None,
        Some(10),
        Some(2),
        None,
        Some(2),
        OpenEditionMinterCustomParams::default(),
        None,
        None,
    );
    assert!(vt.is_ok());
    let vt = vt.unwrap();

    let (mut router, creator, buyer) = (vt.router, vt.accts.creator, vt.accts.buyer);
    let minter_addr = vt.collection_response_vec[0].minter.clone().unwrap();
    // Set to a valid mint time
    setup_block_time(&mut router, GENESIS_MINT_START_TIME + 101, None);

    // Check the Config for the num_tokens value
    let query_config_msg = QueryMsg::Config {};
    let res: ConfigResponse = router
        .wrap()
        .query_wasm_smart(minter_addr.clone(), &query_config_msg)
        .unwrap();
    assert_eq!(res.num_tokens, Some(2));

    // Only the first 2 mints
    for _ in 1..=2 {
        let mint_msg = ExecuteMsg::Mint {};
        let res = router.execute_contract(
            buyer.clone(),
            minter_addr.clone(),
            &mint_msg,
            &coins(MINT_PRICE, NATIVE_DENOM),
        );
        assert!(res.is_ok());
    }

    // 3rd mint fails from exceeding num of tokens
    let mint_msg = ExecuteMsg::Mint {};
    let res = router.execute_contract(
        creator,
        minter_addr,
        &mint_msg,
        &coins(MINT_PRICE, NATIVE_DENOM),
    );
    assert_eq!(
        res.err().unwrap().source().unwrap().to_string(),
        "Sold out"
    );
}

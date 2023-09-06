use cosmwasm_std::Timestamp;
use sg_std::GENESIS_MINT_START_TIME;
use crate::common_setup::templates::{
    open_edition_minter_custom_template, OpenEditionMinterCustomParams,
};

#[test]
fn invalid_code_id() {
    // Set an invalid code id for the nft contract
    let vt = open_edition_minter_custom_template(
        None,
        Some(Timestamp::from_nanos(GENESIS_MINT_START_TIME + 10_000)),
        None,
        Some(10),
        Some(5),
        None,
        None,
        OpenEditionMinterCustomParams::default(),
        None,
        Some(19),
    );
    assert_eq!(
        vt.err()
            .unwrap()
            .err()
            .unwrap()
            .source()
            .unwrap()
            .to_string(),
        "InvalidCollectionCodeId 19".to_string()
    );

    // All the other tests related to Sudo params of the factory contract are tested in the factory
    // tests
}

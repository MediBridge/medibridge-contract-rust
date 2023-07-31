use near_sdk::env;

/// Assert that 1 Yocto NEAR was attached
pub fn assert_one_yocto_near() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 Yocto NEAR"
    )
}

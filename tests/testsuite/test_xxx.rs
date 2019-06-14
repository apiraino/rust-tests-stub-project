use cratetests::common::utils;

#[test]
fn test_1() {
    assert!(true, 2 == 1);

    utils::in_utils::from_utils();
    utils::outside_mod();
}

use cratetests::common;

#[test]
fn test_2() {
    assert!(true, 2 == 1);

    common::fn_in_mod();
    // common::utils::in_utils::from_utils();
    common::utils::outside_mod();
}

use math;

#[test]
fn test_add() {
    assert_eq!(math::add(1, 2), 3);
    assert_eq!(math::add(-1, 1), 0);
    assert_eq!(math::add(0, 0), 0);
}

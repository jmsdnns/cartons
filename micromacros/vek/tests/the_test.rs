use vek::vek;

#[test]
fn test_vek_bang() {
    let v = vek![1, 3, 5];
    assert_eq!(vec![1, 3, 5], v);
}

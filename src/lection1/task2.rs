pub fn is_power_of_two(n: i32) -> bool { 
    let zero_val_expected: i32 = n & n - 1;
    return zero_val_expected == 0;
}

#[test]
fn sort_test() {
	assert_eq!(2 + 2, 4);
}
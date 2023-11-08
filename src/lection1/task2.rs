pub fn is_power_of_two(n: i32) -> bool {
    if n == 0 {
        return false;
    }
    let zero_val_expected: i32 = n & n - 1;
    return zero_val_expected == 0;
}

#[test]
fn is_power_of_two_test() {
	assert_eq!(is_power_of_two(0), false);
	assert_eq!(is_power_of_two(1), true);
	assert_eq!(is_power_of_two(2), true);
	assert_eq!(is_power_of_two(3), false);
	assert_eq!(is_power_of_two(4), true);
	assert_eq!(is_power_of_two(5), false);
}
pub fn add_digits(num: i32) -> i32 {
    let base: i32 = 10;
    let mut res: i32 = 0;
    let mut x_copied = num;
    while x_copied > 0
    {
        res += x_copied % base;
        x_copied /= base;
    }
    if res > 9{
        res = add_digits(res);
    }
    return res;
}

#[test]
fn add_digits_test() {
	assert_eq!(add_digits(19), 1);
}
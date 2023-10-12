pub fn add_digits(num: i32) -> i32 {
    let base: i32 = 10;
    let mut res: i32 = 0;
    let mut x_copied = num;
    while x_copied > 0
    {
        res += x_copied % base;
        x_copied /= base;
    }
    if res > 10{
        res = add_digits(res);
    }
    return res;
}

#[test]
fn sort_test() {
	assert_eq!(2 + 2, 4);
}
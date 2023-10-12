pub fn reverse_bits(x: u32) -> u32 {
    let mut base: u32 = u32::pow(2, 31);
    let mut res: u32 = 0;
    let mut x_copied = x;
    while x_copied != 0
    {
        res += (x_copied & 1) * (base);
        base >>= 1;
        x_copied >>= 1;
    }

    return res;
}    
    
#[test]
fn sort_test() {
	assert_eq!(2 + 2, 4);
}
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {return false;}
    if x < 10 {return true};
    let mut v: Vec<i32> = Vec::new();
    let mut x_copied = x;
    while x_copied > 0 {
        v.push(x_copied % 10);
        x_copied /= 10;
    }
    println!("{}", x);
    for i in 0..(v.len()/2) {
        let rightmost = v[i];
        let leftmost = v[v.len() - i - 1];
        println!("left {}  right {}", leftmost, rightmost);
        if rightmost != leftmost {
            //println!("Nope");
            return false;
        }
    }
    return true;
}

#[test]
fn sort_test() {
	assert_eq!(2 + 2, 4);
}
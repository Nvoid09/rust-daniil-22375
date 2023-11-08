#[derive(Debug)]
pub struct Element <T, U> {
    pub re: T,
    pub im: U
}

pub fn sort<T>(arr : &mut [T], comp: fn(&T, &T)->bool) {
    let mut b:bool = true;
    let size: usize = arr.len();
	if size > 1 {
		while b {
			b = false;
			for i in 0..size-1{
				if comp(&arr[i], &arr[i + 1]){
					arr.swap(i, i + 1);
					b = true;
				}
			}
		}
	}
}

#[test]
fn sort_test() {
	type EL = Element<i32,u64>;
	let el_small: EL = Element {re:1,im:2};
	let el_big: EL = Element {re:4,im:5};
	let compare_two_elements = |a:&EL, b:&EL| -> bool { 
		let left = (a.re*a.re) as i64 + (a.im*a.im) as i64;
		let right = (b.re*b.re) as i64  + (b.im*b.im) as i64;
		return left > right;
	};
	assert_eq!(compare_two_elements(&el_small, &el_big), false);
	assert_eq!(compare_two_elements(&el_big, &el_small), true);

	let mut el_ar : [Element<i32,u64>; 5] = 
	[
		Element {re:1,im:2},
		Element {re:3,im:5},
		Element {re:3,im:4},
		Element {re:5,im:4},
		Element {re:0,im:0}
	];

	let el_ar_correct  : [Element<i32,u64>; 5] = 
	[
		Element {re:0,im:0},
		Element {re:1,im:2},
		Element {re:3,im:4},
		Element {re:3,im:5},
		Element {re:5,im:4}
	];

	sort(&mut el_ar, compare_two_elements);
	for i in 0..5{
		assert_eq!(el_ar[i].re, el_ar_correct[i].re);
		assert_eq!(el_ar[i].im, el_ar_correct[i].im);
	}
}
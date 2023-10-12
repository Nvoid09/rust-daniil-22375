pub fn sort(arr : &mut [i32], comp: fn(&i32, &i32)->bool) {
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
	assert_eq!(2 + 2, 4)
}

// fn main() {
// 	let mut array = [5,4,3,2,1];
// 	println!("{:?}", array);
// 	sort(&mut array, |a:&i32, b:&i32| -> bool { a > b });
// 	println!("{:?}", array);
// }
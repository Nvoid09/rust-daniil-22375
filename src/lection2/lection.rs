struct Point{
    x: i32,
    y: i32
}

// impl Point {
//     fn name_method(&self) -> Self{
//         println!("Point class");
//         Self(1,2)
//     }
// }

trait Say{
    fn say(&self);
}

impl Say for i32{
    fn say(&self){
        println!("HIII");
    }
}

fn main() {
	let array = [5,4,3,2,1];
	println!("{:?}", array);
    let a: i32 = 0;
    a.say();
    // if 2 < 0 {
    //     panic!("2 < 0");
    //     unreachable!();
    // }
}


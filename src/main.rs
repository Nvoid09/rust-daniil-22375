mod lection1 { 
    pub mod task1;
    pub mod task2; 
    pub mod task3;
    pub mod task4;
    pub mod task5;
}

fn main() {
    //task1
    println!("\n==task1==\n");
    let mut array = [5,4,3,2,1];
	println!("{:?}", array);
	lection1::task1::sort(&mut array, |a:&i32, b:&i32| -> bool { a > b });
	println!("{:?}", array);
    //task2
    println!("\n==task2==\n");
    let mut n = 1;
    while n < 17 {
        println!("{} - {}", n, lection1::task2::is_power_of_two(n));
        n+=1;
    }
    println!("Ready");
    //task3
    println!("\n==task3==\n");
    let n: u32 = 0b11111111111111111111111111111111;
    println!("{}", n);
    let a:u32 = (u64::pow(2, 32) - 1) as u32;
    println!("{}", a);
    let n = 0b00000010100101000001111010011100;
    println!("{}", lection1::task3::reverse_bits(n));
    //task4
    println!("\n==task4==\n");
    println!("{}", lection1::task4::add_digits(38));
    //task5
    println!("\n==task5==\n");
    println!("{}", lection1::task5::is_palindrome(121));
    println!("{}", lection1::task5::is_palindrome(1121));
    println!("{}", lection1::task5::is_palindrome(11211));
}
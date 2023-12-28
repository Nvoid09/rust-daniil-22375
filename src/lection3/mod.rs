pub mod task1;
pub mod task2;
use std::str::FromStr;

pub fn lec3_check(){
    let mut tree = task1::BinaryTree::Empty;
    tree.add(10);
    tree.add(5);
    tree.add(15);
    tree.add(1);
    tree.add(2);
    tree.add(3);
    tree.add(0);
    tree.add(10);
    tree.add(30);
    tree.print();

    for value in &tree {
        println!("{}", value);
    }

    // for value in &mut tree {
    //     let old_val = *value;
    //     *value += 1;
    //     println!("mut {} - {}", value, old_val);
    // }


    println!("Check ser:");
    let tree_in_str: String = tree.to_string();
    println!("{}", tree_in_str);
    println!("Check deser:");
    let another_tree = task1::BinaryTree::<i32>::from_str(&tree_in_str).unwrap();
    another_tree.print();
}
pub mod task1;
pub mod task2;

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

    for value in &mut tree {
        let old_val = *value;
        *value += 1;
        println!("mut {} - {}", value, old_val);
    }
}
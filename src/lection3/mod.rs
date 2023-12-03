mod task1;
mod task2;

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

}
#[derive(PartialEq, Debug)]
pub enum BinaryTree<T> {//==option
   Empty,
   NonEmpty(Box<TreeNode<T>>)
}
#[derive(Debug)]
pub struct TreeNode<T> {//==tree
   pub element: T,
   pub left: BinaryTree<T>,
   pub right: BinaryTree<T>,
}


impl<T: Ord + Copy + std::fmt::Display> BinaryTree<T> {
   pub fn add(&mut self, value : T) {
      match self {
         Self::Empty => {
            let node = Box::new(TreeNode {element: value, left: Self::Empty, right: Self::Empty});
            *self = BinaryTree::NonEmpty(node);
         },
         Self::NonEmpty(node) => {
            if value < node.element {
               BinaryTree::add(&mut node.left, value);
            } else {
               BinaryTree::add(&mut node.right, value);
            }
         }
      }
   }
   
   pub fn print(&self) {
      match self {
         Self::Empty => println!("Empty Tree"),
         Self::NonEmpty(_) => {
            self.recursive_print(2);
         }
      }
   }

   fn recursive_print(&self, mut step: i32){
      match self {
         Self::Empty => {}
         Self::NonEmpty(fetch) => {
            let count = 2;
            step += count;
            Self::recursive_print(&fetch.right, step);
            for _ in 2*count..step {
               print!("  ");
            }
            println!("{}", fetch.element);
            Self::recursive_print(&fetch.left, step);
         }
      }
   }
}


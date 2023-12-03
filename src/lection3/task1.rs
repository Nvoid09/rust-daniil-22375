pub enum BinaryTree<T> {
   Empty,
   NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
   element: T,
   left: BinaryTree<T>,
   right: BinaryTree<T>,
}

// use std::cmp::Ordering;

// impl Ord for BinaryTree<T> {
//    fn cmp(&self, other: &Self) -> Ordering {
//          self.height.cmp(&other.height)
//    }
// }

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
      //Box::new(value);
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

// #[test]
// pub fn test_1(){
// }

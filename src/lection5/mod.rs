use crate::lection3::task1::*;

//ссылка
pub struct BinaryTreeIterator<'a, T> {
    tree_stack: Vec<&'a BinaryTree<T>>,
}
impl<'a, T> Iterator for BinaryTreeIterator<'a, T> {
type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(subtree) = self.tree_stack.pop() {
            match subtree {
                BinaryTree::NonEmpty(node) => {
                    let TreeNode //= &**node;
                    {
                        element, 
                        left, 
                        right 
                    } = &**node;

                    self.tree_stack.push(right);
                    self.tree_stack.push(left);
                    return Some(element);
                },
                _ => {}
            }
        }
        None
    }
}
//ссылка std::iter::IntoIterator
impl<'a, T> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = BinaryTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let root = vec!(self);
        BinaryTreeIterator { tree_stack: root }
    }
}
//значение
pub struct BinaryTreeIntoIterator<T> {
    tree_stack: Vec<BinaryTree<T>>,
}
impl<T> Iterator for BinaryTreeIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(subtree) = self.tree_stack.pop() {
            match subtree {
                BinaryTree::NonEmpty(node) => {
                    let TreeNode
                    {
                        element,
                        left,
                        right 
                    } = *node;

                    self.tree_stack.push(right);
                    self.tree_stack.push(left);
                    return Some(element);
                },
                _ => {}
            }
        }
        None
    }
}
//значение std::iter::IntoIterator
impl<T: Ord + std::fmt::Display> IntoIterator for BinaryTree<T> {
    type Item = T;
    type IntoIter = BinaryTreeIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        let root = vec!(self);
        BinaryTreeIntoIterator { tree_stack: root }
    }
}
//мут ссылка
pub struct BinaryTreeMutIterator<'a, T> {
    tree_stack: Vec<&'a mut BinaryTree<T>>,
}
impl<'a, T> std::iter::Iterator for BinaryTreeMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(subtree) = self.tree_stack.pop() {
            match subtree {
                BinaryTree::NonEmpty(node) => {
                    let TreeNode 
                    { 
                        element, 
                        left, 
                        right 
                    } = &mut **node;
                    
                    self.tree_stack.push(right);
                    self.tree_stack.push(left);
                    return Some(element);
                },
                _ => {}
            }
        }
        None
    }
}
//мут ссылка std::iter::IntoIterator
impl<'a, T> IntoIterator for &'a mut BinaryTree<T> {
    type Item = &'a mut T;
    type IntoIter = BinaryTreeMutIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
       let root = vec!(self);
       BinaryTreeMutIterator { tree_stack: root }
    }
}


//FromIterator
impl<T: Ord + Copy + std::fmt::Display> FromIterator<T> for BinaryTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = BinaryTree::Empty;
        for value in iter {
            tree.add(value);
        }
        tree
    }
}

//DoubleEndedIterator
impl<'a, T> std::iter::DoubleEndedIterator for BinaryTreeIterator<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(subtree) = self.tree_stack.pop() {
            match subtree {
                BinaryTree::NonEmpty(node) => {
                    let TreeNode {
                        element, 
                        left, 
                        right 
                    } = &**node;

                    self.tree_stack.push(left);
                    self.tree_stack.push(right);
                    return Some(element);
                },
                _ => {}
            }
        }
        None
    }
}


//tests
impl<T: PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.element == other.element && self.left == other.left && self.right == other.right
    }
}

#[test]
fn test_collect() {
    let vector = vec![10, 5, 1, 0, 2, 3, 15, 10, 30];
    let mut expect_tree = BinaryTree::Empty;
    for el in &vector {
       expect_tree.add(*el);
    }
    let collect_tree: BinaryTree<_> = vector.into_iter().collect();

    assert_eq!(collect_tree, expect_tree);
}
#[test]
fn test_map() {
    let mut init_tree = BinaryTree::Empty;
    let mut expect_tree = BinaryTree::Empty;
    for el in 1..5 {
       init_tree.add(el);
       expect_tree.add(el*el);
       //println!("{}", el);
    }
    let tree: BinaryTree<_> = init_tree.into_iter()
        .map(|x| x * x)
        .collect();

    assert_eq!(tree, expect_tree);
}
#[test]
fn test_filter() {
    let mut init_tree = BinaryTree::Empty;
    let mut expect_tree = BinaryTree::Empty;
    for el in 1..10 {
       init_tree.add(el);
       if el % 2 == 0{
          expect_tree.add(el);
        }
    }
    let filter_tree: BinaryTree<_> = init_tree.into_iter()
        .filter(|&x| {x % 2 == 0})
        .collect();

    assert_eq!(filter_tree, expect_tree);
}

//(de)serializer 
use super::task1::*;

//#[derive(Debug, PartialEq, Eq)]
#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl <T: Sized + std::fmt::Debug> std::fmt::Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res_vec: Vec<&T> = self.into_iter().collect();
        write!(f, "{:?}", res_vec)
    }
}

impl <T: Sized + std::str::FromStr + Copy + std::fmt::Display + Ord> std::str::FromStr for BinaryTree<T> {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sv = s.strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .ok_or("Error")?;
        
        let v: Vec<&str> = sv.split(", ").collect();

        let mut res_from_str = BinaryTree::<T>::Empty; 
        for el in v {
            let val = el.parse::<T>().map_err(|_| "Error")?;
            res_from_str.add(val);
        }
        
        Ok(res_from_str)
    }
}


#[test]
fn serializer_test(){
    let mut tree = BinaryTree::Empty;
    tree.add(10);
    tree.add(5);
    tree.add(15);
    tree.add(1);
    tree.add(2);
    tree.add(3);
    tree.add(0);
    tree.add(10);
    tree.add(30);
    assert_eq!("[10, 5, 1, 0, 2, 3, 15, 10, 30]", tree.to_string());
}

#[test]
fn deserializer_test(){
    let mut tree = BinaryTree::Empty;
    tree.add(10);
    tree.add(5);
    tree.add(15);
    tree.add(1);
    tree.add(2);
    tree.add(3);
    tree.add(0);
    tree.add(10);
    tree.add(30);

    let tree_in_str = "[10, 5, 1, 0, 2, 3, 15, 10, 30]";

    let another_tree:BinaryTree<i32> = std::str::FromStr::from_str(&tree_in_str).unwrap();
    assert_eq!(tree, another_tree);
}
/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::vec;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => self.root = Some(Box::new(TreeNode::<T>::new(value))),
            Some(node) => {
               node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        let mut result = false;
        match &self.root {
            None => result = false,
            Some(node) => {
                // put root into queue
                let mut queue: Vec<&Box<TreeNode<T>>> = vec![];
                queue.push(node);

                while let Some(current_node) = queue.pop() {
                    if current_node.value == value {
                        result = true
                    }

                    // compare value if equal return true
                    // if false, judge has left or right child , put them into the queue
                    if let Some(ref left_child) = current_node.left {
                        queue.push(left_child);
                    }

                    if let Some(ref right_child) = current_node.right {
                        queue.push(right_child);
                    } 
  
                }
            }
        }
        result
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value == self .value {
            return
        }
        match value < self.value {
            true => {
                // value goes to left tree
                match &mut self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::<T>::new(value)));
                    }
                    Some(left_child) => {
                        left_child.insert(value);
                    }
                }
            }
            false => {
                // value goes to right tree
                match &mut self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::<T>::new(value)));
                    }
                    Some(right_child) => {
                        right_child.insert(value);
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    



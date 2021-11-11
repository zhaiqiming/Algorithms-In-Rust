//! Binary-Tree

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    /// Create a new TreeNode
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }

  /// For Leetcoder No.95
  /// Give you an integer n, please generate and return all different binary search trees composed of N nodes with different node values from 1 to n. 
  /// You can return answers in any order.
  // pub fn generate_trees() -> Vec<Option<Rc<RefCell<TreeNode>>>> {

  // }

  /// For Leetcoder No.94
  /// 中序遍历二叉树【迭代】
  pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack = vec![];
    let mut ans = vec![];
    if root.is_none() {
      return ans;
    } 

    while !stack.is_empty() || !root.is_none() {
      while !root.is_none() {
          let node = root.unwrap();
          root = node.borrow_mut().left.take();  
          stack.push(node);
      }

      root = stack.pop();
      ans.push(root.as_ref().unwrap().borrow().val);
      root = root.unwrap().borrow_mut().right.take();
    }
    return ans;
  }

  /// For Leetcoder No.94
  /// 中序遍历二叉树【递归】
  pub fn inorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    if root.is_none() {
      return ans;
    }

    if !root.as_ref().unwrap().borrow().left.is_none()  {
      let left_ans = Self::inorder_traversal_recursion(root.as_ref().unwrap().borrow().left.clone());
      for item in left_ans.iter() {
        ans.push(*item);
      }
    }
    
    ans.push(root.as_ref().unwrap().borrow().val);
    
    if !root.as_ref().unwrap().borrow().right.is_none()  {
      let right_ans = Self::inorder_traversal_recursion(root.as_ref().unwrap().borrow().right.clone());
      for item in right_ans.iter() {
        ans.push(*item);
      }
    }
    ans
  }

  /// For Leetcoder No.94
  /// 中序遍历二叉树【递归】
  pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut que = VecDeque::new();
    if root.is_none() {
      return ans;
    } else {
      que.push_back(root.clone());
    }
    
    while !que.is_empty() {
      let mut temp = vec![];

      for _ in 0..que.len() {
        let now = que.pop_front().unwrap();
        let now_inner = now.as_ref().unwrap().borrow_mut();
        temp.push(now_inner.val);
        
        if now_inner.left.is_some() {
          que.push_back(now_inner.left.clone());
        }
        if now_inner.right.is_some() {
          que.push_back(now_inner.right.clone());
        }
      }
      ans.push(temp);
    }
    return ans;
  }
}


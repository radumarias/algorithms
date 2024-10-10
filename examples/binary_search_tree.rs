use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: Ord + Debug> TreeNode<T> {
    // Create a new tree node
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if value < self.value {
            match self.left {
                Some(ref mut left) => left.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut right) => right.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }

    fn remove(&mut self, value: T) -> Option<TreeNode<T>> {
        if value == self.value {
            // need to remove the current node
            match self.left.take() {
                Some(mut left) => {
                    let mut deepest_right = left.deepest_right();
                    deepest_right.right = self.right.take();
                    return Some(*left);
                }
                None => match self.right.take() {
                    Some(right) => return Some(*right),
                    None => {}
                },
            }
        } else if value < self.value {
            // need to remove from the left
            match self.left {
                Some(ref mut left) => {
                    if left.value == value {
                        match left.left.take() {
                            // re-balance left side
                            Some(mut left_left) => {
                                let mut deepest_right = left_left.deepest_right();
                                deepest_right.right = left.right.take();
                                self.left = Some(left_left)
                            }
                            None => self.left = left.right.take(),
                        }
                    } else {
                        left.remove(value);
                    }
                }
                None => {}
            }
        } else {
            match self.right {
                Some(ref mut right) => {
                    if right.value == value {
                        match right.left.take() {
                            // re-balance left side
                            Some(mut right_left) => {
                                let mut deepest_right = right_left.deepest_right();
                                deepest_right.right = right.right.take();
                                self.right = Some(right_left)
                            }
                            None => self.right = right.right.take(),
                        }
                    } else {
                        right.remove(value);
                    }
                }
                None => {}
            }
        }
        None
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> Option<&Self> {
        if value == self.value {
            Some(self)
        } else if value < self.value {
            match self.left {
                Some(ref left) => left.search(value),
                None => None,
            }
        } else {
            match self.right {
                Some(ref right) => right.search(value),
                None => None,
            }
        }
    }

    fn deepest_left(&mut self) -> &mut Self {
        loop {
            match self.left {
                Some(ref mut left) => return left.deepest_left(),
                None => return self,
            }
        }
    }

    fn deepest_right(&mut self) -> &mut Self {
        loop {
            match self.right {
                Some(ref mut right) => return right.deepest_right(),
                None => return self,
            }
        }
    }

    // Inorder Traversal
    fn inorder(&self) {
        if let Some(ref left) = self.left {
            left.inorder();
        }
        println!("{:?}", self.value);
        if let Some(ref right) = self.right {
            right.inorder();
        }
    }

    // Preorder Traversal
    fn preorder(&self) {
        println!("{:?}", self.value);
        if let Some(ref left) = self.left {
            left.preorder();
        }
        if let Some(ref right) = self.right {
            right.preorder();
        }
    }

    // Postorder Traversal
    fn postorder(&self) {
        if let Some(ref left) = self.left {
            left.postorder();
        }
        if let Some(ref right) = self.right {
            right.postorder();
        }
        println!("{:?}", self.value);
    }

    // Helper function to validate the BST
    fn is_valid_bst(&self, min: Option<&T>, max: Option<&T>) -> bool {
        if let Some(min_val) = min {
            if self.value <= *min_val {
                return false;
            }
        }

        if let Some(max_val) = max {
            if self.value >= *max_val {
                return false;
            }
        }

        let left_is_valid = match &self.left {
            Some(left) => left.is_valid_bst(min, Some(&self.value)),
            None => true,
        };

        let right_is_valid = match &self.right {
            Some(right) => right.is_valid_bst(Some(&self.value), max),
            None => true,
        };

        left_is_valid && right_is_valid
    }

    // Method to find the height of the BST
    fn height(&self) -> i32 {
        let left_height = match &self.left {
            Some(left) => left.height(),
            None => -1,
        };

        let right_height = match &self.right {
            Some(right) => right.height(),
            None => -1,
        };

        1 + std::cmp::max(left_height, right_height)
    }

    // Function to find the lowest common ancestor
    fn lowest_common_ancestor(&self, p: &T, q: &T) -> &T {
        if p < &self.value && q < &self.value {
            // Both nodes are in the left subtree
            match &self.left {
                Some(left) => left.lowest_common_ancestor(p, q),
                None => &self.value, // This case should not happen if p and q are in the tree
            }
        } else if p > &self.value && q > &self.value {
            // Both nodes are in the right subtree
            match &self.right {
                Some(right) => right.lowest_common_ancestor(p, q),
                None => &self.value, // This case should not happen if p and q are in the tree
            }
        } else {
            // The current node is the lowest common ancestor
            &self.value
        }
    }
}

fn main() {
    let mut root = TreeNode::new(5);
    root.insert(3);
    root.insert(8);
    root.insert(1);
    root.insert(4);
    root.insert(9);

    println!("Tree: {:?}", root);
    println!("Search 4: {:?}", root.search(4));
    println!("Search 6: {:?}", root.search(6));

    root.remove(8);
    println!("Search 8: {:?}", root.search(8));
    println!("Search 1: {:?}", root.search(1));
    println!("Search 9: {:?}", root.search(9));
    let new_root = root.remove(5).unwrap();
    println!("new root: {:?}", new_root);
    println!("Search 5: {:?}", new_root.search(5));
    println!("Search 1: {:?}", new_root.search(1));
}

fn search_in_rotated_array(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid as usize] == target {
            return mid as i32;
        }

        // Determine which part is sorted
        if arr[left as usize] <= arr[mid as usize] {
            // Left part is sorted
            if target >= arr[left as usize] && target < arr[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // Right part is sorted
            if target > arr[mid as usize] && target <= arr[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    -1
}
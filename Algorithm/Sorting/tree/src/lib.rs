struct TreeNode<T> {
    data: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialEq + PartialOrd + Copy> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            data: val,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        let update = if self.data < value {
            &mut self.right
        } else {
            &mut self.left
        };
        match update {
            Some(update) => update.insert(value),
            None => *update = Some(Box::new(TreeNode::new(value))),
        }
    }

    fn transform(&self) -> Vec<T> {
        let mut elements = vec![];
        if let Some(left) = &self.left {
            let left_elements = left.transform();
            elements.extend_from_slice(&left_elements);
        }

        elements.push(self.data);

        if let Some(right) = &self.right {
            let right_elements = right.transform();
            elements.extend_from_slice(&right_elements);
        }

        elements
    }
}

pub fn tree_sort<T: Ord + Copy>(array: &mut [T]) -> Vec<T> {
    let len = array.len();
    if len == 0 {
        return array.to_vec();
    }
    let mut tree_node = TreeNode::new(array[0]);
    for i in 1..len {
        tree_node.insert(array[i]);
    }

    tree_node.transform()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        assert_eq!(tree_sort(&mut v), vec![1, 3, 4, 6, 8, 11, 13])
    }
}

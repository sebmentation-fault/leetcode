mod solution;
use solution::Solution;
use solution::ListNode;

impl ListNode {
    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &value in vec.iter().rev() {
            let new_node = ListNode { val: value, next: current };
            current = Some(Box::new(new_node));
        }
        current
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        let mut current = Some(self);
        while let Some(node) = current {
            vec.push(node.val);
            current = node.next.as_deref();
        }
        vec
    }
}

fn main() {
    let l1 = ListNode::from_vec(vec![2, 4, 3]); // Represents the number 342
    let l2 = ListNode::from_vec(vec![5, 6, 4]); // Represents the number 465

    let result = Solution::add_two_numbers(l1, l2);

    // Convert the result linked list back to a vector for easy display
    let result_vec = result.unwrap().to_vec();
    println!("{:?}", result_vec); // Expected output: [7, 0, 8] which represents the number 807
}


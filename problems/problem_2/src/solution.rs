pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_overflow(l1, l2, 0)
    }

    fn add_overflow(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>, mut overflow: i32) -> Option<Box<ListNode>> {
        // handle base case
        if l1.is_none() && l2.is_none() && overflow == 0 {
            return None;
        }

        // get the running total of the values
        if l1.is_some() {
            overflow += l1.as_ref().map_or(0, |node| node.val);
            l1 = l1.and_then(|node| node.next);
        }
        if l2.is_some() {
            overflow += l2.as_ref().map_or(0, |node| node.val);
            l2 = l2.and_then(|node| node.next);
        }

        Some(
            Box::new(
                ListNode {
                    val: overflow % 10,
                    next: Self::add_overflow(l1, l2, overflow / 10)
                }
            )
        )
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

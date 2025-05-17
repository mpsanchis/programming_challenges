#[derive(Debug)]
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
  fn new_from(val: i32, next: Option<Box<ListNode>>) -> Self {
    ListNode {
      next,
      val
    }
  }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut l1 = &l1;
  let mut l2 = &l2;
  let mut val_1 = l1.as_ref().map_or(0, |node| node.val);
  let mut val_2 = l2.as_ref().map_or(0, |node| node.val);
  let mut sum = val_1 + val_2;
  let mut new_val = sum % 10;
  let mut carry = sum / 10;
  let mut solution = Option::Some(Box::new(ListNode::new(new_val)));
  let mut current_digit = &mut solution;
  //println!("current digit (start): {current_digit:?}");

  loop {
    l1 = l1.as_ref().map_or(&Option::None, |node| &node.next);
    val_1 = l1.as_ref().map_or(0, |node| node.val);
    l2 = l2.as_ref().map_or(&Option::None, |node| &node.next);
    val_2 = l2.as_ref().map_or(0, |node| node.val);

    if l1.is_none() && l2.is_none() && carry == 0 {
      break;
    }

    sum = val_1 + val_2 + carry;
    new_val = sum % 10;
    carry = sum / 10;

    let new_digit = Option::Some(Box::new(ListNode::new(new_val)));
    current_digit.as_mut().unwrap().next = new_digit;
    current_digit = &mut current_digit.as_mut().unwrap().next;
    //println!("current digit: {current_digit:?}");
  }

  solution
}


fn main() {
  let l1_1 = Option::Some(Box::new(ListNode::new(8)));
  let l1 = Option::Some(Box::new(ListNode::new_from(0, l1_1)));
  let l2_1 = Option::Some(Box::new(ListNode::new(3)));
  let l2 = Option::Some(Box::new(ListNode::new_from(0, l2_1)));
  // https://leetcode.com/problems/add-two-numbers/description/
  let sol = add_two_numbers(l1, l2);
  println!("solution: {sol:?}");
}

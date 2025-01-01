use std::ptr::NonNull;

pub mod spinlock;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct List {
    head: Option<NonNull<ListNode>>,
}

struct IntoIter {
    head: Option<NonNull<ListNode>>,
}

impl IntoIterator for List {
    type Item = i32;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { head: self.head }
    }
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if let Some(node) = self.head.take() {
                self.head = (*node.as_ptr())
                    .next
                    .take()
                    .map(|n| NonNull::new_unchecked(Box::into_raw(n)));
                Some((*node.as_ptr()).val)
            } else {
                None
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        use std::fmt::Write;

        let l1 = List {
            head: l1.map(|node| unsafe { NonNull::new_unchecked(Box::into_raw(node)) }),
        };
        let l2 = List {
            head: l2.map(|node| unsafe { NonNull::new_unchecked(Box::into_raw(node)) }),
        };

        let s1 = l1.into_iter().fold(String::new(), |mut output, i| {
            let _ = write!(output, "{}", i);
            output
        });
        let s2 = l2.into_iter().fold(String::new(), |mut output, i| {
            let _ = write!(output, "{}", i);
            output
        });

        let bits1 = decimal_to_binary(&s1.chars().rev().collect::<String>());
        println!("{:?}", bits1);

        let bits2 = decimal_to_binary(&s2.chars().rev().collect::<String>());
        println!("{:?}", bits2);

        let sum = add_binary(bits1, bits2);
        println!("{:?}", sum);

        let sum = binary_to_decimal(sum);

        unsafe {
            let dummy =
                NonNull::new_unchecked(Box::into_raw(Box::new(ListNode { val: 0, next: None })));
            let mut tail = dummy;
            for c in sum.chars().rev() {
                let node = NonNull::new_unchecked(Box::into_raw(Box::new(ListNode {
                    val: c.to_digit(10).unwrap() as i32,
                    next: None,
                })));
                let boxed_node = Box::from_raw(node.as_ptr());
                (*tail.as_ptr()).next = Some(boxed_node);
                tail = node;
            }
            (*dummy.as_ptr()).next.take()
        }
    }
}

fn decimal_to_binary(decimal: &str) -> Vec<u8> {
    let mut num = decimal.to_string();
    let mut binary = Vec::new();

    while num != "0" {
        let mut carry = 0;
        let mut new_num = String::new();

        for c in num.chars() {
            let digit = carry * 10 + c.to_digit(10).unwrap();
            carry = digit % 2;
            let new_digit = digit / 2;
            if !(new_num.is_empty() && new_digit == 0) {
                new_num.push(std::char::from_digit(new_digit, 10).unwrap());
            }
        }

        binary.push(carry as u8);
        num = if new_num.is_empty() {
            "0".to_string()
        } else {
            new_num
        };
    }

    binary.reverse();
    binary
}

fn binary_to_decimal(binary: Vec<u8>) -> String {
    let mut decimal = vec![0u8]; // Start with 0 in decimal

    for &bit in &binary {
        multiply_by_two_in_place(&mut decimal);
        if bit == 1 {
            add_one_in_place(&mut decimal);
        }
    }

    // Convert Vec<u8> to String
    let mut decimal_string = String::new();
    let mut leading_zero = true;
    for &digit in &decimal {
        if digit != 0 || !leading_zero {
            decimal_string.push((digit + b'0') as char);
            leading_zero = false;
        }
    }

    if decimal_string.is_empty() {
        decimal_string.push('0');
    }

    decimal_string
}

fn multiply_by_two_in_place(number: &mut Vec<u8>) {
    let mut carry = 0;

    for digit in number.iter_mut().rev() {
        let num = *digit * 2 + carry;
        carry = num / 10;
        *digit = num % 10;
    }

    if carry > 0 {
        number.insert(0, carry);
    }
}

fn add_one_in_place(number: &mut Vec<u8>) {
    let mut carry = 1;

    for digit in number.iter_mut().rev() {
        let num = *digit + carry;
        carry = num / 10;
        *digit = num % 10;
        if carry == 0 {
            break;
        }
    }

    if carry > 0 {
        number.insert(0, carry);
    }
}

fn add_binary(b1: Vec<u8>, b2: Vec<u8>) -> Vec<u8> {
    let mut res = vec![];
    let b1 = b1.iter().rev().collect::<Vec<_>>();
    let b2 = b2.iter().rev().collect::<Vec<_>>();

    let mut i1 = b1.iter();
    let mut i2 = b2.iter();

    let mut carry: Option<u8> = None;

    loop {
        let b1 = i1.next();
        let b2 = i2.next();

        if b1.is_none() && b2.is_none() {
            if let Some(c) = carry {
                res.push(c);
            }
            break;
        }

        let mut sum: u8 = b1
            .map_or_else(|| Some(0), |c| Some(**c))
            .map(|c| c + b2.map_or(0, |c| **c))
            .unwrap();
        if let Some(c) = carry {
            sum += c;
        }
        if sum > 1 {
            carry = Some(1);
            sum %= 2;
        } else {
            carry = None;
        }
        res.push(sum);
    }

    res.iter().rev().copied().collect::<Vec<_>>()
}

fn main() {
    let l1 = ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode { val: 9, next: None })),
        })),
    };
    let l2 = ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        })),
    };

    let mut res = Solution::add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    while let Some(node) = res {
        println!("{}", node.val);
        res = node.next;
    }
}

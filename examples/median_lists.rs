use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = Solution::find_median_sorted_arrays(nums1, nums2);
    println!("Median: {}", result);
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut finder = MedianFinder::new();
        for num in nums1 {
            finder.add_num(num);
        }
        for num in nums2 {
            finder.add_num(num);
        }
        finder.find_median()
    }
}

struct MedianFinder {
    lower_half: BinaryHeap<i32>,
    upper_half: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder {
            lower_half: BinaryHeap::new(),
            upper_half: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.lower_half.push(num);
        if let Some(Reverse(min)) = self.upper_half.peek() {
            if *self.lower_half.peek().unwrap() > *min {
                self.upper_half
                    .push(Reverse(self.lower_half.pop().unwrap()));
                self.lower_half.push(self.upper_half.pop().unwrap().0);
            }
        }

        if self.lower_half.len() > self.upper_half.len() + 1 {
            self.upper_half
                .push(Reverse(self.lower_half.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        if self.lower_half.len() > self.upper_half.len() {
            *self.lower_half.peek().unwrap() as f64
        } else {
            (self.lower_half.peek().unwrap() + self.upper_half.peek().unwrap().0) as f64 / 2.0
        }
    }
}

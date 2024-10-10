use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

fn main() {
    let mut min_heap = BinaryHeap::new();

    // Insert elements into the min-heap
    min_heap.push(Reverse(10));
    min_heap.push(Reverse(4));
    min_heap.push(Reverse(7));
    min_heap.push(Reverse(1));

    // Print and remove elements from the min-heap
    while let Some(Reverse(value)) = min_heap.pop() {
        println!("{}", value);
    }

    let mut max_heap = BinaryHeap::new();

    // Insert elements into the max-heap
    max_heap.push(10);
    max_heap.push(4);
    max_heap.push(7);
    max_heap.push(1);

    // Print and remove elements from the max-heap
    while let Some(value) = max_heap.pop() {
        println!("{}", value);
    }

    let mut min_heap = MinHeap::new();

    // Insert elements into the min-heap
    for i in 1..=10 {
        min_heap.push(i);
    }

    // Print and remove elements from the min-heap
    while let Some(value) = min_heap.pop() {
        println!("{}", value);
    }

    let mut arr = [5, 10, 20, 42, 55];
    heap_sort(&mut arr);
    println!("{arr:?}");
}

struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    fn push(&mut self, value: i32) {
        self.data.push(value);
        self.heapify_up();
    }

    fn pop(&mut self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let root = self.data.swap_remove(0);
        self.heapify_down();
        Some(root)
    }

    fn heapify_up(&mut self) {
        let mut index = self.data.len() - 1;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.data[index] >= self.data[parent] {
                break;
            }
            self.data.swap(index, parent);
            index = parent;
        }
    }

    fn heapify_down(&mut self) {
        let mut index = 0;
        let len = self.data.len();
        loop {
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            let mut smallest = index;

            if left < len && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < len && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            self.data.swap(index, smallest);
            index = smallest;
        }
    }
}

fn heap_sort(arr: &mut [i32]) {
    let len = arr.len();

    // Step 1: Build a max-heap from the array
    for start in (0..len / 2).rev() {
        heapify_down(arr, start, len);
    }

    // Step 2: Extract the max element and heapify down
    for end in (1..len).rev() {
        arr.swap(0, end);
        heapify_down(arr, 0, end);
    }
}

fn heapify_down(arr: &mut [i32], start: usize, end: usize) {
    let mut root = start;

    loop {
        let left_child = 2 * root + 1;
        let right_child = 2 * root + 2;
        let mut biggest = root;

        if left_child < end && arr[left_child] > arr[biggest] {
            biggest = left_child;
        }
        if right_child < end && arr[right_child] > arr[biggest] {
            biggest = right_child;
        }
        if biggest == root {
            return;
        }
        arr.swap(root, biggest);
        root = biggest;
    }
}

fn find_kth_largest(nums: Vec<i32>, k: usize) -> i32 {
    let mut heap = BinaryHeap::with_capacity(k);
    for num in nums {
        heap.push(Reverse(num));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.peek().unwrap().0
}

#[derive(Eq, PartialEq)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val) // Reverse order for min-heap
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap = BinaryHeap::new();
    for list in lists {
        if let Some(node) = list {
            heap.push(Reverse(node));
        }
    }

    let mut dummy = ListNode { val: 0, next: None };
    let mut tail = &mut dummy;

    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.0.next.take() {
            heap.push(std::cmp::Reverse(next));
        }
        tail.next = Some(node.0);
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}

fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut frequencry = HashMap::new();
    for num in nums {
        *frequencry.entry(num).or_insert(num) += 1;
    }
    let mut min_heap = BinaryHeap::with_capacity(k);
    for (num, count) in frequencry {
        min_heap.push(Reverse((count, num)));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    min_heap.into_iter().map(|Reverse((_, num))| num).collect()
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

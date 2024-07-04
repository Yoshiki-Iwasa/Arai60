// Step4
// 目的: 指摘修正

use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

#[allow(unused)]
impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for n in nums {
            heap.push(Reverse(n));
            if heap.len() > k {
                heap.pop();
            }
        }

        Self { k, heap }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.k {
            self.heap.pop();
        }
        let Reverse(kth) = self.heap.peek().unwrap();
        *kth
    }
}

// max heap sort
#[allow(unused)]
fn heap_sort_max(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = vec![];
    build_max_heap(&mut nums);
    for _ in 0..nums.len() {
        let last_index = nums.len() - 1;
        nums.swap(0, last_index);
        sorted_nums.push(nums.pop().unwrap());
        max_heapify(&mut nums, 0)
    }
    sorted_nums
}

fn build_max_heap(nums: &mut [i32]) {
    for i in (0..=nums.len() / 2).rev() {
        max_heapify(nums, i)
    }
}

fn max_heapify(nums: &mut [i32], root_index: usize) {
    let Some(mut largest) = nums.get(root_index) else {
        return;
    };

    let left_index = 2 * root_index + 1;
    let right_index = 2 * root_index + 2;

    let mut largest_index = root_index;

    if nums.get(left_index).is_some_and(|left| left > largest) {
        largest_index = left_index;
    }

    // largest indexのアクセスは保証されているのでOK
    largest = &nums[largest_index];

    if nums.get(right_index).is_some_and(|right| right > largest) {
        largest_index = right_index
    }

    if largest_index != root_index {
        nums.swap(largest_index, root_index);
        max_heapify(nums, largest_index)
    }
}

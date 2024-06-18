// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  -

  他の人のコードを読んで考えたこと
  - min heapを使ってk個維持すれば、そこで一番小さいやつがkthになるか

  改善する時にかんがえたこと
  - min heapを使って、k番
  - 自前のヒープソートでもAcceptしたいし、これも書けるようになりたい
*/

use std::{cmp::Reverse, collections::BinaryHeap};

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

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

// min
fn heap_sort_min(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = vec![];
    build_min_heap(&mut nums);
    for _ in 0..nums.len() {
        let last_index = nums.len() - 1;
        nums.swap(0, last_index);
        sorted_nums.push(nums.pop().unwrap());
        min_heapify(&mut nums, 0)
    }
    sorted_nums
}

fn build_min_heap(nums: &mut [i32]) {
    for i in (0..=nums.len() / 2).rev() {
        min_heapify(nums, i)
    }
}

fn min_heapify(nums: &mut [i32], root_index: usize) {
    let Some(root) = nums.get(root_index) else {
        return;
    };

    let left_index = 2 * root_index + 1;
    let right_index = 2 * root_index + 2;

    let mut smallest_index = root_index;

    if nums.get(left_index).is_some_and(|left| left < root) {
        smallest_index = left_index;
    }

    let smallest = nums.get(smallest_index).unwrap();

    if nums.get(right_index).is_some_and(|right| right < smallest) {
        smallest_index = right_index
    }

    if smallest_index != root_index {
        nums.swap(smallest_index, root_index);
        min_heapify(nums, smallest_index)
    }
}

// max heap sort
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
    let Some(root) = nums.get(root_index) else {
        return;
    };

    let left_index = 2 * root_index + 1;
    let right_index = 2 * root_index + 2;

    let mut largest_index = root_index;

    if nums.get(left_index).is_some_and(|left| left > root) {
        largest_index = left_index;
    }

    let largest = nums.get(largest_index).unwrap();

    if nums.get(right_index).is_some_and(|right| right > largest) {
        largest_index = right_index
    }

    if largest_index != root_index {
        nums.swap(largest_index, root_index);
        max_heapify(nums, largest_index)
    }
}

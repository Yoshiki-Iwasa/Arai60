// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - ヒープソートの具体的な内容をしらなかった。
  - あらかじめアルゴリズムイントロダクションで調べた





  何を考えて解いていたか
  - ヒープ条件（max heapの場合）
    完全2分木
    親ノードは子ノードの値より大きい
  - 毎度要素がたされる度にヒープソートすればいい
  - ヒープソートだとtime outになるのはなぜだ

  正解してから気づいたこと
  - 普通にVecについてるsort()をすればよかった
  - クイックソートだとどうなるんだろう
  -
*/

struct KthLargest {
    k: i32,
    nums: Vec<i32>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self { k, nums }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.nums.push(val);

        self.nums.sort_by(|a, b| b.cmp(a));

        *self.nums.get((self.k - 1) as usize).unwrap()
    }

    fn add_with_heap_sort(&mut self, val: i32) -> i32 {
        self.nums.push(val);
        self.nums = heap_sort(self.nums.clone());

        let (k_nums, _) = self.nums.split_at(self.k as usize);
        self.nums = k_nums.to_vec();
        *self.nums.last().unwrap()
    }
}

// fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
//     let mut sorted_nums = vec![];
//     build_max_heap(&mut nums);
//     for _ in 0..nums.len() {
//         let last_index = nums.len() - 1;
//         nums.swap(0, last_index);
//         sorted_nums.push(nums.pop().unwrap());
//         max_heapify(&mut nums, 0)
//     }
//     sorted_nums
// }

// fn build_max_heap(nums: &mut [i32]) {
//     for i in (0..=nums.len() / 2).rev() {
//         max_heapify(nums, i)
//     }
// }

// fn max_heapify(nums: &mut [i32], root_index: usize) {
//     let Some(root) = nums.get(root_index) else {
//         return;
//     };

//     let left_index = 2 * root_index + 1;
//     let right_index = 2 * root_index + 2;

//     let mut largest_index = root_index;

//     if nums.get(left_index).is_some_and(|left| left > root) {
//         largest_index = left_index;
//     }

//     let largest = nums.get(largest_index).unwrap();

//     if nums.get(right_index).is_some_and(|right| right > largest) {
//         largest_index = right_index
//     }

//     if largest_index != root_index {
//         nums.swap(largest_index, root_index);
//         max_heapify(nums, largest_index)
//     }
// }

fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = vec![];
    build_max_heap(&mut nums);
    while !nums.is_empty() {
        let last_index = nums.len() - 1;
        nums.swap(0, last_index);
        sorted_nums.push(nums.pop().unwrap());
        max_heapify(&mut nums, 0);
    }
    sorted_nums
}

fn build_max_heap(nums: &mut [i32]) {
    let end = nums.len() / 2;
    for i in (0..=end).rev() {
        max_heapify(nums, i);
    }
}

fn max_heapify(nums: &mut [i32], root_index: usize) {
    let root = match nums.get(root_index) {
        Some(&val) => val,
        None => return,
    };

    let left_index = 2 * root_index + 1;
    let right_index = 2 * root_index + 2;

    let mut largest_index = root_index;

    if nums.get(left_index).map_or(false, |left| left > &root) {
        largest_index = left_index;
    }

    if nums
        .get(right_index)
        .map_or(false, |right| right > nums.get(largest_index).unwrap())
    {
        largest_index = right_index
    }

    if largest_index != root_index {
        nums.swap(largest_index, root_index);
        max_heapify(nums, largest_index)
    }
}

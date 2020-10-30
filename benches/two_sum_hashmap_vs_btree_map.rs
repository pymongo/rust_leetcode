/*!
test bench_test_two_sum_btree_map ... bench:       1,244 ns/iter (+/- 181)
test bench_two_sum_bitwise        ... bench:       1,113 ns/iter (+/- 142)
test bench_two_sum_hashmap        ... bench:       1,628 ns/iter (+/- 402)
TODO 为什么在Java里面TreeMap(红黑树实现)比HashMap耗时长，但是在Rust中BTreeMap(好像是BST实现)耗时比HashMap短
*/
#![feature(test)]
extern crate leetcode;
extern crate test;
use leetcode::easy::btree_map_two_sum::{
    two_sum_bitwise, two_sum_btree_map, two_sum_hashmap, TEST_CASES,
};

#[bench]
fn bench_test_two_sum_btree_map(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for &(input, target, expected) in TEST_CASES.iter() {
            let nums: Vec<i32> = input.to_vec();
            let result = two_sum_btree_map(nums, target);
            let expected: Vec<i32> = expected.to_vec();
            assert_eq!(result, expected);
        }
    });
}

#[bench]
fn bench_two_sum_hashmap(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for &(input, target, expected) in TEST_CASES.iter() {
            let nums: Vec<i32> = input.to_vec();
            let result = two_sum_hashmap(nums, target);
            let expected: Vec<i32> = expected.to_vec();
            assert_eq!(result, expected);
        }
    });
}

#[bench]
fn bench_two_sum_bitwise(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for &(input, target, expected) in TEST_CASES.iter() {
            let nums: Vec<i32> = input.to_vec();
            let result = two_sum_bitwise(nums, target);
            let expected: Vec<i32> = expected.to_vec();
            assert_eq!(result, expected);
        }
    });
}

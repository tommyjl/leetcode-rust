#![feature(test)]

extern crate test;

use leetcode_rust::hopsnes::*;
use rand::prelude::*;
use test::Bencher;

const SEED: u64 = 12345;

fn make_vecs(left_capacity: usize, right_capacity: usize) -> (Vec<i32>, Vec<i32>) {
    let mut rng = SmallRng::seed_from_u64(SEED);
    let left = new_vec(left_capacity, right_capacity, &mut rng);
    let right = new_vec(right_capacity, 0, &mut rng);
    (left, right)
}

fn new_vec(length: usize, zeroes: usize, rng: &mut SmallRng) -> Vec<i32> {
    let capacity = length + zeroes;
    let mut vec = Vec::with_capacity(capacity);
    for i in 0..capacity {
        let next = if i < length { rng.next_u32() as i32 } else { 0 };
        vec.push(next);
    }
    vec.sort();
    vec
}

macro_rules! mergerino_bench {
    ($($test:ident, $func:ident, $left:literal, $right:literal;)+) => {
        $(
            #[bench]
            fn $test(b: &mut Bencher) {
                let (mut left, mut right) = make_vecs($left, $right);
                b.iter(|| $func(&mut left, &mut right));
            }
        )+
    };
}

mergerino_bench! {
    slow_10k,   mergerino_slow,  10_000, 10_000;
    slow_1k,    mergerino_slow,   1_000,  1_000;

    fast_100k,  mergerino_fast, 100_000, 100_000;
    fast_10k,   mergerino_fast,  10_000,  10_000;
    fast_1k,    mergerino_fast,   1_000,   1_000;

    fast_v2_assign_100k,    mergerino_fast_v2_assign, 100_000, 100_000;
    fast_v2_assign_10k,     mergerino_fast_v2_assign,  10_000,  10_000;
    fast_v2_assign_1k,      mergerino_fast_v2_assign,   1_000,   1_000;

    fast_v2_swap_100k,      mergerino_fast_v2_swap,   100_000, 100_000;
    fast_v2_swap_10k,       mergerino_fast_v2_swap,    10_000,  10_000;
    fast_v2_swap_1k,        mergerino_fast_v2_swap,     1_000,   1_000;

    heap_100k,      mergerino_heap,   100_000, 100_000;
    heap_10k,       mergerino_heap,    10_000,  10_000;
    heap_1k,        mergerino_heap,     1_000,   1_000;
}

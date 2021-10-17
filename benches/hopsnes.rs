#![feature(test)]

extern crate test;

use leetcode_rust::hopsnes::*;
use rand::prelude::*;
use test::Bencher;

const LEFT_CAPACITY: usize = 10_000;
const RIGHT_CAPACITY: usize = 10_000;

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

#[bench]
fn slow(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(69420);
    let mut left = new_vec(LEFT_CAPACITY, RIGHT_CAPACITY, &mut rng);
    let mut right = new_vec(RIGHT_CAPACITY, 0, &mut rng);
    b.iter(|| mergerino_slow(&mut left, &mut right));
}

#[bench]
fn fast(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(69420);
    let mut left = new_vec(LEFT_CAPACITY, RIGHT_CAPACITY, &mut rng);
    let mut right = new_vec(RIGHT_CAPACITY, 0, &mut rng);
    b.iter(|| mergerino_fast(&mut left, &mut right));
}

#[bench]
fn fast_v2(b: &mut Bencher) {
    let mut rng = SmallRng::seed_from_u64(69420);
    let mut left = new_vec(LEFT_CAPACITY, RIGHT_CAPACITY, &mut rng);
    let mut right = new_vec(RIGHT_CAPACITY, 0, &mut rng);
    b.iter(|| mergerino_fast_v2(&mut left, &mut right));
}

#![allow(unused_imports)]
use bittwidhash::BitTwidHash;
use std::hash::{BuildHasher, Hash, Hasher};

const FILEPATH: &str = "test-data/million.txt";
const NLINES: u64 = 1_000_000;

// 150M iterations should collide but even once no more often than 0.1%
// 35M iterations should collide but even once no more often than 0.01%
const NITER: u32 = 35_000_000;

macro_rules! bhash {
    () => {
        BitTwidHash::new_with_secret(std::hash::RandomState::new().build_hasher().finish())
    };
}

#[test]
fn collisions_lines() {
    let bh0 = bhash!();
    let mut resmap = std::collections::BTreeMap::<u64, u64>::new();
    let lines = get_lines();
    for (i, line) in (0..NLINES).zip(lines.flatten()) {
        let mut h = bh0.build_hasher();
        line.hash(&mut h);
        match resmap.insert(h.finish(), i) {
            None => (),
            Some(x) => panic!("Collision between line {} and line {}", i + 1, x + 1),
        };
    }
}

#[test]
#[should_panic]
fn collisions_lines_panics() {
    let bh0 = bhash!();
    let mut resmap = std::collections::BTreeMap::<u64, u64>::new();
    let lines = get_lines();
    for (i, line) in (0..NLINES + 2).zip(lines.flatten()) {
        let mut h = bh0.build_hasher();
        line.hash(&mut h);
        match resmap.insert(h.finish(), i) {
            None => (),
            Some(x) => panic!("Collision between line {} and line {}", i + 1, x + 1),
        };
    }
}

#[test]
fn collisions_num_iter() {
    let bh0 = bhash!();
    let mut resmap = std::collections::HashMap::<u32, u32, BitTwidHash>::with_capacity_and_hasher(
        NITER as usize,
        bh0.build_hasher(),
    );
    for i in 0..NITER {
        match resmap.insert(i, i) {
            None => (),
            Some(x) => panic!("Collision between iteration {} and {}", i + 1, x + 1),
        };
    }
}

//============Helper-Functions================================================
//============================================================================

use std::io::BufRead;

fn get_lines() -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let f = std::fs::File::open(std::path::Path::new(FILEPATH)).expect("Couldn't open file");
    std::io::BufReader::new(f).lines()
}

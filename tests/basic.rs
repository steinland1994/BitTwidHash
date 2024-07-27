use bittwidhash::BitTwidHash;
use std::hash::{BuildHasher, Hash, Hasher};
use std::io::{self, Write};

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

const FILEPATH: &str = "test-data/million.txt";
const NLINES: usize = 1_000;

macro_rules! bhash {
    () => {
        BitTwidHash::new_with_secret(std::hash::RandomState::new().build_hasher().finish())
    };
}

#[test]
fn repeatability_lines() {
    let bh0 = bhash!();
    let mut resvec: Vec<u64> = Vec::new();
    let mut lines = get_lines();
    for (_, line) in (0..NLINES).zip(lines.flatten()) {
        let mut h = bh0.build_hasher();
        line.hash(&mut h);
        resvec.push(h.finish());
    }

    lines = get_lines();
    for (i, line) in (0..NLINES).zip(lines.flatten()) {
        let mut h = bh0.build_hasher();
        line.hash(&mut h);
        if resvec[i] != h.finish() {
            panic!(
                "Hashing is not repeatable, failed at line {}: {}",
                i + 1,
                line
            )
        }
    }
}

#[test]
fn birthday_collisions_lines() {
    let mut wrkarr = [[0u64; 1024]; 64];
    let bh0 = bhash!();
    let lines = get_lines();
    for line in lines.flatten() {
        let mut h = bh0.build_hasher();
        line.hash(&mut h);
        let x = h.finish();
        for i in 0..64 {
            let j = x.rotate_left(i) & 0x3FF;
            wrkarr[i as usize][j as usize] += 1;
        }
    }
    print_colls_1024(wrkarr);
}

#[test]
fn birthday_collisions_numbers() {
    let mut wrkarr = [[0u64; 1024]; 64];
    let bh0 = bhash!();
    for a in 0..1000000 {
        let mut h = bh0.build_hasher();
        a.hash(&mut h);
        let x = h.finish();
        for i in 0..64 {
            let j = x.rotate_left(i) & 0x3FF;
            wrkarr[i as usize][j as usize] += 1;
        }
    }
    print_colls_1024(wrkarr);
}

//============Helper-Functions================================================
//============================================================================

fn get_lines() -> Lines<BufReader<File>> {
    let f = File::open(Path::new(FILEPATH)).expect("Couldn't open file");
    BufReader::new(f).lines()
}

fn print_colls_1024(wrkarr: [[u64; 1024]; 64]) {
    let mut resarr = [[032; 10]; 64];
    let mut resvec: Vec<u64> = Vec::new();
    for (i, mut arr) in (0..64).zip(wrkarr) {
        arr.sort();
        for j in 0..5 {
            resarr[i][j] = arr[j];
            resarr[i][j + 5] = arr[1023 - j];
            resvec.push(arr[j]);
            resvec.push(arr[1023 - j]);
        }
    }
    resvec.sort();

    write!(io::stdout(), "\n\nOverall min:").expect("writeline failed");
    for i in 0..5 {
        write!(io::stdout(), " {:0>4}", resvec[i]).expect("writeline failed");
    }
    resvec.reverse();
    write!(io::stdout(), "\nOverall max:").expect("writeline failed");
    for i in 0..5 {
        write!(io::stdout(), " {:0>4}", resvec[i]).expect("writeline failed");
    }
    write!(io::stdout(), "\n\n").expect("writeline failed");
    for (j, arr) in (0..64).zip(resarr) {
        write!(io::stdout(), "{: >2}: min:", j).expect("writeline failed");
        for i in 0..5 {
            write!(io::stdout(), " {: >4}", arr[i]).expect("writeline failed");
        }
        write!(io::stdout(), "   max:").expect("writeline failed");
        for i in 5..10 {
            write!(io::stdout(), " {: >4}", arr[i]).expect("writeline failed");
        }
        write!(io::stdout(), "\n").expect("writeline failed");

        if ((j + 1) & 3) == 0 {
            write!(io::stdout(), "\n").expect("writeline failed");
        }
    }
}

extern crate rayon;
extern crate test;

use self::rayon::prelude::*;
use self::test::Bencher;
use std::ops::Range;

const NUMS: Range<u64> = 0..1048576;

#[test]
fn test_iter() {
    let res: u64 = NUMS.into_iter()
        .find(|&x| {
            // println!("iter trying {}", x);
            let y = x + 1;
            y > 100 && (y % 17 == 0)
        })
        .unwrap();
    println!("{}", res)
}

#[test]
fn test_par_iter() {
    let res: u64 = NUMS.into_par_iter()
        .find_first(|&x| {
            // println!("par_iter trying {}", x);
            let y = x + 1;
            y > 100 && (y % 17 == 0)
        })
        .unwrap();
    println!("{}", res)
}

#[test]
fn test_par_iter_map() {
    let res: u64 = NUMS.into_par_iter()
        .map(|x| {
            // println!("par_iter_map trying {}", x);
            x + 1
        })
        .find_first(|&x| x > 100 && (x % 17 == 0))
        .unwrap();
    println!("{}", res)
}

#[bench]
fn bench_iter(b: &mut Bencher) {
    b.iter(|| test_iter())
}

#[bench]
fn bench_par_iter(b: &mut Bencher) {
    b.iter(|| test_par_iter())
}

#[bench]
fn bench_par_iter_map(b: &mut Bencher) {
    b.iter(|| test_par_iter_map())
}

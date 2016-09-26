// Copyright (c) 2016 project developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


use std::io;
use std::io::prelude::*;
use std::fs::File;

#[path = "lib.rs"]
mod algo;

fn part1_week1() -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open("./priv/IntegerArray.txt"));

    try!(f.read_to_string(&mut s));
    let mut vals: Vec<usize> = s.lines()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    println!("got vals -> {:?}", algo::merge_sort_and_count_inversions(&mut vals[..]));
    Ok(())
}

fn part1_week2() -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open("./priv/QuickSort.txt"));

    try!(f.read_to_string(&mut s));
    let mut vals: Vec<i32> = s.lines()
        .map(|s| s.trim().parse().unwrap())
        .collect();

    println!("got vals -> {:?}", algo::qsort::quick_sort(&mut vals[..]));
    Ok(())

}

fn main() {
    // part1_week1().unwrap();

    part1_week2();
}

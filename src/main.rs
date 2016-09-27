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
use std::collections::BTreeMap;

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

/// Run the randomized contraction algorithm for the min cut problem and
/// use it on the above graph to compute the min cut.
/// Note:
/// > cargo run | sort -n | uniq -c | sort -n -r | head
fn part1_week3() -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open("./priv/kargerMinCut.txt"));

    try!(f.read_to_string(&mut s));

    let g = s.lines()
        .map(|line|
             line.trim()
             .split('\t')
             .map(|s| s.parse::<u32>().unwrap())
             .collect::<Vec<_>>())
        .map(|mut uvs| {
            (uvs[0], uvs[1..].to_owned())
        })
        .collect::<algo::karger::Graph>();

    for _ in 0 .. 500 {
        println!("{:?}", g.minimum_cut_karger().edges());
    }
    Ok(())
}

fn main() {
    // part1_week1().unwrap();

    //part1_week2();
    part1_week3();
}

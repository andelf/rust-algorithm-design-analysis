// Copyright (c) 2016 project developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#![allow(dead_code)]

use std::io;
use std::io::prelude::*;
use std::fs::File;


#[path = "lib.rs"]
mod algo;

fn part1_week1() -> io::Result<()> {
    use algo::inversions::*;

    let mut s = String::new();
    let mut f = try!(File::open("./priv/IntegerArray.txt"));

    try!(f.read_to_string(&mut s));
    let mut vals: Vec<usize> = s.lines()
                                .map(|s| s.trim().parse().unwrap())
                                .collect();

    println!("got vals -> {:?}",
             merge_sort_and_count_inversions(&mut vals[..]));
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
             .map(|line| {
                 line.trim()
                     .split('\t')
                     .map(|s| s.parse::<u32>().unwrap())
                     .collect::<Vec<_>>()
             })
             .map(|uvs| (uvs[0], uvs[1..].to_owned()))
             .collect::<algo::karger::Graph>();

    for _ in 0..500 {
        println!("{:?}", g.minimum_cut_karger().edges());
    }
    Ok(())
}


/// FIXME: wrong answer, all tests passed. :(
/// How to verify(in R):
/// ```r
/// require(igraph);
/// dat <- read.csv("priv/SCC.txt", sep = ' ', header = F)
/// m <- as.matrix(dat)
/// el <- cbind(m[,"V1"], m[,"V2"])
/// g <- graph.edgelist(el, directed = T)
/// clu <- clusters(g, mode = "strong")
/// sort(clu$csize, decreasing = T)[:5]
/// ```
fn part1_week4() -> io::Result<()> {
    use algo::graphs::Digraph;

    let mut s = String::new();
    let mut f = try!(File::open("./priv/SCC.txt"));

    try!(f.read_to_string(&mut s));

    // Vertices are labeled as positive integers from 1 to 875714.
    // While Digraph counts from 0 to 875713
    let mut g = Digraph::new(875714);

    s.lines()
        .map(|line| {
            let mut it = line.trim()
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap() - 1);
            let u = it.next().unwrap();
            let v = it.next().unwrap();
            g.add_edge(u, v);
        })
        .last();

    let scc = g.kosaraju_sharir_scc();
    println!("got => {:?}", scc.week4_programming_assignment());



    Ok(())
}



#[allow(unused_must_use)]
fn main() {
    // part1_week1();
    // part1_week2();
    // part1_week3();
    part1_week4();
}

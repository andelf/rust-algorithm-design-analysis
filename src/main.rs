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

extern crate petgraph;

fn part1_week5() -> io::Result<()> {
    use self::petgraph::graph::{Graph, NodeIndex};
    use self::petgraph::algo::dijkstra;

    let mut s = String::new();
    let mut f = try!(File::open("./priv/dijkstraData.txt"));
    try!(f.read_to_string(&mut s));

    // an undirected weighted graph with 200 vertices labeled 1 to 200
    let mut g = Graph::<u32, usize>::new();
    for i in 0..200 {
        let ix = g.add_node(i);
    }

    s.lines()
     .map(|line| {
         let segs = line.trim().split('\t').collect::<Vec<_>>();
         let u = segs[0].parse::<usize>().unwrap() - 1;
         let vs = segs[1..]
                      .iter()
                      .map(|tok| {
                          let pair = tok.split(',')
                                        .map(|s| s.parse::<usize>().unwrap())
                                        .collect::<Vec<usize>>();
                          // v, weight
                          (pair[0] - 1, pair[1])
                      })
                      .map(|(v, weight)| {
                          g.add_edge(NodeIndex::new(u), NodeIndex::new(v), weight);
                          g.add_edge(NodeIndex::new(v), NodeIndex::new(u), weight);
                      })
                      .last();
     })
     .last();

    let hm = dijkstra(&g,
                      NodeIndex::new(0),
                      None,
                      |gr, n| gr.edges(n).map(|(n, &e)| (n, e)));

    for i in vec![7, 37, 59, 82, 99, 115, 133, 165, 188, 197] {
        print!("{:?},", hm.get(&NodeIndex::new(i - 1)).unwrap());
    }
    println!("");

    Ok(())
}


#[allow(unused_must_use)]
fn main() {
    // part1_week1();
    // part1_week2();
    // part1_week3();
    // part1_week4();
    part1_week5();
}

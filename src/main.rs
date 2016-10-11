// Copyright (c) 2016 project developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

#![allow(dead_code)]
#![feature(inclusive_range_syntax)]

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

// FIXME: remove usage of petgraph
fn part1_week5() -> io::Result<()> {
    use self::petgraph::graph::{Graph, NodeIndex};
    use self::petgraph::algo::dijkstra;

    let mut s = String::new();
    let mut f = try!(File::open("./priv/dijkstraData.txt"));
    try!(f.read_to_string(&mut s));

    // an undirected weighted graph with 200 vertices labeled 1 to 200
    let mut g = Graph::<u32, usize>::new();
    for i in 0..200 {
        g.add_node(i);
    }

    s.lines()
     .map(|line| {
         let segs = line.trim().split('\t').collect::<Vec<_>>();
         let u = segs[0].parse::<usize>().unwrap() - 1;
         segs[1..]
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


fn part1_week6_1() -> io::Result<()> {
    use std::collections::hash_set::HashSet;

    let mut s = String::new();
    let mut f = try!(File::open("./priv/algo1-programming_prob-2sum.txt"));
    try!(f.read_to_string(&mut s));

    let mut arr: Vec<i64> = s.lines().map(|line| line.trim().parse::<i64>().unwrap()).collect();
    arr.sort();
    arr.dedup();

    let size = arr.len();

    let mut target = HashSet::new();
    for &x in &arr {
        let y_min = -10000 - x;
        let y_max = 10000 - x;

        let mut y_min_pos = arr.binary_search(&y_min).unwrap_or_else(|pos| pos);
        let mut y_max_pos = arr.binary_search(&y_max).unwrap_or_else(|pos| pos);
        if y_min_pos >= size {
            y_min_pos = size - 1;
        }
        if y_max_pos >= size {
            y_max_pos = size - 1;
        }

        for &y in &arr[y_min_pos...y_max_pos] {
            let t = x + y;
            if x != y && t >= -10000 && t <= 10000 {
                target.insert(t);
            }
        }
    }

    println!("result: {}", target.len());
    Ok(())
}


fn part1_week6_2() -> io::Result<()> {
    use algo::heaps::MedianMaintainer;

    let mut s = String::new();
    let mut f = try!(File::open("./priv/Median.txt"));
    try!(f.read_to_string(&mut s));

    let mut mm = MedianMaintainer::new();
    let sum_of_medians: i32 = s.lines()
                               .map(|line| {
                                   let n = line.trim().parse::<i32>().expect("parse ok!");
                                   mm.push(n);
                                   mm.peek_median().cloned().unwrap()
                               })
                               .sum();
    println!("Sum of Medians Modulo: {}", sum_of_medians % 10000);
    Ok(())
}



fn part2_week1_1() -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open("./priv/jobs.txt"));
    try!(f.read_to_string(&mut s));

    let mut jobs: Vec<(i64, i64)> = s.lines()
                                     .skip(1)
                                     .map(|line| {
                                         let pair = line.trim()
                                                        .split(' ')
                                                        .map(|s| s.parse::<i64>().unwrap())
                                                        .collect::<Vec<i64>>();
                                         (pair[0], pair[1])  // (weight, length)
                                     })
                                     .collect();

    jobs.sort_by_key(|j| (j.0 - j.1, j.0));
    jobs.reverse();             // decrease order

    // report the sum of weighted completion times
    let mut acc_time = 0;
    let mut sum_of_weighted_completion_time = 0;
    for j in &jobs {
        acc_time += j.1;
        sum_of_weighted_completion_time += j.0 * acc_time;
    }
    println!("by diff: sum of weighted completion times: {}",
             sum_of_weighted_completion_time);
    Ok(())
}


fn part2_week1_2() -> io::Result<()> {
    let mut s = String::new();
    let mut f = try!(File::open("./priv/jobs.txt"));
    try!(f.read_to_string(&mut s));

    let mut jobs: Vec<(i64, i64)> = s.lines()
                                     .skip(1)
                                     .map(|line| {
                                         let pair = line.trim()
                                                        .split(' ')
                                                        .map(|s| s.parse::<i64>().unwrap())
                                                        .collect::<Vec<i64>>();
                                         (pair[0], pair[1])  // (weight, length)
                                     })
                                     .collect();

    jobs.sort_by(|j, k| (j.0 as f32 / j.1 as f32).partial_cmp(&(k.0 as f32 / k.1 as f32)).unwrap());
    jobs.reverse();             // decrease order

    // report the sum of weighted completion times
    let mut acc_time = 0;
    let mut sum_of_weighted_completion_time = 0;
    for j in &jobs {
        acc_time += j.1;
        sum_of_weighted_completion_time += j.0 * acc_time;
    }
    println!("by ratio: sum of weighted completion times: {}",
             sum_of_weighted_completion_time);
    Ok(())
}

fn part2_week1_3() -> io::Result<()> {
    use algo::mst::{EdgeWeightedGraph, Edge};

    let mut s = String::new();
    let mut f = try!(File::open("./priv/edges.txt"));
    try!(f.read_to_string(&mut s));

    let header: Vec<usize> = s.splitn(3, char::is_whitespace)
                              .take(2)
                              .map(|n| n.parse().unwrap())
                              .collect();
    let number_of_nodes = header[0];
    let _numer_of_edges = header[1];

    // println!("N {} E {}", number_of_nodes, numer_of_edges);

    let mut g = EdgeWeightedGraph::new(number_of_nodes);

    s.lines()
     .skip(1)
     .map(|line| {
         let vals: Vec<i64> = line.trim()
                                  .split(' ')
                                  .map(|s| s.parse::<i64>().unwrap())
                                  .collect();
         let u = vals[0] as usize - 1; // node name starts from 0
         let v = vals[1] as usize - 1;
         let w = vals[2];
         g.add_edge(Edge::new(u, v, w));
     })
     .last();

    println!("Sum of MST weights: {}",
             g.prim_mst().edges().iter().map(|e| e.weight()).sum::<i64>());
    Ok(())
}


fn part2_week2() -> io::Result<()> {
    use algo::clustering::read_test_data_string;

    let mut s = String::new();
    let mut f = try!(File::open("../priv/clustering1.txt"));
    try!(f.read_to_string(&mut s));

    read_test_data_string(&s);

    Ok(())
}

#[allow(unused_must_use)]
fn main() {
    // # Part 1
    // part1_week1();
    // part1_week2();
    // part1_week3();
    // part1_week4();
    // part1_week5();
    // part1_week6_1();
    // part1_week6_2();

    // # Part 2
    // part2_week1_1();
    // part2_week1_2();
    // part2_week1_3();

    part2_week2();
}

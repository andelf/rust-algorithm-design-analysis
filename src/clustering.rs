//! Application to Clustering

extern crate itertools;

use std::collections::hash_map::HashMap;
use self::itertools::Itertools;
use std::fmt;

/// UnionFind Algorithm from rust-adivon
pub struct UnionFind {
    id: Vec<usize>,
    /// number of objects in the tree rooted at i.
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            id: (0..n).collect(),
            rank: vec![0; n],
            count: n,
        }
    }

    // root_of
    pub fn find(&mut self, mut p: usize) -> usize {
        assert!(p < self.id.len());
        while p != self.id[p] {
            self.id[p] = self.id[self.id[p]];    // path compression by halving
            p = self.id[p];
        }
        p
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Are the two sites p and q in the same component?
    pub fn connected(&mut self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);

        if i == j {
            return;
        }
        if self.rank[i] < self.rank[j] {
            self.id[i] = j;
        } else if self.rank[i] > self.rank[j] {
            self.id[j] = i;
        } else {
            self.id[j] = i;
            self.rank[i] += 1;
        }
        self.count -= 1;
    }
}

impl fmt::Display for UnionFind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.id {
            try!(write!(f, "{} ", i));
        }
        Ok(())
    }
}


#[test]
fn test_uf() {
    let mut uf = UnionFind::new(10);
    uf.union(4, 3);
    uf.union(3, 8);
    uf.union(6, 5);
    uf.union(9, 4);
    uf.union(2, 1);
    uf.union(5, 0);
    uf.union(7, 2);
    uf.union(6, 1);
    assert!(uf.count() == 2);
}




pub fn part2_week2_assignment1(s: &str) {
    let number_of_nodes: usize = s.splitn(2, char::is_whitespace)
                                  .next()
                                  .map(|n| n.parse().unwrap())
                                  .unwrap();

    let mut edges = s.lines()
                     .skip(1)
                     .map(|line| {
                         let it = line.trim()
                                      .split(' ')
                                      .map(|s| s.parse().unwrap())
                                      .collect::<Vec<i64>>();
                         (it[0] as usize, it[1] as usize, it[2])
                     })
                     .collect::<Vec<(usize, usize, i64)>>();

    edges.sort_by_key(|t| t.2);
    edges.reverse();

    let mut uf = UnionFind::new(number_of_nodes);

    while uf.count() > 4 {
        if let Some((u, v, _weight)) = edges.pop() {
            uf.union(u - 1, v - 1);
        }
    }


    for &(u, v, w) in edges.iter().rev() {
        if !uf.connected(u - 1, v - 1) {
            println!("maximum spacing => {}", w);
            break;
        }
    }

}


#[inline]
pub fn hamming_distance(a: u32, b: u32) -> u32 {
    (a ^ b).count_ones()
}

pub fn part2_week2_assignment2(s: &str, min_distance: usize) {
    let header = s.splitn(3, char::is_whitespace)
                  .take(2)
                  .map(|n| n.parse::<usize>().unwrap())
                  .collect_vec();

    let number_of_nodes = header[0];
    let number_of_bits = header[1];

    // with duplicated item
    let mut nodes = Vec::with_capacity(number_of_nodes);
    let node_id = s.lines()
                   .skip(1)
                   .take(number_of_nodes)
                   .enumerate()
                   .map(|(id, line)| {
                       let val = u32::from_str_radix(&line.trim().split(' ').join(""), 2).unwrap();
                       nodes.push(val);
                       (val, id)
                   })
                   .collect::<HashMap<_, _>>();

    let mut uf = UnionFind::new(number_of_nodes);

    for (i, &node) in nodes.iter().enumerate() {
        node_id.get(&node).map(|&nid| {
            if nid != i {
                uf.union(i, nid);
            }
        });
    }

    for distance in 1...min_distance {
        for (i, &node) in nodes.iter().enumerate() {
            // 0..24
            for offs in (0..number_of_bits).combinations(distance) {
                let new_node = offs.iter().fold(node, |acc, &off| acc ^ (1 << off));
                node_id.get(&new_node).map(|&nid| {
                    uf.union(i, nid);
                });
            }
        }
    }
    println!("merge distance 2 => {}", uf.count());
}



#[test]
fn test_pa2() {
    let case = "7 5
0 0 0 0 0
1 1 1 1 1
1 1 1 1 0
1 1 1 0 0
1 0 0 1 1
0 0 1 1 1
0 1 1 1 1
";
    part2_week2_assignment2(&case);
}

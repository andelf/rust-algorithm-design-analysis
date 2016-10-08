//! Copied from rust-algs4, rust-adivon

use std::iter;
use std::fmt;
use std::cmp;


/// An index priority queue
pub struct IndexMinPQ<T: PartialOrd> {
    nmax: usize,
    n: usize,
    pq: Vec<usize>,
    qp: Vec<usize>,
    keys: Vec<Option<T>>,
}

impl<T: PartialOrd> IndexMinPQ<T> {
    pub fn with_capacity(nmax: usize) -> IndexMinPQ<T> {
        let mut keys = Vec::new();
        for _ in 0..nmax + 1 {
            keys.push(None);
        }
        IndexMinPQ {
            nmax: nmax,
            n: 0,
            pq: vec![0; nmax + 1],
            qp: vec![usize::max_value(); nmax + 1],
            keys: keys,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, i: usize) -> bool {
        assert!(i < self.nmax, "index out of bounds");
        self.qp[i] != usize::max_value()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    // Associates key with index i
    pub fn insert(&mut self, i: usize, key: T) {
        assert!(i < self.nmax, "index out of bounds");
        if self.contains(i) {
            panic!("index already in pq");
        }
        self.n += 1;
        self.qp[i] = self.n;
        self.pq[self.n] = i;
        self.keys[i] = Some(key);
        let n = self.n;
        self.swim(n)
    }

    pub fn min_index(&self) -> usize {
        assert!(self.n != 0, "priority queue underflow");
        self.pq[1]
    }

    pub fn min_key(&self) -> Option<&T> {
        if self.n == 0 {
            None
        } else {
            self.keys[self.pq[1]].as_ref()
        }
    }

    pub fn del_min(&mut self) -> Option<usize> {
        if self.n == 0 {
            None
        } else {
            let min = self.pq[1];
            let n = self.n;
            self.exch(1, n);
            self.n -= 1;
            self.sink(1);
            self.qp[min] = usize::max_value(); // delete
            // help with gc
            self.keys[self.pq[self.n + 1]] = None;
            self.pq[self.n + 1] = usize::max_value();
            Some(min)
        }
    }

    pub fn key_of(&self, i: usize) -> Option<&T> {
        if i >= self.nmax || !self.contains(i) {
            None
        } else {
            self.keys[i].as_ref()
        }
    }

    pub fn change_key(&mut self, i: usize, key: T) {
        if i >= self.nmax || !self.contains(i) {
            panic!("blah....");
        }
        self.keys[i] = Some(key);
        let p = self.qp[i];
        self.swim(p);
        let p = self.qp[i];
        self.sink(p);
    }

    pub fn decrease_key(&mut self, i: usize, key: T) {
        if i >= self.nmax || !self.contains(i) {
            panic!("decrease_key");
        }
        self.keys[i] = Some(key);
        let p = self.qp[i];
        self.swim(p);
    }

    pub fn increase_key(&mut self, i: usize, key: T) {
        if i >= self.nmax || !self.contains(i) {
            panic!("increase_key");
        }
        self.keys[i] = Some(key);
        let p = self.qp[i];
        self.sink(p);
    }

    pub fn delete(&mut self, i: usize) {
        if i >= self.nmax || !self.contains(i) {
            panic!("delete");
        }
        let index = self.qp[i];
        let n = self.n;
        self.exch(index, n);
        self.n -= 1;
        self.swim(index);
        self.sink(index);
        self.keys[i] = None;
        self.qp[i] = usize::max_value();
    }

    #[inline]
    fn greater(&self, i: usize, j: usize) -> bool {
        self.keys[self.pq[i]] > self.keys[self.pq[j]]
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.qp.swap(self.pq[i], self.pq[j]);
    }

    fn swim(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.greater(k / 2, k) {
            self.exch(k, k / 2);
            k /= 2;
        }
    }

    fn sink(&mut self, k: usize) {
        let mut k = k;
        while 2 * k <= self.n {
            let mut j = 2 * k;
            if j < self.n && self.greater(j, j + 1) {
                j += 1;
            }
            if !self.greater(k, j) {
                break;
            }
            self.exch(k, j);
            k = j;
        }
    }
}

#[test]
fn test_index_min_pq() {
    let strings = vec!["it", "was", "the", "best", "of", "times", "it", "was", "the", "worst"];
    let mut pq = IndexMinPQ::with_capacity(strings.len());

    for (i, s) in strings.iter().enumerate() {
        pq.insert(i, s);
    }

    while !pq.is_empty() {
        let i = pq.del_min().unwrap();
        assert!(!strings[i].is_empty());
    }

    for (i, s) in strings.iter().enumerate() {
        pq.insert(i, s);
    }

    while !pq.is_empty() {
        pq.del_min();
    }
}



/// a weighted edge
#[derive(Clone, Copy)]
pub struct Edge {
    v: usize,
    w: usize,
    weight: i64,
}

impl Edge {
    pub fn new(v: usize, w: usize, weight: i64) -> Edge {
        Edge {
            v: v,
            w: w,
            weight: weight,
        }
    }

    pub fn weight(&self) -> i64 {
        self.weight
    }

    pub fn either(&self) -> usize {
        self.v
    }

    pub fn other(&self, vertex: usize) -> usize {
        if vertex == self.v {
            self.w
        } else if vertex == self.w {
            self.v
        } else {
            panic!("illegal endppint")
        }
    }

    fn swap(mut self) -> Edge {
        let v = self.v;
        self.v = self.w;
        self.w = v;
        self
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<cmp::Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{} {:.5}", self.v, self.w, self.weight)
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}({:.2})", self.v, self.w, self.weight)
    }
}


pub struct EdgeWeightedGraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<Edge>>,
}

/// an edge-weighted graph
impl EdgeWeightedGraph {
    pub fn new(v: usize) -> EdgeWeightedGraph {
        EdgeWeightedGraph {
            v: v,
            e: 0,
            adj: iter::repeat(Vec::new()).take(v).collect::<Vec<Vec<Edge>>>(),
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    #[inline]
    fn validate_vertex(&self, v: usize) {
        assert!(v < self.v, "vertex is not between 0 and max V")
    }

    pub fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.validate_vertex(v);
        self.validate_vertex(w);
        self.adj[v].push(e.clone());
        if v != w {
            self.adj[w].push(e.swap());
        }
        self.e += 1
    }

    // this implements IntoIterator
    pub fn adj(&self, v: usize) -> Vec<Edge> {
        self.validate_vertex(v);
        self.adj[v].iter().map(|e| e.clone()).collect::<Vec<Edge>>()
    }

    pub fn degree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn edges(&self) -> Vec<Edge> {
        self.adj
            .iter()
            .flat_map(|adj| adj.iter().map(|e| e.clone()).collect::<Vec<Edge>>().into_iter())
            .filter(|ref e| e.either() <= e.other(e.either()))
            .collect::<Vec<Edge>>()
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("graph G {\n");
        for i in 0..self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for e in self.edges() {
            let v = e.either();
            let w = e.other(v);
            dot.push_str(&format!("  {} -- {} [ label=\"{}\" ];\n", v, w, e.weight))
        }
        dot.push_str("}\n");
        dot
    }
}


#[test]
fn test_edge_weighted_graph() {
    let mut g = EdgeWeightedGraph::new(6);
    g.add_edge(Edge::new(0, 1, 7));
    g.add_edge(Edge::new(1, 2, 10));
    g.add_edge(Edge::new(0, 2, 9));
    g.add_edge(Edge::new(0, 5, 14));
    g.add_edge(Edge::new(1, 3, 15));
    g.add_edge(Edge::new(2, 5, 2));
    g.add_edge(Edge::new(2, 3, 11));
    g.add_edge(Edge::new(4, 5, 9));
    g.add_edge(Edge::new(3, 4, 6));
    g.add_edge(Edge::new(2, 2, 1));

    assert_eq!(10, g.edges().len());
    assert!(!g.to_dot().is_empty());
}



pub struct PrimMST<'a> {
    graph: &'a EdgeWeightedGraph,
    edge_to: Vec<Option<Edge>>,
    dist_to: Vec<i64>,
    marked: Vec<bool>,
    pq: IndexMinPQ<i64>,
}

impl<'a> PrimMST<'a> {
    fn new<'b>(graph: &'b EdgeWeightedGraph) -> PrimMST<'b> {
        let n = graph.v();
        let edge_to = iter::repeat(None).take(n).collect();
        let dist_to = iter::repeat(i64::max_value()).take(n).collect();
        let marked = iter::repeat(false).take(n).collect();
        let pq = IndexMinPQ::with_capacity(n);

        let mut ret = PrimMST {
            graph: graph,
            edge_to: edge_to,
            dist_to: dist_to,
            marked: marked,
            pq: pq,
        };
        // run Prim for all vertices
        // get a minimum spanning forest
        for v in 0..n {
            if !ret.marked[v] {
                ret.prim(v);
            }
        }
        ret
    }

    // Prim's algorithm, start from vertex s
    fn prim(&mut self, s: usize) {
        self.dist_to[s] = 0;
        self.pq.insert(s, 0);

        while !self.pq.is_empty() {
            let v = self.pq.del_min().unwrap();
            self.scan(v);
        }
    }

    fn scan(&mut self, v: usize) {
        self.marked[v] = true;
        for e in self.graph.adj(v) {
            let w = e.other(v);
            if self.marked[w] {
                continue;
            }
            if e.weight() < self.dist_to[w] {
                self.dist_to[w] = e.weight();
                self.edge_to[w] = Some(e);
                if self.pq.contains(w) {
                    self.pq.decrease_key(w, self.dist_to[w]);
                } else {
                    self.pq.insert(w, self.dist_to[w]);
                }
            }
        }
    }

    pub fn edges(&self) -> Vec<Edge> {
        let mut mst = vec![];
        for e in self.edge_to.iter() {
            e.map(|e| mst.push(e.clone()));
        }
        mst.into_iter().collect::<Vec<Edge>>()
    }
}

impl EdgeWeightedGraph {
    pub fn prim_mst<'a>(&'a self) -> PrimMST<'a> {
        PrimMST::new(self)
    }
}


#[test]
fn test_edge_weighted_graph_mst() {
    let mut g = EdgeWeightedGraph::new(6);
    g.add_edge(Edge::new(0, 1, 7));
    g.add_edge(Edge::new(1, 2, 10));
    g.add_edge(Edge::new(0, 2, 9));
    g.add_edge(Edge::new(0, 5, 14));
    g.add_edge(Edge::new(1, 3, 15));
    g.add_edge(Edge::new(2, 5, 2));
    g.add_edge(Edge::new(2, 3, 11));
    g.add_edge(Edge::new(4, 5, 9));
    g.add_edge(Edge::new(3, 4, 6));
    g.add_edge(Edge::new(2, 2, 1));

    assert_eq!(33_i64, g.prim_mst().edges().iter().map(|e| e.weight).sum());
}

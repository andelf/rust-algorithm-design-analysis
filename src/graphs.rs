// Copy Graph code of week 3 here.

// extern crate adivon;
// pub use self::adivon::graph::directed::{Digraph, KosarajuSharirSCC};

use std::collections::vec_deque::VecDeque;


#[derive(Clone, Debug)]
pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<usize>>
}

impl Digraph {
    pub fn new(v: usize) -> Digraph {
        Digraph {
            v: v,
            e: 0,
            adj: vec![vec![]; v]
        }
    }

    pub fn resize(&mut self, n: usize) {
        assert!(n > self.v, "no need to resize");

        self.v = n;
        self.adj.resize(n, vec![]);
    }

    fn validate_vertex(&self, v: usize) {
        assert!(v < self.v, "vertex is not between 0 and {}", self.v - 1)
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        self.validate_vertex(v);
        self.validate_vertex(w);

        self.e += 1;
        self.adj[v].push(w);
    }

    pub fn outdegree(&self, v: usize) -> usize {
        self.validate_vertex(v);
        self.adj[v].len()
    }

    pub fn number_of_self_loops(&self) -> usize {
        let mut count = 0;
        for v in 0 .. self.v() {
            for &w in self.adj(v) {
                if v == w {
                    count += 1;
                }
            }
        }
        count / 2
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("digraph G {\n");
        for i in 0 .. self.v {
            dot.push_str(&format!("  {};\n", i));
        }

        for (v, adj) in self.adj.iter().enumerate() {
            for w in adj.iter() {
                dot.push_str(&format!("  {} -> {};\n", v, w));
            }
        }
        dot.push_str("}\n");
        dot
    }

    pub fn adj(&self, v: usize) -> &[usize] {
        &self.adj[v]
    }

    pub fn reverse(&self) -> Digraph {
        let v = self.v;
        let mut adj = vec![vec![]; v];

        for s in 0..v {
            for &e in self.adj(s) {
                adj[e].push(s);
            }
        }
        Digraph {
            v: v,
            e: self.e,
            adj: adj
        }
    }

    pub fn dfs(&self, s: usize) -> SearchPaths {
        let mut path = SearchPaths::new(self, SearchSource::Single(s));
        path.dfs();
        path
    }

    pub fn dfs_multi_source<T: IntoIterator<Item=usize>>(&self, s: T) -> SearchPaths {
        let mut path = SearchPaths::new(self, SearchSource::Multi(s.into_iter().collect()));
        path.dfs();
        path
    }

    pub fn bfs(&self, s: usize) -> SearchPaths {
        let mut path = SearchPaths::new(self, SearchSource::Single(s));
        path.bfs();
        path
    }

    pub fn reverse_dfs_postorder(&self) -> ::std::vec::IntoIter<usize> {
        let mut dfo = DepthFirstOrder::new(self);
        dfo.init();
        dfo.reverse_post.into_iter()
    }

    pub fn kosaraju_sharir_scc(&self) -> KosarajuSharirSCC {
        KosarajuSharirSCC::new(self)
    }
}

pub enum SearchSource {
    Single(usize),
    Multi(Vec<usize>)
}

impl SearchSource {
    fn iter(&self) -> ::std::vec::IntoIter<usize> {
        match *self {
            SearchSource::Single(ref i) => vec![*i].into_iter(),
            SearchSource::Multi(ref vs) => vs.clone().into_iter()
        }
    }

    fn contains(&self, v: usize) -> bool {
        match *self {
            SearchSource::Single(ref i) => *i == v,
            SearchSource::Multi(ref vs) => vs.contains(&v)
        }
    }
}

pub struct SearchPaths<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    edge_to: Vec<Option<usize>>,
    source: SearchSource
}

impl<'a> SearchPaths<'a> {
    fn new(graph: &Digraph, source: SearchSource) -> SearchPaths {
        let mut marked = vec![false; graph.v()];
        let edge_to = vec![None; graph.v()];

        for s in source.iter() {
            marked[s] = true;
        }

        SearchPaths {
            graph: graph,
            marked: marked,
            edge_to: edge_to,
            source: source
        }
    }

    fn dfs_from(&mut self, v: usize) {
        self.marked[v] = true;
        for &w in self.graph.adj(v) {
            if !self.marked[w] {
                self.dfs_from(w);
                self.edge_to[w] = Some(v);
            }
        }
    }

    fn dfs(&mut self) {
        for v in self.source.iter() {
            self.dfs_from(v);
        }
    }

    fn bfs(&mut self) {
        let mut q = VecDeque::new();
        for s in self.source.iter() {
            q.push_back(s);
        }
        while !q.is_empty() {
            let v = q.pop_front().unwrap();
            for &w in self.graph.adj(v) {
                if !self.marked[w] {
                    self.edge_to[w] = Some(v);
                    q.push_back(w);
                    self.marked[w] = true;
                }
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Option<Vec<usize>> {
        if self.has_path_to(v) {
            let mut path = vec![];
            let mut x = v;
            while !self.source.contains(x) {
                path.push(x);
                x = self.edge_to[x].unwrap();
            }
            path.push(x);
            Some(path)
        } else {
            None
        }
    }
}

pub struct DepthFirstOrder<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    reverse_post: Vec<usize>
}

impl<'a> DepthFirstOrder<'a> {
    fn new(graph: &Digraph) -> DepthFirstOrder {
        let marked = vec![false; graph.v()];

        DepthFirstOrder {
            graph: graph,
            marked: marked,
            reverse_post: vec![]
        }
    }

    fn init(&mut self) {
        for v in 0 .. self.graph.v() {
            if !self.marked[v] {
                self.dfs(v)
            }
        }
    }

    fn dfs(&mut self, v: usize) {
        self.marked[v] = true;
        for &w in self.graph.adj(v) {
            if !self.marked[w] {
                self.dfs(w);
            }
        }
        self.reverse_post.push(v);
    }
}

/// Compute the strongly-connected components of a digraph using the
/// Kosaraju-Sharir algorithm.
pub struct KosarajuSharirSCC<'a> {
    graph: &'a Digraph,
    marked: Vec<bool>,
    id: Vec<Option<usize>>,
    count: usize,
    vertices: usize,
    _leader: usize,
    final_id: Vec<(usize,usize)>,
}

impl<'a> KosarajuSharirSCC<'a> {
    fn new(graph: &Digraph) -> KosarajuSharirSCC {
        let n = graph.v();
        let mut cc = KosarajuSharirSCC {
            graph: graph,
            marked: vec![false; n],
            id: vec![None; n],
            count: 0,
            vertices: n,
            _leader: 0,
            final_id: vec![],
        };
        cc.init();
        cc
    }

    fn init(&mut self) {
        let g_rev = self.graph.reverse();

        for v in g_rev.reverse_dfs_postorder() {
            if !self.marked[v] {
                self._leader = v;
                self.dfs(v, self.graph);
                self.count += 1;
            }
        }

        let groups = self.id.clone();

        self.marked = vec![false; self.vertices];
        self.id = vec![None; self.vertices];
        self.count = 0;


        for v in g_rev.reverse_dfs_postorder() {
            if !self.marked[v] {
                self._leader = v;
                self.dfs(v, &g_rev);
                self.count += 1;
            }
        }

        // println!("leaders: \nG    : {:?}\nG_rev: {:?}", self.id, groups);

        // node with the same "leader" is the SCC
        self.final_id = self.id.iter()
            .enumerate()
            .map(|(i, id)| {
                let a = id.unwrap();
                let b = groups[i].unwrap();
                (a, b)
            })
            .collect();
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn id(&self, v: usize) -> usize {
        self.id[v].unwrap()
    }

    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    pub fn week4_programming_assignment(&self) -> Vec<usize> {
        use ::std::collections::btree_map::BTreeMap;

        let mut counter = BTreeMap::new();
        // for id in self.id.iter() {
        //     *counter.entry(id.unwrap()).or_insert(0) += 1;
        // }
        for id in self.final_id.iter() {
            *counter.entry(id).or_insert(0) += 1;
        }
        let mut res: Vec<usize> = counter.values().cloned().collect();
        res.sort_by(|a, b| b.cmp(&a));
        res.into_iter().take(5).collect()
    }

    // non recursive version of dfs
    fn dfs(&mut self, v: usize, graph: &Digraph) {
        // used as a stack
        let mut q = VecDeque::with_capacity(65535);
        q.push_back(v);
        while let Some(v) = q.pop_back() {
            if self.marked[v] {
                continue;
            }
            // println!("visiting ... {}", v+1);
            self.marked[v] = true;
            self.id[v] = Some(self._leader);

            q.extend(graph.adj(v).iter());
        }
    }
}

pub fn read_graph_from_string(s: &str) -> Digraph {
    let mut g = Digraph::new(1);

    s.lines()
        .map(|line| {
            let mut it = line.trim()
                .split(' ')
                .map(|s| s.parse::<usize>().unwrap() - 1);
            let u = it.next().unwrap();
            let v = it.next().unwrap();
            if u >= g.v { g.resize(u+1); }
            if v >= g.v { g.resize(v+1); }
            g.add_edge(u, v);
        })
        .last();
    g
}


#[test]
fn test_bad() {
    let case2 = "1 2\n2 3\n3 1\n3 4\n5 4\n6 4\n8 6\n6 7\n7 8";
    let g = read_graph_from_string(case2);
    let scc = g.kosaraju_sharir_scc();
    let r = scc.week4_programming_assignment();
    assert_eq!(r, vec![3,3,1,1]);
}


#[test]
fn test_programming_assigment4() {

    let case0 = "1 2\n2 6\n2 3\n2 4\n3 1\n3 4\n4 5\n5 4\n6 5\n6 7\n7 6\n7 8\n8 5\n8 7";
    let g = read_graph_from_string(case0);
    let r = g.kosaraju_sharir_scc().week4_programming_assignment();
    assert_eq!(r, vec![3, 3, 2]);

    let case1 = "1 4\n2 8\n3 6\n4 7\n5 2\n6 9\n7 1\n8 5\n8 6\n9 7\n9 3";
    let g = read_graph_from_string(case1);
    let r = g.kosaraju_sharir_scc().week4_programming_assignment();
    assert_eq!(r, vec![3,3,3]);

    let case2 = "1 2\n2 3\n3 1\n3 4\n5 4\n6 4\n8 6\n6 7\n7 8";
    let g = read_graph_from_string(case2);
    let scc = g.kosaraju_sharir_scc();
    let r = scc.week4_programming_assignment();
    assert_eq!(r, vec![3,3,1,1]);

    // M-x replace-string ; C-q C-j ENTER \n ENTER
    let case3 = "1 2\n2 3\n3 1\n3 4\n5 4\n6 4\n8 6\n6 7\n7 8\n4 3\n4 6";
    let g = read_graph_from_string(case3);
    let r = g.kosaraju_sharir_scc().week4_programming_assignment();
    assert_eq!(r, vec![7, 1]);

    let case4 = "1 2\n2 3\n2 4\n2 5\n3 6\n4 5\n4 7\n5 2\n5 6\n5 7\n6 3\n6 8\n7 8\n7 10\n8 7\n9 7\n10 9\n10 11\n11 12\n12 10";

    let g = read_graph_from_string(case4);
    let r = g.kosaraju_sharir_scc().week4_programming_assignment();
    assert_eq!(r, vec![6,3,2,1]);

}

extern crate rand;

use std::collections::btree_map::BTreeMap;
use std::iter::FromIterator;

use self::rand::Rng;


#[derive(Debug, Clone)]
pub struct Graph {
    pub adj: BTreeMap<u32, Vec<u32>>
}

impl Graph {
    pub fn new(vs: Vec<u32>, es: Vec<Vec<u32>>) -> Graph {
        let adj = vs.iter()
            .enumerate()
            .map(|(i, &u)| (u, es[i].clone()))
            .collect();
        Graph {
            adj: adj
        }
    }

    pub fn random_edge(&self) -> (u32, u32) {
        let mut rng = rand::thread_rng();

        let end_vertices: Vec<_> = self.adj.keys().collect();
        let u = rng.choose(&end_vertices).unwrap();
        let v = rng.choose(self.adj.get(u).unwrap()).unwrap();
        (**u, *v)
    }

    pub fn contract(&mut self, u: u32, v: u32) {
        assert!(u != v);
        assert!(self.adj.contains_key(&u) && self.adj.contains_key(&v));

        let v_to = self.adj.remove(&v).expect("v not in Graph");

        self.adj.get_mut(&u)
            .map(|u_to| {
                *u_to = u_to.iter()
                    .cloned()
                    .chain(v_to.into_iter())
                    .filter(|&w| w != u && w != v)
                    .collect();
            });
        self.adj.values_mut()
            .map(|vs|
                 vs.iter_mut()
                 .map(|w| {
                     if *w == v {
                         *w = u;
                     }
                 }).last()
            ).last();
    }

    pub fn vertices(&self) -> usize {
        self.adj.len()
    }

    pub fn edges(&self) -> usize {
        // assume undirected graph
        self.adj.values().map(|vs| vs.len()).sum::<usize>() / 2
    }

    pub fn minimum_cut_karger(&self) -> Graph {
        if self.vertices() < 2 {
            panic!("single vertex!");
        }
        let mut g = self.clone();

        while g.vertices() > 2 {
            let (u, v) = g.random_edge();
            g.contract(u, v);
        }

        g
    }
}



impl FromIterator<(u32, Vec<u32>)> for Graph {
    fn from_iter<I: IntoIterator<Item=(u32, Vec<u32>)>>(iter: I) -> Self {
        Graph {
            adj: iter.into_iter().collect()
        }
    }
}


#[test]
fn test_graph_basic() {
    let g = Graph::new(
        vec![1, 2, 3, 4],
        vec![vec![2,3],
             vec![1, 3, 4],
             vec![1, 2, 4],
             vec![2, 3]]);

    assert_eq!(g.vertices(), 4);
    assert_eq!(g.edges(), 5);
}



#[test]
fn test_graph_contract() {

    let mut g = Graph::new(
        vec![1, 2, 3, 4],
        vec![vec![2,3],
             vec![1, 3, 4],
             vec![1, 2, 4],
             vec![2, 3]]);

    println!("random edge => {:?}", g.random_edge());
    println!("random edge => {:?}", g.random_edge());
    println!("random edge => {:?}", g.random_edge());

    println!("g => {:?}", g);
    g.contract(1, 3);
    println!("contract g => {:?}", g);

    g.contract(1, 2);
    println!("contract g => {:?}", g);


}

#[test]
fn test_min_cut() {
    let g = Graph::new(
        vec![1, 2, 3, 4],
        vec![vec![2,3],
             vec![1, 3, 4],
             vec![1, 2, 4],
             vec![2, 3]]);

    println!("min cut => {:?}", g.minimum_cut_karger());

}

//! Programming Assignment #5

extern crate rustc_serialize;

use std::collections::btree_map::BTreeMap;
pub use self::rustc_serialize::json::Json as Property;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Outgoing,
    Incoming,
    Both,
}


pub struct Graph {
    nodes: Vec<Option<Node>>,
    rels: Vec<Relationship>,
    reuse_nodes: Vec<usize>,
    reuse_rels: Vec<usize>,
}

pub struct Node {
    id: usize,
    name: i32,
    next_rel: Option<usize>
    // TODO: support prop
    // prop: BTreeMap<String, Property>,
    // TODO: label
}

pub struct Relationship {
    id: usize,                  // idx in Graph
    start_node: usize,
    start_prev: Option<usize>,
    start_next: Option<usize>,
    end_node: usize,
    end_prev: Option<usize>,
    end_next: Option<usize>,
    // TODO: relationship type, property
    // prop: BTreeMap<String, Property>,
}


impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            rels: vec![],
            reuse_nodes: vec![],
            reuse_rels: vec![],
        }
    }

    pub fn with_capacity(n: usize) -> Graph {
        Graph {
            nodes: Vec::with_capacity(n),
            rels: Vec::with_capacity(n),
            reuse_nodes: vec![],
            reuse_rels: vec![],
        }
    }

    pub fn create_node(&mut self) -> Node {
        unimplemented!()
    }

    pub fn get_node_by_id(&self, id: usize) -> Node {
        unimplemented!()
    }

    pub fn get_rel_by_id(&self, id: usize) -> Relationship {
        unimplemented!()
    }

    pub fn iter_all_nodes(&self) -> ::std::slice::Iter<Node> {
        unimplemented!()
    }

    pub fn iter_all_rels(&self) -> ::std::slice::Iter<Relationship> {
        unimplemented!()
    }
}


impl Node {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn iter_rels(&self) -> ::std::slice::Iter<Relationship> {
        unimplemented!()
    }

    pub fn has_rel(&self) -> bool {
        self.next_rel.is_some()
    }

    pub fn iter_rels_of_dir(&self, dir: Direction) -> ::std::slice::Iter<Relationship> {
        unimplemented!()
    }

    pub fn create_rel_to(&mut self, other: &Node) -> Relationship {
        unimplemented!()
    }

    pub fn get_degree(&self) -> usize {
        unimplemented!()
    }

    pub fn get_degree_of_dir(&mut self, dir: Direction) -> usize {
        unimplemented!()
    }
}


impl Relationship {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn start_node(&self) -> &Node {
        unimplemented!()
    }

    pub fn end_node(&self) -> &Node {
        unimplemented!()
    }

    pub fn other_node(&self, node: &Node) -> &Node {
        unimplemented!()
    }

    pub fn nodes(&self) -> ::std::slice::Iter<Node> {
        unimplemented!()
    }


}

//Implementation of Trie algorithm from "Efficient Subsequence Search for Databases"
//https://link.springer.com/chapter/10.1007/978-3-642-38562-9_45

use std::collections::{HashMap, HashSet};
use std::ptr;
use arrayvec::ArrayVec;

const CARDINALITY: usize = 3;           //TERMINOLOGY K: number of symbols in alphabet (TODO: should really use a macro to generate from Sigma definition)
const ATTR_MAX: usize = 10;             //TERMINOLOGY M: max length of attributes
const EMPTY_NODE_LINK: NodeLink = None; //Necessary for initializing arrays

type Level = usize;
type Position = usize;
type Id = usize;
type NodeLink = Option<Box<Node>>; //inspired by: https://rust-unofficial.github.io/too-many-lists/second-final.html

#[derive(Clone)]
enum Sigma { //TERMINOLOGY Σ: alphabet of symbols
    A, B, C,
}
impl From<char> for Sigma {
    fn from(c: char) -> Self {
        match c {
            'a' => Self::A,
            'b' => Self::B,
            'c' => Self::C,
            _ => panic!(),
        }
    }
}
impl From<Sigma> for char {
    fn from(s: Sigma) -> Self {
        match s {
            Sigma::A => 'a',
            Sigma::B => 'b',
            Sigma::C => 'c',
        }
    }
}
impl From<Sigma> for usize { //for navigating [T; CARDINALITY] arrays, (TODO: we actually want a macro for making this substitution at compile-time)
    fn from(s: Sigma) -> Self {
        match s {
            Sigma::A => 0,
            Sigma::B => 1,
            Sigma::C => 2,
        }
    }
}
impl Default for Sigma { //necessary for root node
    fn default() -> Self {
        Sigma::A
    }
}

#[derive(Clone)]
struct SigString(ArrayVec<Sigma, ATTR_MAX>);
impl From<&str> for SigString {
    fn from(s: &str) -> Self {
        let mut vec = ArrayVec::new();
        for c in s.chars().take(ATTR_MAX) {
            vec.push(Sigma::from(c));
        }
        SigString(vec)
    }
}
impl From<SigString> for String {
    fn from(SigString(vec): SigString) -> Self {
        let mut string = String::new();
        for s in vec.into_iter() {
            string.push(char::from(s));
        }
        string
    }
}

struct Essd { 
    table: Vec<SigString>, //id = vector index (we never need to delete items from the table, only make them unsearchable through the trie)
    trie: NodeLink,
    //linked_ids: ...
}
impl Essd {
    fn table_length(&self) -> usize { //TERMINOLOGY N: number of tuples in table
        self.table.len()
    }
    fn node(&self, x: Position, y: Level) -> Option<&Node> { //TERMINOLOGY node(x, y): refers to node at position x, level y
        panic!() //awkward to implement and probably unnecessary? (TODO: Maybe implement?)
    }
}
impl Essd {
    fn new() -> Self {
        let root_node = Node::new(
            ptr::null_mut(),
            ptr::null_mut(),
            Sigma::default(), 
            0,
            0,
            ptr::null_mut(),
            ATTR_MAX,
        );
        Essd {
            table: Vec::new(),
            trie: Some(Box::new(root_node)), //root node
        }
    }
    fn insert(&mut self, attribute: SigString) { //No id necessary here since Essd.table: Vec<SigString>
        unimplemented!()
    }
    fn search(&self, SigString(query): SigString) -> Vec<(Id, SigString)> {
        let _query_length = query.len(); //TERMINOLOGY L: length of given query (l <= m)
        let trie = self.trie.as_ref().unwrap();
        let ids = trie.search(&query);
        let mut results = Vec::new();
        for id in ids {
            results.push((id, self.table[id].to_owned()));
        }
        results
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}

struct Node {
    children: [NodeLink; CARDINALITY],
    start_tuple: *mut Id,
    end_tuple: *mut Id,
    label: Sigma,                                   //Sigma::default() for root node                (should never be used at root node)
    first_occour: Vec<[*mut Node; CARDINALITY]>,    //e.g. first_occour[5][usize::from(Sigma::A)]   (find first fresh occourence of A, 6 levels beneath this node)
    last_occour: Vec<[*mut Node; CARDINALITY]>,
    level: Level,                                   //0 for root node
    next: *mut Node,
    position: Position,                             //0 for root and first-inserted node at each level
    parent: *mut Node,
}
impl Node {
    fn new(
        start_tuple: *mut Id, 
        end_tuple: *mut Id,
        label: Sigma,
        level: usize,
        position: usize,
        parent: *mut Node,
        max_level: usize,
        ) -> Self {
        Node {
            children: [EMPTY_NODE_LINK; CARDINALITY],
            first_occour: vec![[ptr::null_mut(); CARDINALITY]; max_level - level],
            last_occour: vec![[ptr::null_mut(); CARDINALITY]; max_level - level],
            next: ptr::null_mut(),
            start_tuple,
            end_tuple,
            label,
            level,
            position,
            parent,
        }
    }
    fn insert(&mut self, id: Id, attribute: &[Sigma]) { //does not support update (i.e. id should not already exist)
        unimplemented!()
    }
    fn search(&self, query: &[Sigma]) -> HashSet<Id> {
        unimplemented!()
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}
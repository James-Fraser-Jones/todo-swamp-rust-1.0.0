//Implementation of Trie algorithm from "Efficient Subsequence Search for Databases"
//https://link.springer.com/chapter/10.1007/978-3-642-38562-9_45

use std::collections::{HashMap, HashSet};
use std::ptr;
use arrayvec::ArrayVec;

const K: usize = 3;         //TERMINOLOGY: number of symbols in alphabet (should really be generated from Sigma definition)
const M: usize = 10;        //TERMINOLOGY: max length of attributes
const NoLink: Link = None;  //Necessary for initializing arrays

type Level = usize;
type Position = usize;
type Id = usize;
type Link = Option<Box<Node>>; //inspired by: https://rust-unofficial.github.io/too-many-lists/second-final.html

#[derive(Clone)]
enum Sigma { //TERMINOLOGY: alphabet
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
impl From<Sigma> for usize { //for navigating [T; K] arrays, we actually want a macro for making this substitution at compile-time
    fn from(s: Sigma) -> Self {
        match s {
            Sigma::A => 0,
            Sigma::B => 1,
            Sigma::C => 2,
        }
    }
}
impl Default for Sigma {
    fn default() -> Self {
        Sigma::A
    }
}

#[derive(Clone)]
struct SigString(ArrayVec<Sigma, M>);
impl From<&str> for SigString {
    fn from(s: &str) -> Self {
        let mut vec = ArrayVec::new();
        for c in s.chars().take(M) {
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
    trie: Link,
    //linked_ids: ...
}
impl Essd {
    fn n(&self) -> usize { //TERMINOLOGY: number of tuples in table
        self.table.len()
    }
    fn node(&self, x: Position, y: Level) -> Option<&Node> { //TERMINOLOGY: refers to node at position x, level y
        panic!() //awkward to implement (and probably unnecessary?)
    }
}
impl Essd {
    fn new() -> Self {
        Essd {
            table: Vec::new(),
            trie: Some(Box::new(Node::new())),
        }
    }
    fn insert(&mut self, id: Id, attribute: SigString) { //does not support update (i.e. id should not already exist)
        unimplemented!()
    }
    fn search(&self, SigString(query): SigString) -> Vec<(Id, SigString)> {
        let _l = query.len(); //TERMINOLOGY: length of given query (l <= m)
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
    children: [Link; K],
    start_tuple: *mut Id,
    end_tuple: *mut Id,
    label: Sigma,                       //Sigma::default() for root node                (should never be used at root node)
    first_occour: [[*mut Node; M]; K],  //e.g. first_occour[usize::from(Sigma::A)][5]   (this wastes a LOT of space since only root node needs all M depths)
    last_occour: [[*mut Node; M]; K],
    level: Level,                       //0 for root node
    next: *mut Node,
    position: Position,                 //0 for root and first-inserted node at each level
    parent: *mut Node,
}
impl Node {
    fn new() -> Self {
        Node {
            children: [NoLink; K],
            start_tuple: ptr::null_mut(),
            end_tuple: ptr::null_mut(),
            label: Sigma::default(),
            first_occour: [[ptr::null_mut(); M]; K],
            last_occour: [[ptr::null_mut(); M]; K],
            level: 0,
            next: ptr::null_mut(),
            position: 0,
            parent: ptr::null_mut(),
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
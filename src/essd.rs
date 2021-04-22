use std::collections::{HashMap, HashSet};
use arrayvec::ArrayVec;
use std::fmt::Display;

type Level = usize;
type Position = usize;
type Id = usize;

struct WrapStr<'a>(&'a str);
struct WrapString(String);

type SigString = ArrayVec<Sigma, m>;
impl From<WrapStr<'_>> for SigString {
    fn from(WrapStr(s): WrapStr) -> Self {
        let mut sig_string = ArrayVec::new();
        for c in s.chars().take(m) {
            sig_string.push(match c {
                'a' => Sigma::A,
                'b' => Sigma::B,
                'c' => Sigma::C,
                _ => panic!(),
            });
        }
        sig_string
    }
}
impl From<SigString> for WrapString {
    fn from(s: SigString) -> Self {
        let mut string = String::new();
        for c in s.into_iter() {
            string.push(match c {
                Sigma::A => 'a',
                Sigma::B => 'b',
                Sigma::C => 'b',
            });
        }
        WrapString(string)
    }
}

#[derive(Clone)]
enum Sigma { //TERMINOLOGY: alphabet
    A, B, C,
}

const k: usize = 3; //TERMINOLOGY: number of characters in alphabet
const m: usize = 10; //TERMINOLOGY: max length of string

//Implementation of Trie algorithm from "Efficient Subsequence Search for Databases"
//https://link.springer.com/chapter/10.1007/978-3-642-38562-9_45
struct Essd { 
    table: HashMap<Id, SigString>,
    trie: Box<Node>, //box to ensure all nodes are heap allocated
}
impl Essd {
    fn n(&self) -> usize { //TERMINOLOGY: number of tuples in table
        self.table.len()
    }
    fn node(&self, x: Position, y: Level) -> Option<&Node> { //TERMINOLOGY: refers to node at position x, level y
        panic!() //awkward to implement (and technically unnecessary)
    }
}
impl Essd {
    fn new() -> Self {
        Essd {
            table: HashMap::new(),
            trie: Box::new(Node::new()),
        }
    }
    fn insert(&mut self, id: Id, attribute: SigString) {
        unimplemented!()
    }
    fn search(&self, query: SigString) -> Vec<(Id, SigString)> {
        let l = query.len(); //TERMINOLOGY: length of given query (l <= m)
        let ids = self.trie.search(query);
        let mut results = Vec::new();
        for id in ids {
            let val: SigString = (*self.table.get(&id).unwrap()).to_owned();
            results.push((id, val))
        }
        results
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}

struct Node {
    children: HashMap<Sigma, Node>,
    start_tuple_id: Id,
    end_tuple_id: Id,
    label: Sigma,
    first_occour: HashMap<Sigma, HashMap<Level, *mut Node>>,
    last_occour: HashMap<Sigma, HashMap<Level, *mut Node>>,
    level: Level,
    next: *mut Node,
    position: Position,
    parent: *mut Node,
}
impl Node {
    fn new() -> Self {
        unimplemented!()
    }
    fn insert(&mut self, id: Id, attribute: SigString) {
        unimplemented!()
    }
    fn search(&self, query: SigString) -> HashSet<Id> {
        unimplemented!()
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}
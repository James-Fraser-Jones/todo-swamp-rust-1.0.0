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
    linked_ids: Vec<(Id, usize, usize)>,   //hacky way of getting linked-list-like functionality (TODO: Possibly remove prev_index if we don't need it)
}
impl Essd {
    fn table_length(&self) -> usize { //TERMINOLOGY N: number of tuples in table
        self.table.len()
    }
    fn node(&self, x: Position, y: Level) -> Option<&Node> { //TERMINOLOGY node(x, y): refers to node at position x, level y
        panic!() //(TODO: implement)
    }
}
impl Essd {
    fn new() -> Self {
        let root_node = Node::new(
            std::usize::MAX,
            std::usize::MAX,
            Sigma::default(), 
            0,
            0,
            ptr::null_mut(),
            ATTR_MAX,
        );
        Essd {
            table: Vec::new(),
            trie: Some(Box::new(root_node)), //root node
            linked_ids: Vec::new(),
        }
    }
    fn insert(&mut self, attribute: SigString) { //No id necessary here since Essd.table: Vec<SigString>
        unimplemented!()
    }
    fn search(&self, SigString(query): SigString) -> Vec<(Id, SigString)> {
        let _query_length = query.len(); //TERMINOLOGY L: length of given query (L <= M)
        let trie = self.trie.as_ref().unwrap();
        let ids = trie.search(&query, &self.linked_ids);
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
    children: [NodeLink; CARDINALITY], //TODO: Implement with pointers instead and have Vec<Vec<Node>> parametrized by level then position

    start_id_index: usize,                          //std::usize::MAX in the case of node containing no ids
    end_id_index: usize,                            //std::usize::MAX in the case of node containing no ids

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
        start_id_index: usize, 
        end_id_index: usize,
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
            start_id_index,
            end_id_index,
            label,
            level,
            position,
            parent,
        }
    }
    fn insert(&mut self, id: Id, attribute: &[Sigma]) { //does not support update (i.e. id should not already exist)
        unimplemented!()
        /*
        push id onto the end of linked-ids, once we reach the end of the "chain" it should be apparent what to set the indices to
        and which other elements' indices to change to ensure correct ordering of "linked" elements

        so go down the trie chain

        upon reaching an empty node, switch to "empty node mode" where we can assume all children beyond this point will also be empty
        in "empty node mode", we not only add specified id but have to modify pointers on the empty node and its ancestor and sibling nodes
        specifically:
            next
                use variable with pointer to immediate parent's last_occour for your symbol to get that node and set its "next" pointer to yourself
            first_occour & last occour
                for all ancestors up to and including first ancestor with same label as the node you added,
                check first_occour for your symbol at relative level +1 for each ancestor above, if it's null, 
                then add pointer to yourself at first_occour and last_occour, otherwise just add pointer at last_occour
            start_id_index & end_id_index
                upon reaching the first empty node, check parent's start_id_index and end_id_index,
                for this range, lookup the associated word in the table and use lexicographical ordering to determine current insert word's position
                use this to determine which indices to update in order to propperly "insert" id into linked-ids
                if word happens to be *first* or *last* in this sublist, then chance parent's (and all further relevant ancestors) start_id_index or end_id_index to 
                reflect this. (ancestors are only relevant when they have the same start_id_index or end_id_index)
                for all further empty nodes, start_id_index and end_id_index should just be set to index of the insert word's id in the linked_ids
        
        upon reaching a non-existent node, switch to "create nodes mode" where we know we will have to create a node for all remaining characters
        so here we: create node, then do the same (as above)
        */
    }
    fn search(&self, query: &[Sigma], ids: &Vec<(Id, usize, usize)>) -> HashSet<Id> {
        let mut tuples = HashSet::new();
        if query.len() == 0 {
            return self.tuples_in_subtree(ids);
        }
        for (level, first) in self.first_occour.iter().enumerate() {
            let first_ptr = first[usize::from(query[0].to_owned())];
            if !first_ptr.is_null() {
                let mut node = first_ptr;
                unsafe {
                    tuples = tuples.union(&(*node).search(&query[1..], ids)).cloned().collect(); //TODO: figure out if this is inefficient
                    while node != self.last_occour[level][usize::from(query[0].to_owned())] {
                        node = (*node).next;
                        tuples = tuples.union(&(*node).search(&query[1..], ids)).cloned().collect();
                    }
                }
            }
        }
        tuples
    }
    fn tuples_in_subtree(&self, ids: &Vec<(Id, usize, usize)>) -> HashSet<Id> {
        let mut tuples = HashSet::new();
        let mut index = self.start_id_index;
        let mut val = ids[index];
        tuples.insert(val.0);
        while index != self.end_id_index {
            index = val.2;
            val = ids[index];
            tuples.insert(val.0);
        }
        tuples
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}
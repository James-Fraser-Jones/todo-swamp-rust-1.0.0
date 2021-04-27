//Implementation of Trie algorithm from "Efficient Subsequence Search for Databases"
//https://link.springer.com/chapter/10.1007/978-3-642-38562-9_45

use std::collections::HashSet;
use arrayvec::ArrayVec;

const CARDINALITY: usize = 3;   //TERMINOLOGY K: number of symbols in alphabet (does not include special 'Root' symbol)
const ATTR_MAX: usize = 10;     //TERMINOLOGY M: max length of attributes

#[derive(PartialEq, Eq, Hash, Clone)]
struct Id(usize);
struct Lvl(usize);
struct Pos(usize);
type PosPtr = Option<Pos>;

const POS_NULL: PosPtr = None;  //Necessary for initializing arrays
const EMPTY_NODES: Vec<Node> = Vec::new();

#[derive(Clone)]
enum Sigma { //TERMINOLOGY Σ: alphabet of symbols
    A, B, C, Root,
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
            _ => panic!(),
        }
    }
}
impl From<Sigma> for usize { //for indexing arrays, (TODO: we actually want a macro for making this substitution at compile-time)
    fn from(s: Sigma) -> Self {
        match s {
            Sigma::A => 0,
            Sigma::B => 1,
            Sigma::C => 2,
            _ => panic!(),
        }
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
    trie: [Vec<Node>; ATTR_MAX+1],
    linked_ids: Vec<(Id, Option<usize>)>, //hacky way of getting linked-list-like functionality
}
impl Essd {
    fn table_length(&self) -> usize { //TERMINOLOGY N: number of tuples in table
        self.table.len()
    }
    fn node(&self, Pos(x): Pos, Lvl(y): Lvl) -> Option<&Node> { //TERMINOLOGY node(x, y): refers to node at position x, level y
        self.trie.get(y).and_then(|vec| vec.get(x))
    }
    fn root(&self) -> &Node {
        self.node(Pos(0), Lvl(0)).unwrap()
    }
}
impl Essd {
    fn new() -> Self {
        let root_node = Node::new(
            Sigma::Root, 
            Lvl(0),
            Pos(0),
            POS_NULL,
        );
        let mut trie = [EMPTY_NODES;ATTR_MAX+1];
        trie[0].push(root_node);
        Essd {
            table: Vec::new(),
            trie, //root node
            linked_ids: Vec::new(),
        }
    }
    fn insert(&mut self, attribute: SigString) { //No id necessary here since Essd.table: Vec<SigString>
        unimplemented!()
    }
    fn search(&self, SigString(query): SigString) -> Vec<(Id, SigString)> {
        let _query_length = query.len(); //TERMINOLOGY L: length of given query (L <= M)
        let root_node = self.root();
        let ids = root_node.search(&query, &self.trie, &self.linked_ids);
        let mut results = Vec::new();
        for id in ids {
            results.push((id, self.table[id.0].to_owned()));
        }
        results
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}

struct Node {
    id_index_range: Option<(usize, usize)>,
    label: Sigma,
    first_occour: Vec<[PosPtr; CARDINALITY]>,   //e.g. first_occour[5][usize::from(Sigma::A)]   (find first fresh occourence of A, 6 levels beneath this node)
    last_occour: Vec<[PosPtr; CARDINALITY]>,    //first element of first_occour and last_occour both give direct children of the node
    level: Lvl,                                 //0 for root node
    next: PosPtr,
    position: Pos,                              //0 for root and first-inserted node at each level
    parent: PosPtr,
}
impl Node {
    fn new(
        label: Sigma,
        level: Lvl,
        position: Pos,
        parent: PosPtr,
        ) -> Self {
        Node {
            first_occour: vec![[POS_NULL; CARDINALITY]; ATTR_MAX-level.0],
            last_occour: vec![[POS_NULL; CARDINALITY]; ATTR_MAX-level.0],
            next: POS_NULL,
            id_index_range: None,
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
                (nope this is wrong, immediate parent won't contain first_occour or last_occour for your symbol which is why your node is empty)
                (we actually need to go up the stack to the first instance of the same symbol as your node and look at its last occour and then add yourself to the next of that instead)
            first_occour & last occour
                for all ancestors up to and including first ancestor with same label as the node you added,
                check first_occour for your symbol at relative level +1 for each ancestor above, if it's null, 
                then add pointer to yourself at first_occour and last_occour, otherwise just add pointer at last_occour
            start_id_index & end_id_index
                upon reaching the first empty node, check parent's start_id_index and end_id_index,
                for this range, lookup the associated word in the table and use lexicographical ordering to determine current insert word's position
                use this to determine which indices to update in order to propperly "insert" id into linked-ids
                if word happens to be *first* or *last* in this sublist, then change parent's (and all further relevant ancestors) start_id_index or end_id_index to 
                reflect this. (ancestors are only relevant when they have the same start_id_index or end_id_index)
                for all further empty nodes, start_id_index and end_id_index should just be set to index of the insert word's id in the linked_ids
        
        upon reaching a non-existent node, switch to "create nodes mode" where we know we will have to create a node for all remaining characters
        so here we: create node, then do the same (as above)
        */

        //TODO: figure out how we can make this recursive
    }
    fn search(&self, query: &[Sigma], trie_chunk: &[Vec<Node>], ids: &Vec<(Id, Option<usize>)>) -> HashSet<Id> {
        let mut tuples = HashSet::new();
        if query.len() == 0 {
            return self.tuples_in_subtree(ids);
        }
        for (relative_level, first_arr) in self.first_occour.iter().enumerate() {
            let first_ptr = first_arr[usize::from(query[0].to_owned())];
            if let Some(Pos(first_pos)) = first_ptr {
                let mut node = &trie_chunk[relative_level][first_pos];
                tuples = tuples.union(&node.search(&query[1..], &trie_chunk[relative_level+1..], ids)).cloned().collect(); //TODO: figure out if this is inefficient
                while node.position.0 != self.last_occour[relative_level][usize::from(query[0].to_owned())].unwrap().0 {
                    node = &trie_chunk[relative_level][node.next.unwrap().0];
                    tuples = tuples.union(&node.search(&query[1..], &trie_chunk[relative_level+1..], ids)).cloned().collect();
                }
            }
        }
        tuples
    }
    fn tuples_in_subtree(&self, ids: &Vec<(Id, Option<usize>)>) -> HashSet<Id> {
        let mut tuples = HashSet::new();
        let (start_idx, end_idx) = self.id_index_range.unwrap();
        let mut index = start_idx;
        let mut val = ids[index];
        tuples.insert(val.0);
        while index != end_idx {
            index = val.1.unwrap();
            val = ids[index];
            tuples.insert(val.0);
        }
        tuples
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}
//Implementation of Trie algorithm from "Efficient Subsequence Search for Databases"
//https://link.springer.com/chapter/10.1007/978-3-642-38562-9_45

//Assertions:
//1: Max trie depth and hence maximum searchable attribute (string) length is static (set by ATTR_MAX)
//2: Each record has only 1 attribute to have a subsequence matched on
//3: Each record has a unique attribute (unsure whether this is really necessary but simplifies reasoning for now)

use std::collections::HashSet;
use arrayvec::ArrayVec;
use std::ptr::NonNull;

const CARDINALITY: usize = 3;   //TERMINOLOGY K: number of symbols in alphabet (does not include special 'Root' symbol)
const ATTR_MAX: usize = 10;     //TERMINOLOGY M: max length of attributes

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Id(usize);
#[derive(PartialEq, Eq, Clone, Copy)]
struct Lvl(usize);
#[derive(PartialEq, Eq, Clone, Copy)]
struct Pos(usize);

const EMPTY_LEVEL: Vec<Node> = Vec::new(); //needed for array initialization

struct Link<T>(T, Option<NonNull<Link<T>>>);
type LinkedList<T> = Vec<Link<T>>; //hacky way of getting linked-list-like functionality

type Trie = [Vec<Node>; ATTR_MAX + 1];

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
    trie: Trie,
    ids: LinkedList<Id>, 
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
        let mut trie = [EMPTY_LEVEL; ATTR_MAX + 1];
        let root_node = Node::new(Sigma::Root, Lvl(0), Pos(0), None);
        trie[0].push(root_node);
        Essd {
            table: Vec::new(),
            trie,
            ids: Vec::new(),
        }
    }
    fn insert(&mut self, attribute: SigString) { //No id necessary here since Essd.table: Vec<SigString>
        unimplemented!()
    }
    fn search(&self, SigString(query): SigString) -> Vec<(Id, SigString)> {
        let query_length = query.len(); //TERMINOLOGY L: length of given query (L <= M)
        if query_length > ATTR_MAX { panic!() }
        let ids = self.root().search(&query);
        ids.into_iter().map(|id| (id, self.table[id.0].to_owned())).collect()
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}

struct Node {
    label: Sigma,

    level: Lvl,
    position: Pos,

    //e.g: self.fresh[5][usize::from(Sigma::A)].unwrap().0 (pointer to first fresh occourence of A, 6 levels beneath this node)
    fresh: Vec<[Option<(NonNull<Node>, NonNull<Node>)>; CARDINALITY]>,
    id_range: Option<(NonNull<Link<Id>>, NonNull<Link<Id>>)>,

    next: Option<NonNull<Node>>,
    parent: Option<NonNull<Node>>,
}
impl Node {
    fn new(
        label: Sigma,
        level: Lvl,
        position: Pos,
        parent: Option<NonNull<Node>>,
        ) -> Self {
        Node {
            label,
            level,
            position,
            fresh: vec![[None; CARDINALITY]; ATTR_MAX-level.0],
            next: None,
            id_range: None,
            parent,
        }
    }
    fn get_child<'a>(&self, symbol: Sigma) -> Option<&Node> {
        self.fresh[0][usize::from(symbol)].map(|(first, _)| unsafe { &*first.as_ptr() })
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

        fn insert(&mut self, id: Id, attribute: &[Sigma])
        Add id to hashset of current node,
        If next node exists (check direct child of first character in attribute), recursively call insert on this node with first character removed from attribute
        If next node doesn't exist, create it and before recursing again do the following:
            Go up parent chain checking fresh of your letter at your level, until you hit a node with the same symbol as you, or you hit the root
            If it's None, then make it Some(you, you)
            If it's Some(first, last) 
                then (the first time you see this) follow the last to that node and make its new "next" you, 
                and make your *next* its previous next,
                then overwrite the "last" pointer with you
                the next time you see this, check if this "last" has the same last as previous and, if so, change it to you as before
                if not, you can stop without going further 
        */

        //TODO: figure out how we can make this recursive
    }
    fn search(&self, query: &[Sigma]) -> HashSet<Id> {
        let mut tuples = HashSet::new();
        if query.len() == 0 {
            return self.tuples_in_subtree();
        }
        for fresh_level in self.fresh.iter() {
            let fresh_level_symbol = fresh_level[usize::from(query[0].to_owned())];
            if let Some((first_ptr, last_ptr)) = fresh_level_symbol {
                let mut node = unsafe { &*first_ptr.as_ptr() };
                let last_node_pos = unsafe { &*last_ptr.as_ptr() }.position;
                tuples = tuples.union(&node.search(&query[1..])).cloned().collect(); //TODO: figure out if this is inefficient
                while node.position != last_node_pos {
                    node = unsafe { &*node.next.unwrap().as_ptr() };
                    tuples = tuples.union(&node.search(&query[1..])).cloned().collect();
                }
            }
        }
        tuples
    }
    fn tuples_in_subtree(&self) -> HashSet<Id> {
        let mut tuples = HashSet::new();
        let (start_ptr, end_ptr) = self.id_range.unwrap(); //this function should never be called when id_index_range is None
        let mut link = unsafe { &*start_ptr.as_ptr() };
        let end_link = unsafe { &*end_ptr.as_ptr() };
        tuples.insert(link.0);
        while link.0 != end_link.0 {
            link = unsafe { &*link.1.unwrap().as_ptr() };
            tuples.insert(link.0);
        }
        tuples
    }
    fn delete(&mut self, id: Id) {
        unimplemented!()
    }
}
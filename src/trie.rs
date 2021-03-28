use std::collections::{HashMap, HashSet};

const CHARS: [char; 27] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','-'];

//recursive, no tree pruning
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trie1 {
    children: HashMap<char, Trie1>,
    ids: HashSet<u64>,
}
impl Trie1 {
    pub fn new() -> Self {
        Trie1{
            children: HashMap::new(),
            ids: HashSet::new(),
        }
    }

    pub fn add(&mut self, id: u64, insert: &str) {
        fn add_rec(trie: &mut Trie1, id: u64, insert: &str) {
            trie.ids.insert(id);
            if insert.len() == 0 {
                return
            }
            let first_char = insert.chars().nth(0).unwrap();
            if let None = trie.children.get_mut(&first_char) {
                trie.children.insert(first_char, Trie1::new());
            }
            let trie = trie.children.get_mut(&first_char).unwrap();
            add_rec(trie, id, &insert[1..]);
        }
        add_rec(self, id, insert)
    }
    
    pub fn search(&self, search: &str) -> HashSet<u64> {
        fn search_rec(trie: &Trie1, search: &str) -> HashSet<u64> {
            if search.len() == 0 {
                return trie.ids.clone()
            }
            let first_char = search.chars().nth(0).unwrap();
            let mut results = HashSet::new();
            if let Some(trie) = trie.children.get(&first_char) {
                results = results.union(&search_rec(trie, &search[1..])).cloned().collect();
            }
            for c in CHARS.iter() {
                if *c != first_char {
                    if let Some(trie) = trie.children.get(c) {
                        results = results.union(&search_rec(trie, &search)).cloned().collect();
                    }
                }
            }
            results
        };
        search_rec(self, search)
    }

    pub fn delete(&mut self, id: u64) {
        fn delete_rec(trie: &mut Trie1, id: u64) {
            trie.ids.remove(&id);
            for trie in trie.children.values_mut() {
                delete_rec(trie, id)
            }
        }
        delete_rec(self, id)
    }
}

// pub struct Trie<'a> {
//     children: HashMap<char, Trie<'a>>,
//     ids: HashSet<u32>,
//     fresh: HashMap<char, HashMap<u32, &'a Trie<'a>>>,
//     next: Option<&'a Trie<'a>>,
// }

// impl<'a> Trie<'a> {
//     pub fn new() -> Self {
//         Trie{
//             children: HashMap::new(),
//             ids: HashSet::new(),
//             fresh: HashMap::new(),
//             next: None,
//         }
//     }
    
//     pub fn search(&self, search: &str) -> HashSet<u32> {
//         fn search_rec(trie: &Trie, search: &str) -> HashSet<u32> {
//             if search.len() == 0 {
//                 return trie.ids.clone()
//             }
//             let first_char = search.chars().nth(0).unwrap();
//             let mut results = HashSet::new();
//             if let Some(map) = trie.fresh.get(&first_char){
//                 for trie in map.values() {
//                     results = results.union(&search_rec(trie, &search[1..])).cloned().collect();
//                     while let Some(trie) = trie.next {
//                         results = results.union(&search_rec(trie, &search[1..])).cloned().collect();
//                     }
//                 }
//             }
//             results
//         };
//         search_rec(self, search)
//     }
// }

// //=========================================DATABASE===============================================
// pub struct Database<'a> { 
//     records: Vec<(usize, String)>,
//     index: Trie<'a>,
//     next_id: usize,
// }

// impl<'a> Database<'a> {
//     fn new() -> Database<'a> {
//         Database {
//             records: vec![],
//             index: Trie {
//                 children: HashMap::new(), //TODO: replace inefficient hashing algorithm
//                 parent: None,
//                 level: 0,
//                 position: 1,
//                 first_occour: HashMap::new(),
//                 last_occour: HashMap::new(),
//                 next: None,
//                 ids: vec![],
//             },
//             next_id: 0,
//         }
//     }

//     fn search(&self, subsequence: &str) -> Vec<(usize, &str)> {
//         //query trie directly, then do lookup into results using retrieved indices instead code below

//         let mut results = vec![];
//         for (index, sequence) in &self.records {
//             if match_subsequence(sequence, subsequence) {
//                 results.push((*index, &sequence[..]));
//             }
//         }
//         results
//     }

//     fn insert(&mut self, new: &str) {
//         //insert into trie

//         self.records.push((self.next_id, new.to_owned()));
//         self.next_id = self.next_id + 1;
//     }

//     fn delete(&mut self, index: usize) {
//         //delete from trie

//         self.records.remove(index);
//     }
// }

// struct Trie<'a> {
//     children: HashMap<char, Trie<'a>>,
//     parent: Option<&'a Trie<'a>>,

//     level: usize,
//     position: usize,

//     first_occour: HashMap<(char, usize), Trie<'a>>,
//     last_occour: HashMap<(char, usize), Trie<'a>>,
//     next: Option<&'a Trie<'a>>,

//     ids: Vec<usize>,
// }

// //=========================================SIMPLE DATABASE===============================================
// pub struct SimpleDatabase { 
//     records: HashMap<usize, String>,
//     index: SimpleTrie,
//     next_id: usize,
// }
// impl SimpleDatabase {
//     fn new() -> SimpleDatabase {
//         SimpleDatabase {
//             records: HashMap::new(),
//             index: SimpleTrie::new(),
//             next_id: 0,
//         }
//     }
//     fn search(&self, subsequence: &str) -> Vec<(usize, &str)> {
//         let mut results = vec![];
        
//         results
//     }
//     fn insert(&mut self, new: &str) {
//         //update trie
//         let mut trie = &mut self.index;
//         trie.ids.push(self.next_id);
//         for b in new.chars() {
//             trie = trie.children.entry(b).or_insert(SimpleTrie::new());
//             trie.ids.push(self.next_id);
//         }
//         //update records
//         self.records.insert(self.next_id, new.to_owned());
//         //increment id counter
//         self.next_id = self.next_id + 1;
//     }
//     fn delete(&mut self, i: usize) {
//         //remove from records
//         let record = self.records.remove(&i).unwrap();
//         //remove from trie
//         let mut chars = record.chars();
//         let mut trie = &mut self.index;
//         trie.ids.remove(trie.ids.iter().position(|x| *x == i).unwrap()); //one-off removal of id from root node
//         let mut old_c = chars.next().unwrap();
//         let mut next_trie = trie.children.entry(old_c).or_insert(SimpleTrie::new());
//         for c in chars {
//             //remove entire hashmap if node only contains id to be removed
//             if next_trie.ids.len() == 1 {
//                 //trie.children.remove(&old_c); //okay for real I'm just going to do the linked list tutorial because this is way too hard
//                 break
//             }

//             //else just remove that id 
//             next_trie.ids.remove(trie.ids.iter().position(|x| *x == i).unwrap());

//             //update vars for next loop
//             trie = next_trie;
//             next_trie = trie.children.entry(c).or_insert(SimpleTrie::new());
//             old_c = c;
//         }
//     }
// }
// struct SimpleTrie {
//     children: HashMap<char, SimpleTrie>,
//     ids: Vec<usize>,
// }
// impl SimpleTrie {
//     fn new() -> SimpleTrie {
//         SimpleTrie {
//             children: HashMap::new(),
//             ids: vec![],
//         }
//     }
// }

// //=========================================NO DATABASE===============================================

// pub struct NoDatabase { 
//     records: Vec<(usize, String)>,
//     next_id: usize,
// }
// impl NoDatabase {
//     fn new() -> NoDatabase {
//         NoDatabase {
//             records: vec![],
//             next_id: 0,
//         }
//     }
//     fn search(&self, subsequence: &str) -> Vec<(usize, &str)> {
//         let mut results = vec![];
//         for (index, sequence) in &self.records {
//             if match_subsequence(sequence, subsequence) {
//                 results.push((*index, &sequence[..]));
//             }
//         }
//         results
//     }
//     fn insert(&mut self, new: &str) {
//         self.records.push((self.next_id, new.to_owned()));
//         self.next_id = self.next_id + 1;
//     }
//     fn delete(&mut self, index: usize) {
//         self.records.remove(index);
//     }
// }

// fn match_subsequence(sequence: &str, subsequence: &str) -> bool {
//     let l = subsequence.len();
//     if l == 0 { //prevent unsafe memory access if subsequence ended up being empty slice 
//         return true //empty string is technically a subsequence of every string
//     }
//     let sub = subsequence.as_bytes();
//     let mut i = 0;
//     for b in sequence.as_bytes() {
//         unsafe { //safe because termination is guaranteed before i gets too large
//             if b == sub.get_unchecked(i) {
//                 i = i + 1;
//                 if i == l {
//                     return true
//                 }
//             }
//         }
//     }
//     false
// }
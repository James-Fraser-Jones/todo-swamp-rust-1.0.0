use std::collections::{HashMap, HashSet};

const CHARS: [char; 27] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','-'];

//non-recursive, no tree pruning
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trie2 {
    children: HashMap<char, Trie2>,
    ids: HashSet<u64>,
}
impl Trie2 {
    pub fn new() -> Self {
        Trie2{
            children: HashMap::new(),
            ids: HashSet::new(),
        }
    }

    pub fn add(&mut self, id: u64, insert: &str) {
        let mut trie = self;
        trie.ids.insert(id);
        for c in insert.chars() {
            trie = trie.children.entry(c).or_insert(Trie2::new());
            trie.ids.insert(id);
        }
    }

    pub fn search(&self, search: &str) -> HashSet<u64> {
        let mut results = HashSet::new();
        let mut tries_to_visit = vec![(self, search)];
        while let Some((trie, search)) = tries_to_visit.pop() {
            if let Some(first_char) = search.chars().nth(0) {
                for c in CHARS.iter() {
                    if let Some(new_trie) = trie.children.get(c) {
                        let new_search = if *c != first_char { &search[1..] } else { search };
                        tries_to_visit.push((new_trie, new_search));
                    }
                }
            }
            else {
                results = results.union(&trie.ids).cloned().collect();
            }
        }
        results
    }

    pub fn delete(&mut self, id: u64) {
        let mut tries_to_visit = vec![self];
        while let Some(trie) = tries_to_visit.pop() {
            if trie.ids.remove(&id) {
                for new_trie in trie.children.values_mut() {
                    tries_to_visit.push(new_trie)
                }
            }
        }
    }
}

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
            if let Some(first_char) = insert.chars().nth(0) {
                let trie = trie.children.entry(first_char).or_insert(Trie1::new());
                add_rec(trie, id, &insert[1..]);
            }
        }
        add_rec(self, id, insert)
    }
    
    pub fn search(&self, search: &str) -> HashSet<u64> {
        fn search_rec(trie: &Trie1, search: &str) -> HashSet<u64> {
            if let Some(first_char) = search.chars().nth(0) {
                let mut results = HashSet::new();
                for c in CHARS.iter() {
                    if let Some(trie) = trie.children.get(c) {
                        let new_search = if *c != first_char { &search[1..] } else { search };
                        results = results.union(&search_rec(trie, new_search)).cloned().collect();
                    }
                }
                results
            }
            else {
                trie.ids.clone()
            }
        };
        search_rec(self, search)
    }

    pub fn delete(&mut self, id: u64) {
        fn delete_rec(trie: &mut Trie1, id: u64) {
            if trie.ids.remove(&id) {
                for trie in trie.children.values_mut() {
                    delete_rec(trie, id)
                }
            }
        }
        delete_rec(self, id)
    }
}
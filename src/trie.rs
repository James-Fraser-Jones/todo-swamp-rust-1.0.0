use std::collections::{HashMap, HashSet};

const CHARS: [char; 27] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','-'];

//non-recursive, search-match pruning
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trie3 {
    children: HashMap<char, Trie3>,
    ids: HashSet<u64>,
}
impl Trie3 {
    pub fn new() -> Self {
        Trie3{
            children: HashMap::new(),
            ids: HashSet::new(),
        }
    }

    pub fn add(&mut self, id: u64, inserts: Vec<&str>) {
        for insert in inserts {
            Self::add_single(self, id, insert)
        }
    }

    fn add_single(&mut self, id: u64, insert: &str) {
        let mut trie = self;
        trie.ids.insert(id);
        for c in insert.chars() {
            trie = trie.children.entry(c).or_insert(Trie3::new());
            trie.ids.insert(id);
        }
    }

    pub fn search(&self, searches: Vec<&str>, filter: Option<&HashSet<u64>>) -> HashSet<u64> {
        let mut searches = searches.iter();
        if let Some(first_search) = searches.next() {
            let mut result = Self::search_single(self, first_search, filter);
            for search in searches { //use results of previous searches to filter ids in subsequent searches
                result = result.intersection(&Self::search_single(self, search, Some(&result))).cloned().collect();
            }
            result
        }
        else {
            HashSet::new()
        }
    }

    fn search_single(&self, search: &str, filter: Option<&HashSet<u64>>) -> HashSet<u64> {
        let mut results = HashSet::new();
        let mut tries_to_visit = vec![(self, search)];
        'trie: while let Some((trie, search)) = tries_to_visit.pop() {
            if let Some(f) = filter { 
                let mut keep_searching = false;
                for id in f { //if this trie contains an index in the filter, keep searching, otherwise skip this branch
                    if trie.ids.contains(id) {
                        keep_searching = true;
                        break
                    }
                }
                if !keep_searching {
                    continue 'trie
                }
            }
            if let Some(first_char) = search.chars().nth(0) {
                for c in CHARS.iter() {
                    if let Some(new_trie) = trie.children.get(c) {
                        let new_search = if *c == first_char { &search[1..] } else { search };
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

    pub fn add(&mut self, id: u64, inserts: Vec<&str>) {
        for insert in inserts {
            Self::add_single(self, id, insert)
        }
    }

    fn add_single(&mut self, id: u64, insert: &str) {
        let mut trie = self;
        trie.ids.insert(id);
        for c in insert.chars() {
            trie = trie.children.entry(c).or_insert(Trie2::new());
            trie.ids.insert(id);
        }
    }

    pub fn search(&self, searches: Vec<&str>) -> HashSet<u64> {
        let mut matches = searches.iter().map(|search| Self::search_single(self, search));
        if let Some(first_match) = matches.next() {
            return matches.fold(first_match, |acc, next_match| acc.intersection(&next_match).cloned().collect())
        }
        HashSet::new()
    }

    fn search_single(&self, search: &str) -> HashSet<u64> {
        let mut results = HashSet::new();
        let mut tries_to_visit = vec![(self, search)];
        while let Some((trie, search)) = tries_to_visit.pop() {
            if let Some(first_char) = search.chars().nth(0) {
                for c in CHARS.iter() {
                    if let Some(new_trie) = trie.children.get(c) {
                        let new_search = if *c == first_char { &search[1..] } else { search };
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

    pub fn add(&mut self, id: u64, inserts: Vec<&str>) {
        fn add_rec(trie: &mut Trie1, id: u64, insert: &str) {
            trie.ids.insert(id);
            if let Some(first_char) = insert.chars().nth(0) {
                let trie = trie.children.entry(first_char).or_insert(Trie1::new());
                add_rec(trie, id, &insert[1..]);
            }
        }
        for insert in inserts {
            add_rec(self, id, insert)
        }
    }
    
    pub fn search(&self, searches: Vec<&str>) -> HashSet<u64> {
        fn search_rec(trie: &Trie1, search: &str) -> HashSet<u64> {
            if let Some(first_char) = search.chars().nth(0) {
                let mut results = HashSet::new();
                for c in CHARS.iter() {
                    if let Some(trie) = trie.children.get(c) {
                        let new_search = if *c == first_char { &search[1..] } else { search };
                        results = results.union(&search_rec(trie, new_search)).cloned().collect();
                    }
                }
                results
            }
            else { //search string is empty, we successfully matched whole string, so return ids for current node
                trie.ids.clone()
            }
        };
        let mut matches = searches.iter().map(|search| search_rec(self, search));
        if let Some(first_match) = matches.next() {
            return matches.fold(first_match, |acc, next_match| acc.intersection(&next_match).cloned().collect())
        }
        HashSet::new()
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
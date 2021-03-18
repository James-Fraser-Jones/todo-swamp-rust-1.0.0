use std::fmt;
use std::cmp::Ordering;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word(String);

impl Word {
    pub fn new(s: &str) -> Word {
        Word(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

//custom display implementation for a Vec of Words
struct Words<'a> {
    arr: &'a Vec<Word>,
}
impl fmt::Display for Words<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::new();
        for word in self.arr {
            display_string.push_str(&word.to_string());
            display_string.push_str(" ");
        }
        display_string.pop();
        write!(f, "{}", display_string)
    }
}

//custom display implementation for a Vec of Tags
struct Tags<'a> {
    arr: &'a Vec<Tag>,
}
impl fmt::Display for Tags<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_string = String::new();
        for tag in self.arr {
            display_string.push_str(&tag.to_string());
            display_string.push_str(" ");
        }
        display_string.pop();
        write!(f, "{}", display_string)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Vec<Word>,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Vec<Word>, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} \"{}\" {}", self.index, Words{arr: &self.description}, Tags{arr: &self.tags})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: vec![],
        }
    }

    pub fn push(&mut self, description: Vec<Word>, tags: Vec<Tag>) -> TodoItem {
        let item = TodoItem::new(self.top_index, description, tags, false);
        let item_c = item.clone();
        self.items.push(item);
        self.top_index = Index::new(self.top_index.value() + 1);
        item_c
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        if let Ok(n) = self.items.binary_search_by_key(&idx, |item| item.index) {
            self.items[n].done = true;
            Some(idx) //TODO: figure out under what circumstances we return None
        }
        else {
            None
        }
    }

    pub fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        let mut results = vec![];
        'item: for item in self.items.iter() { 
            if item.done { //don't search done items
                continue 'item
            }
            'param: for param in &sp.params { 
                match param {
                    SearchWordOrTag::RawWord(sw) => {
                        for Word(w) in &item.description {
                            if match_subsequence(w, sw) {
                                continue 'param //successful match, try next search parameter
                            }
                        }
                        continue 'item //failed to match with any word in description, try next item
                    }
                    SearchWordOrTag::RawTag(st) => {
                        for Tag(t) in &item.tags {
                            if match_subsequence(t, st) {
                                continue 'param //successful match, try next search parameter
                            }
                        }
                        continue 'item //failed to match with any tag, try next item
                    }
                }
            }
            results.push(item); //successfully matched every seach parameter, add to results
        }
        results
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchWordOrTag {
    RawWord (String),
    RawTag (String),
}

fn match_subsequence(sequence: &str, subsequence: &str) -> bool {
    let l = subsequence.len();
    if l == 0 { //prevent unsafe memory access if subsequence ended up being empty slice 
        return true //empty string is technically a subsequence of every string
    }
    let sub = subsequence.as_bytes();
    let mut i = 0;
    for b in sequence.as_bytes() {
        unsafe { //safe because termination is guaranteed before i gets too large
            if b == sub.get_unchecked(i) {
                i = i + 1;
                if i == l {
                    return true
                }
            }
        }
    }
    false
}
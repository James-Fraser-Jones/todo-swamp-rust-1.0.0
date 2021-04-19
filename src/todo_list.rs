use std::fmt;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::mem;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Index(u64);
impl Index {
    pub fn new(i: u64) -> Self {
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
    pub fn new(s: &str) -> Self {
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
    pub fn new(s: &str) -> Self {
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
    pub fn new(index: Index, description: Vec<Word>, tags: Vec<Tag>, done: bool) -> Self {
        TodoItem {
            index,
            description,
            tags,
            done,
        }
    }
}
impl PartialOrd for TodoItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.index.partial_cmp(&other.index)
    }
}
impl Ord for TodoItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}
impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} \"{}\" {}", self.index, Words{arr: &self.description}, Tags{arr: &self.tags})
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchWordOrTag {
    RawWord (String),
    RawTag (String),
}

pub trait TodoLister {
    fn push(&mut self, description: Vec<Word>, tags: Vec<Tag>) -> TodoItem;
    fn done_with_index(&mut self, idx: Index) -> Option<Index>;
    fn search(&self, sp: SearchParams) -> Vec<&TodoItem>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    items: Vec<TodoItem>,
}
impl TodoList {
    pub fn new() -> Self {
        TodoList {
            items: Vec::new(),
        }
    }
    fn match_subsequence(sequence: &str, subsequence: &str) -> bool {
        let mut sub_index = 0;
        let sub_bytes = subsequence.as_bytes(); //this only splits on exact characters when we're using ASCII, not unicode
        for byte in sequence.as_bytes().iter() {
            if sub_index == subsequence.len() {
                return true
            }
            unsafe { //safe because termination is guaranteed before index gets too large
                if byte == sub_bytes.get_unchecked(sub_index) {
                    sub_index += 1;
                }
            }
        }
        sub_index == subsequence.len()
    }
}
impl TodoLister for TodoList {
    fn push(&mut self, description: Vec<Word>, tags: Vec<Tag>) -> TodoItem {
        let item = TodoItem::new(Index::new(self.items.len() as u64), description, tags, false);
        let item_c = item.clone();
        self.items.push(item);
        item_c
    }
    fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        if let Ok(n) = self.items.binary_search_by_key(&idx, |item| item.index) {
            self.items[n].done = true;
            Some(idx)
        }
        else {
            None
        }
    }
    fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        let mut results = Vec::new();
        'item: for item in self.items.iter() { 
            if item.done { //don't search done items
                continue 'item
            }
            'param: for param in &sp.params { 
                match param {
                    SearchWordOrTag::RawWord(sw) => {
                        for Word(w) in &item.description {
                            if Self::match_subsequence(w, sw) {
                                continue 'param //successful match, try next search parameter
                            }
                        }
                        continue 'item //failed to match with any word in description, try next item
                    }
                    SearchWordOrTag::RawTag(st) => {
                        for Tag(t) in &item.tags {
                            if Self::match_subsequence(t, st) {
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

//with previous match filtering
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList2 {
    items: Vec<TodoItem>,
    item_refs: RefCell<Option<Vec<usize>>>,
}
impl TodoList2 {
    pub fn new() -> Self {
        TodoList2 {
            items: Vec::new(),
            item_refs: RefCell::new(Some(Vec::new())),
        }
    }
    fn search_initial<'a>(&'a self, item_refs: &mut Vec<&'a TodoItem>, search: SearchWordOrTag) {
        match search {
            SearchWordOrTag::RawWord(subsequence) => {
                for item in &self.items {
                    for Word(sequence) in &item.description {
                        if Self::match_subsequence(&sequence, &subsequence) {
                            item_refs.push(item);
                            break
                        }
                    }
                }
            },
            SearchWordOrTag::RawTag(subsequence) => {
                for item in &self.items {
                    for Tag(sequence) in &item.tags {
                        if Self::match_subsequence(&sequence, &subsequence) {
                            item_refs.push(item);
                            break
                        }
                    }
                }
            },
        }
    }
    fn search_filter(self: &TodoList2, refs: &mut Vec<&TodoItem>, search: SearchWordOrTag) {
        match search {
            SearchWordOrTag::RawWord(subsequence) => {
                refs.retain(|item| {
                    for Word(sequence) in &item.description {
                        if Self::match_subsequence(&sequence, &subsequence) {
                            return true
                        }
                    }
                    false
                })
            },
            SearchWordOrTag::RawTag(subsequence) => {
                refs.retain(|item| {
                    for Tag(sequence) in &item.tags {
                        if Self::match_subsequence(&sequence, &subsequence) {
                            return true
                        }
                    }
                    false
                })
            },
        }
    }
    fn match_subsequence(sequence: &str, subsequence: &str) -> bool {
        let mut sub_index = 0;
        let sub_bytes = subsequence.as_bytes(); //this only splits on exact characters when we're using ASCII, not unicode
        for byte in sequence.as_bytes().iter() {
            if sub_index == subsequence.len() {
                return true
            }
            // if sequence.len() - seq_index < subsequence.len() - sub_index { //length checking added here (seems to make it slightly slower overall)
            //     return false
            // }
            unsafe { //safe because termination is guaranteed before index gets too large
                if byte == sub_bytes.get_unchecked(sub_index) {
                    sub_index += 1;
                }
            }
        }
        sub_index == subsequence.len()
    }
}
impl<'a> TodoLister for TodoList2 {
    fn push(&mut self, description: Vec<Word>, tags: Vec<Tag>) -> TodoItem {
        let item = TodoItem::new(Index::new(self.items.len() as u64), description, tags, false);
        let item_c = item.clone();
        self.items.push(item);
        item_c
    }
    fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        if let Ok(n) = self.items.binary_search_by_key(&idx, |item| item.index) {
            self.items[n].done = true;
            Some(idx)
        }
        else {
            None
        }
    }
    fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        //get item_refs
        let entry: &mut Option<Vec<usize>> = &mut self.item_refs.borrow_mut();
        let item_refs: Vec<usize> = mem::take(entry).unwrap();
        let mut item_refs: Vec<&TodoItem> = item_refs.into_iter().filter_map(|_| None).collect(); //should not cause a realloc

        //add and filter references
        let mut params = sp.params.into_iter();
        if let Some(first_param) = params.next() {
            self.search_initial(&mut item_refs, first_param);
            for param in params {
                self.search_filter(&mut item_refs, param);
            }
        }

        //save results
        let results = item_refs.to_owned();

        //put item_refs back
        let item_refs: Vec<usize> = item_refs.into_iter().filter_map(|_| None).collect();
        *entry = Some(item_refs);

        //return results
        results
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriedoList<T: Trie + Default> {
    items: Vec<TodoItem>,
    words: T,
    tags: T,
}
impl<T: Trie + Default> TriedoList<T> {
    pub fn new() -> Self {
        TriedoList {
            items: Vec::new(),
            words: T::default(),
            tags: T::default(),
        }
    }
}
impl<T: Trie + Default> TodoLister for TriedoList<T> {
    fn push(&mut self, description: Vec<Word>, tags: Vec<Tag>) -> TodoItem {
        self.words.add(self.items.len() as u64, description.iter().map(|Word(s)| &s[..]).collect());
        self.tags.add(self.items.len() as u64, tags.iter().map(|Tag(t)| &t[..]).collect());
        let item = TodoItem::new(Index::new(self.items.len() as u64), description, tags, false);
        let item_c = item.clone();
        self.items.push(item);
        item_c
    }
    fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        self.words.delete(idx.value());
        if let Ok(n) = self.items.binary_search_by_key(&idx, |item| item.index) {
            self.items[n].done = true;
            Some(idx)
        }
        else {
            None
        }
    }
    fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {
        let mut word_searches = Vec::new();
        let mut tag_searches = Vec::new();
        for param in &sp.params {
            match param {
                SearchWordOrTag::RawWord(w) => {
                    word_searches.push(&w[..]);
                },
                SearchWordOrTag::RawTag(t) => {
                    tag_searches.push(&t[..]);
                },
            }
        }
        let indices;
        if word_searches.len() > 0 && tag_searches.len() > 0 {
            let word_indices = self.words.search(word_searches, None);
            indices = word_indices.intersection(&self.tags.search(tag_searches, Some(&word_indices))).cloned().collect();
        }
        else if word_searches.len() > 0 {
            indices = self.words.search(word_searches, None);
        }
        else if tag_searches.len() > 0 {
            indices = self.tags.search(tag_searches, None);
        }
        else {
            return Vec::new() 
        }
        indices.iter().map(|index| &self.items[*index as usize]).collect()
    }
}
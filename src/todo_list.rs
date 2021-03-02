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
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
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

pub struct Tags<'a> {
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
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: Index, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
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
        write!(f, "{} \"{}\" {}", self.index, self.description, Tags{arr: &self.tags})
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

    pub fn push(&mut self, description: Description, tags: Vec<Tag>) -> TodoItem {
        let item = TodoItem::new(self.top_index, description, tags, false);
        let item_c = item.clone();
        self.items.push(item);
        self.top_index = Index::new(self.top_index.value() + 1);
        item_c
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        if let Ok(n) = self.items.binary_search_by_key(&idx, |item| item.index) { //TODO: check whether this moves ownership by accident
            self.items[n].done = true;
        }
        Some(idx) //TODO: figure out under what circumstances we return None
    }

    pub fn search(&self, sp: SearchParams) -> Vec<&TodoItem> {

        fn substring_match(string: &String, substring: &String) -> bool{ //TODO: check whether this function actually works
            string.split(substring).count() > 1
        }

        let mut results = vec![];

        for item in self.items.iter() {
            let mut include_item = true;

            if item.done { //don't include search for done items
                include_item = false;
            }

            if !include_item {continue} //TODO: figure out most efficient way to do this kind of short-circuit in rest of this loop

            for SearchWord(w) in &sp.words {
                //search description
                let Description(w2) = &item.description;
                if !substring_match(w2, w){
                    include_item = false;
                }
            }

            println!("Search tags: {:?}, item tags: {:?}", &sp.tags, &item.tags);

            for Tag(t) in &sp.tags {
                //search all tags
                for Tag(t2) in &item.tags {
                    if !substring_match(t2, t){
                        println!("Attempting to find substring: {}, inside string: {}", t, t2);
                        include_item = false;
                    }
                }
            }

            if include_item {
                results.push(item);
            }
        }

        results
    }
}

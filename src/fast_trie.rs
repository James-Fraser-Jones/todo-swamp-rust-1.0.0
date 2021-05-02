use std::collections::HashSet;
use std::iter;
use std::slice;
use std::str;

const CARDINALITY: usize = 27;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Id(u64);

#[derive(PartialEq, Eq, Clone, Copy)]
struct Index(usize);

struct FastIterMutParent<'a> {
    level_iter: iter::Rev<iter::Take<slice::IterMut<'a, Vec<Node>>>>,
    position: Option<Index>,
}
impl<'a> FastIterMutParent<'a> {
    fn new(fast: &'a mut Fast, level: Index, position: Index) -> Self {
        FastIterMutParent {
            level_iter: fast.0.iter_mut().take(level.0 + 1).rev(),
            position: Some(position),
        }
    }
}
impl<'a> Iterator for FastIterMutParent<'a> {
    type Item = &'a mut Node;
    fn next(&mut self) -> Option<Self::Item> {
        let level = self.level_iter.next()?;
        let node = &mut level[self.position?.0];
        self.position = node.parent;
        Some(node)
    }
}

struct FastIterMutChild<'a, 'b> {
    level_iter: iter::Take<slice::IterMut<'a, Vec<Node>>>,
    position: Option<Index>,
    char_iter: str::Chars<'b>,
}
impl<'a, 'b> FastIterMutChild<'a, 'b> {
    fn new(fast: &'a mut Fast, word: &'b str) -> Self {
        FastIterMutChild {
            level_iter: fast.0.iter_mut().take(word.len() + 1),
            position: Some(Index(0)),
            char_iter: word.chars(),
        }
    }
}
impl<'a, 'b> Iterator for FastIterMutChild<'a, 'b> {
    type Item = &'a mut Node;
    fn next(&mut self) -> Option<Self::Item> {
        let level = self.level_iter.next()?;
        let node = &mut level[self.position?.0];
        self.position = node.child(self.char_iter.next()?);
        Some(node)
    }
}

struct Fast(Vec<Vec<Node>>);

impl Fast {
    pub fn new() -> Self {
        Fast(vec![vec![Node::root()]])
    }
    pub fn insert(&mut self, id: Id, attributes: &[&str]) {
        for attribute in attributes {
            self.insert_single(id, attribute)
        }
    }
    pub fn search(&self, attributes: &[&str]) -> HashSet<Id> {
        let mut results = HashSet::new();
        for attribute in attributes { //TODO: Eventually include previous match filtering here
            results = results.intersection(&self.search_single(attribute)).cloned().collect();
        }
        results
    }
}

impl Fast {
    fn iter_mut_parent(&mut self, level: Index, position: Index) -> FastIterMutParent<'_> {
        FastIterMutParent::new(self, level, position)
    }
    fn iter_mut_child<'a, 'b>(&'a mut self, word: &'b str) -> FastIterMutChild<'a, 'b> {
        FastIterMutChild::new(self, word)
    }
}

impl Fast {
    fn insert_single(&mut self, id: Id, attribute: &str) {
        //ensure we have enough levels to insert into
        let diff = attribute.len() - (self.0.len() - 1);
        self.0.append(&mut iter::repeat(Vec::new()).take(diff).collect());

        self.insert_at_something(id, attribute)
    }
    pub fn delete(&mut self, id: Id) {
        Self::delete_at_position(self.0.iter_mut(), vec!(Index(0)), id)
    }
    fn search_single(&self, attribute: &str) -> HashSet<Id> {
        self.search_at_something(attribute)
    }
}
impl Fast {
    fn insert_at_something(&mut self, id: Id, attribute: &str) {
        unimplemented!()
    }

    fn search_at_something(&self, attribute: &str) -> HashSet<Id> {
        unimplemented!()
    }

    fn delete_at_position(mut levels: slice::IterMut<Vec<Node>>, mut positions: Vec<Index>, id: Id) {
        while let Some(level) = levels.next() {
            if positions.is_empty() {
                break
            }
            let mut next_positions = Vec::new();
            for position in positions {
                let node = &mut level[position.0];
                if node.ids.remove(&id) {
                    next_positions.append(&mut node.children());
                }
            }
            positions = next_positions;
        }
    }
}

#[derive(Clone)]
struct Node {
    ids: HashSet<Id>,
    label: char,
    fresh: Vec<[Option<(Index, Index)>; CARDINALITY]>,
    next: Option<Index>,
    parent: Option<Index>,
}
impl Node {
    fn root() -> Self {
        Node {
            ids: HashSet::new(),
            label: '*',
            fresh: Vec::new(),
            next: None,
            parent: None,
        }
    }
    fn new(id: Id, label: char, parent: Index) -> Self {
        Node {
            ids: [id].iter().cloned().collect(),
            label,
            fresh: Vec::new(),
            next: None,
            parent: Some(parent),
        }
    }
    fn children(&self) -> Vec<Index> {
        self.fresh
            .get(0).unwrap_or(&[None; CARDINALITY])
            .iter().filter_map(|e| e.map(|(f,_)| f)).collect()
    }
    fn child(&self, label: char) -> Option<Index> {
        self.fresh
            .get(0)
            .and_then(|arr| arr[char_to_index(label)].map(|(f,_)| f))
    }
}

fn char_to_index(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 'a' as usize,
        '-' => CARDINALITY,
        _ => panic!(),
    }
}

// #[derive(PartialEq, Eq, Clone, Copy)]
// struct Cursor {
//     level: Index, 
//     position: Index,
// }
// impl Cursor {
//     fn new(level: Index, position: Index) -> Self {
//         Cursor {level, position}
//     }
//     fn root() -> Self {
//         Cursor::new(Index(0), Index(0))
//     }
// }

// impl Fast {
//     fn node(&self, cursor: Cursor) -> &Node {
//         //it shouldn't be possible to get a cursor from one of the functions below which doesn't point to a valid node
//         self.0.get(cursor.level.0).unwrap().get(cursor.position.0).unwrap()
//     }
//     fn node_mut(&mut self, cursor: Cursor) -> &mut Node {
//         //it shouldn't be possible to get a cursor from one of the functions below which doesn't point to a valid node
//         self.0.get_mut(cursor.level.0).unwrap().get_mut(cursor.position.0).unwrap()
//     }
//     fn fresh(&self, cursor: Cursor, node: &Node, label: char, rel_level: Index, first: bool) -> Option<Cursor> {
//         let arr = node.fresh.get(cursor.level.0)?;
//         let (first_index, last_index) = arr[char_to_index(label)]?;
//         let fresh_cursor = Cursor::new(Index(cursor.level.0 + rel_level.0 + 1), if first {first_index} else {last_index});
//         Some(fresh_cursor)
//     }
//     fn child(&self, cursor: Cursor, node: &Node, label: char) -> Option<Cursor> {
//         self.fresh(cursor, node, label, Index(0), true)
//     }
//     fn next(&self, cursor: Cursor, node: &Node) -> Option<Cursor> {
//         let next_position = node.next?;
//         let next_cursor = Cursor::new(cursor.level, next_position);
//         Some(next_cursor)
//     }
//     fn parent(&self, cursor: Cursor, node: &Node) -> Option<Cursor> {
//         let parent_position = node.parent?;
//         let parent_cursor = Cursor::new(Index(cursor.level.0-1), parent_position);
//         Some(parent_cursor)
//     }
// }

// fn delete_at_position_recursive(chunk: &mut [Vec<Node>], position: Index, id: Id) {
//     if chunk.len() == 0 {
//         return
//     }
//     let (level, next_chunk) = chunk.split_at_mut(1);
//     let node = &mut level[0][position.0];
//     if node.ids.remove(&id) {
//         for child in node.children() {
//             Self::delete_at_position_recursive(next_chunk, child, id);
//         }
//     }
// }
// fn delete_at_position_slice(mut todo: Vec<(&mut [Vec<Node>], Vec<Index>)>, id: Id) {
//     while let Some((chunk, positions)) = todo.pop() {
//         if chunk.len() == 0 || positions.len() == 0 {
//             continue
//         }
//         let (first_chunk, next_chunk) = chunk.split_at_mut(1);
//         let level = &mut first_chunk[0];
//         let mut next_positions = Vec::new();
//         for position in positions {
//             let node = &mut level[position.0];
//             if node.ids.remove(&id) {
//                 next_positions.append(&mut node.children());
//             }
//         }
//         todo.push((next_chunk, next_positions));
//     }
// }

/*
fn insert_at_cursor(&mut self, cursor: Cursor, id: Id, attribute: &str) {
    Add id to hashset of current node,
    If child node exists (check direct child of first character in attribute), recursively call insert on this node with first character removed from attribute
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

/*

fn insert_at_cursor(&mut self, cursor: Cursor, id: Id, attribute: &str) {
    Step 1:
        Add id to hashset of current node,
        go to next node down etc...
        when you encounter a missing node, take note of your depth, then add all remaining needed nodes to all further levels (up to length of inserted string)
    Step 2:
        Start most recently inserted node at lowest depth (length of string)
        Keep going up updatating relevent indices
        Stop when you hit root node, or when you run out of things you need to update (because you hit nodes with the same symbol and not in update depth range)
        If you hit a node with the same symbol and in the update depth range, update it as usual, but then it replaces your current one for that symbol and you update with that one instead

    If child node exists (check direct child of first character in attribute), recursively call insert on this node with first character removed from attribute
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
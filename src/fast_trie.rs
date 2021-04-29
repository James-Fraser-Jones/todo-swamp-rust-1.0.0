use std::collections::HashSet;

const CARDINALITY: usize = 27;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Id(u64);

#[derive(PartialEq, Eq, Clone, Copy)]
struct Index(usize);

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    level: Index, 
    position: Index,
}
impl Node {
    fn new(level: Index, position: Index) -> Self {
        Node {level, position}
    }
    fn root() -> Self {
        Node::new(Index(0), Index(0))
    }
}

struct Fast(Vec<Vec<Element>>);
impl Fast {
    pub fn new() -> Self {
        Fast(vec![vec![Element::root()]])
    }
    pub fn insert(&mut self, id: Id, attributes: &[&str]) {
        for attribute in attributes {
            self.insert_single(id, attribute)
        }
    }
    pub fn search(&self, attributes: &[&str]) -> HashSet<Id> {
        let mut results = HashSet::new();
        for attribute in attributes {
            results = results.intersection(&self.search_single(attribute)).cloned().collect();
        }
        results
    }
}
impl Fast {
    fn insert_single(&mut self, id: Id, attribute: &str) {
        self.insert_at_node(Node::root(), id, attribute)
    }
    fn search_single(&self, attribute: &str) -> HashSet<Id> {
        self.search_at_node(Node::root(), attribute)
    }
    pub fn delete(&mut self, id: Id) {
        self.delete_at_node(Node::root(), id)
    }
}
impl Fast {
    fn insert_at_node(&mut self, node: Node, id: Id, attribute: &str) {
        unimplemented!()
    }
    fn search_at_node(&self, node: Node, attribute: &str) -> HashSet<Id> {
        unimplemented!()
    }
    fn delete_at_node(&mut self, node: Node, id: Id) {
        unimplemented!()
    }
}
impl Fast {
    fn element(&self, node: Node) -> Option<&Element> {
        self.0.get(node.level.0).and_then(|vec| vec.get(node.position.0))
    }
    fn element_mut(&mut self, node: Node) -> Option<&mut Element> {
        self.0.get_mut(node.level.0).and_then(|vec| vec.get_mut(node.position.0))
    }

    fn fresh(&self, node: Node, element: &Element, label: char, rel_level: Index, first: bool) -> Option<Node> {
        let arr = element.fresh.get(node.level.0)?;
        let (first_index, last_index) = arr[Self::char_to_index(label)]?;
        let fresh_node = Node::new(Index(node.level.0 + rel_level.0 + 1), if first {first_index} else {last_index});
        Some(fresh_node)
    }
    fn child(&self, node: Node, element: &Element, label: char) -> Option<Node> {
        self.fresh(node, element, label, Index(0), true)
    }
    fn next(&self, node: Node, element: &Element) -> Option<Node> {
        let next_position = element.next?;
        let next_node = Node::new(node.level, next_position);
        Some(next_node)
    }
    fn parent(&self, node: Node, element: &Element,) -> Option<Node> {
        let parent_position = element.parent?;
        let parent_node = Node::new(Index(node.level.0-1), parent_position);
        Some(parent_node)
    }

    fn char_to_index(c: char) -> usize {
        match c {
            'a'..='z' => c as usize - 'a' as usize,
            '-' => CARDINALITY,
            _ => panic!(),
        }
    }
}

struct Element {
    ids: HashSet<Id>,
    label: char,
    fresh: Vec<[Option<(Index, Index)>; CARDINALITY]>,
    next: Option<Index>,
    parent: Option<Index>,
}
impl Element {
    fn root() -> Self {
        Element {
            ids: HashSet::new(),
            label: '*',
            fresh: Vec::new(),
            next: None,
            parent: None,
        }
    }
    fn new(id: Id, label: char, level: Index, parent: Index) -> Self {
        Element {
            ids: [id].iter().cloned().collect(),
            label,
            fresh: Vec::new(),
            next: None,
            parent: Some(parent),
        }
    }
}
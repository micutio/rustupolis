extern crate indextree_ng;
use indextree_ng::{Arena, NodeId};

use tuple::{Tuple, E};

pub enum Node<T> {
    Root,
    Path(E),
    Leaf(Option<T>),
}

pub struct Tree<T> {
    arena: Arena<Node<T>>,
    root_id: NodeId,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        let mut arena = Arena::new();
        let root_id = arena.new_node(Node::Root);
        Tree {
            arena: arena,
            root_id: root_id,
        }
    }

    pub fn insert(&mut self, tup: Tuple, item: T) -> NodeId {
        let id = self.root_id.clone();
        self.do_insert(id, tup, item)
    }

    fn do_insert(&mut self, id: NodeId, tup: Tuple, item: T) -> NodeId {
        if tup.is_empty() {
            return self.arena.new_node(Node::Leaf(Some(item)));
        }
        let next = id.children(&self.arena)
            .filter(|child_id| match self.arena[*child_id].data {
                Node::Path(ref e) => e == tup.first(),
                _ => false,
            })
            .next();
        match next {
            Some(id) => self.do_insert(id, tup.rest(), item),
            None => {
                let new_node = self.arena.new_node(Node::Path(tup.first().clone()));
                self.do_insert(new_node, tup.rest(), item)
            }
        }
    }

    pub fn take(&mut self, tup: Tuple) -> Option<T> {
        let id = self.root_id.clone();
        self.do_take(id, tup)
    }

    fn do_take(&mut self, id: NodeId, tup: Tuple) -> Option<T> {
        if tup.is_empty() {
            let child_id = match id.children(&self.arena)
                .filter(|child_id| match self.arena[*child_id].data {
                    Node::Leaf(_) => true,
                    _ => false,
                })
                .next()
            {
                Some(child_id) => child_id,
                None => return None,
            };
            let result = match self.arena[child_id].data {
                Node::Leaf(ref mut item) => item.take(),
                _ => return None,
            };
            self.arena.remove_node(child_id);
            return result;
        }
        let children = id.children(&self.arena)
            .filter(|child_id| match self.arena[*child_id].data {
                Node::Path(ref e) => e.matches(tup.first()),
                _ => false,
            })
            .collect::<Vec<NodeId>>();
        for child_id in children {
            if let Some(item) = self.do_take(child_id, tup.rest()) {
                return Some(item);
            }
        }
        None
    }
}

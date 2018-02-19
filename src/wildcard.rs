extern crate indextree_ng;
use indextree_ng::{Arena, NodeId};

use tuple::{Tuple, E};
use error::{Error, ResultExt};

pub enum Node<T> {
    Root,
    Path(E),
    Leaf(Option<T>),
}

/// A wildcard tree that can insert and take values associated with a pending wildcard. Used by
/// Space for coordination.
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

    pub fn insert(&mut self, tup: Tuple, item: T) -> Result<(), Error> {
        debug!("insert {:?}", tup);
        let id = self.root_id.clone();
        self.do_insert(id, tup, item)
    }

    fn do_insert(&mut self, id: NodeId, tup: Tuple, item: T) -> Result<(), Error> {
        trace!("do_insert {:?} {:?}", id, tup);
        if tup.is_empty() {
            let child_id = self.arena.new_node(Node::Leaf(Some(item)));
            id.append(child_id, &mut self.arena)
                .chain_err(|| "insert failed")?;
            trace!("do_insert appending {:?} child of {:?}", child_id, id);
            return Ok(());
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
                let child_id = self.arena.new_node(Node::Path(tup.first().clone()));
                id.append(child_id, &mut self.arena)
                    .chain_err(|| "insert failed")?;
                trace!("do_insert appending {:?} child of {:?}", child_id, id);
                self.do_insert(child_id, tup.rest(), item)
            }
        }
    }

    pub fn take(&mut self, tup: Tuple) -> Option<T> {
        debug!("take {:?}", tup);
        let id = self.root_id.clone();
        self.do_take(id, tup)
    }

    fn do_take(&mut self, id: NodeId, tup: Tuple) -> Option<T> {
        trace!("take {:?} {:?}", id, tup);
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
            trace!("take matched, removing node and returning");
            child_id.detach(&mut self.arena);
            self.arena.remove_node(child_id);
            return result;
        }
        let children = id.children(&self.arena)
            .filter(|child_id| {
                let node = &self.arena[*child_id].data;
                trace!("take: child {:?}", child_id);
                match node {
                    &Node::Path(ref e) => e.matches(tup.first()),
                    _ => false,
                }
            })
            .collect::<Vec<NodeId>>();
        trace!("take: potential matches: {:?}", children);
        for child_id in children {
            if let Some(item) = self.do_take(child_id, tup.rest()) {
                return Some(item);
            }
        }
        None
    }
}

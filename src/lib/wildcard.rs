extern crate indextree_ng;
use indextree_ng::{Arena, NodeId};

use crate::error::{Error, ResultExt};
use crate::tuple::{Tuple, E};

pub enum Node<T> {
    Root,
    Path(E),
    Leaf(Option<T>),
}

/// An arena-based wildcard tree that can insert and take values
/// associated with a pending wildcard.
/// Used by Space for coordination.
pub struct Tree<T> {
    arena:   Arena<Node<T>>,
    root_id: NodeId,
}

impl<T> Default for Tree<T> {
    fn default() -> Tree<T> {
        Tree::<T>::new()
    }
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        let mut arena = Arena::new();
        let root_id = arena.new_node(Node::Root);
        Tree { arena, root_id }
    }

    /// Public interface for inserting an item into the wildcard tree,
    /// along a tuple 'path'.
    /// # Errors
    /// `IndexTreeError` if inserting into the wildcard tree fails
    pub fn insert(&mut self, tup: Tuple, item: T) -> Result<(), Error> {
        debug!("insert {:?}", tup);
        let id = self.root_id;
        self.do_insert(id, tup, item)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn do_insert(&mut self, id: NodeId, tup: Tuple, item: T) -> Result<(), Error> {
        trace!("do_insert {:?} {:?}", id, tup);
        // If we have an empty tuple, insert the item in a new leaf node of NodeId.
        if tup.is_empty() {
            let child_id = self.arena.new_node(Node::Leaf(Some(item)));
            // TODO: More expressive error description
            id.append(child_id, &mut self.arena)
                .chain_err(|| "insert failed")?;
            trace!("do_insert appending {:?} child of {:?}", child_id, id);
            return Ok(());
        }
        // If the tuple is not empty, look for a child node whose id matches
        // the first element of the tuple. If we can't find it, we create a new child node.
        let next = id
            .children(&self.arena)
            .find(|child_id| match self.arena[*child_id].data {
                Node::Path(ref e) => e == tup.first(),
                _ => false,
            });
        // Finally continue inserting with the rest of the tuple.
        if let Some(id) = next {
            self.do_insert(id, tup.rest(), item)
        } else {
            let child_id = self.arena.new_node(Node::Path(tup.first().clone()));
            id.append(child_id, &mut self.arena)
                .chain_err(|| "insert failed")?;
            trace!("do_insert appending {:?} child of {:?}", child_id, id);
            self.do_insert(child_id, tup.rest(), item)
        }
    }

    /// Public interface for retrieving an item out of the wildcard tree,
    /// from a tuple 'path'.
    pub fn take(&mut self, tup: Tuple) -> Option<T> {
        debug!("take {:?}", tup);
        let id = self.root_id;
        self.do_take(id, tup)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn do_take(&mut self, id: NodeId, tup: Tuple) -> Option<T> {
        trace!("take {:?} {:?}", id, tup);
        if tup.is_empty() {
            let Some(child_id) = id
                .children(&self.arena)
                .find(|child_id| matches!(self.arena[*child_id].data, Node::Leaf(_))) else { return None };
            let result = match self.arena[child_id].data {
                Node::Leaf(ref mut item) => item.take(),
                _ => return None,
            };
            trace!("take matched, removing node and returning");
            child_id.detach(&mut self.arena);
            self.arena.remove_node(child_id);
            return result;
        }
        let children = id
            .children(&self.arena)
            .filter(|child_id| {
                let node = &self.arena[*child_id].data;
                trace!("take: child {:?}", child_id);
                match node {
                    Node::Path(ref e) => e.matches(tup.first()),
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

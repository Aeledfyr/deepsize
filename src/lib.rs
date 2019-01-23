#![forbid(missing_docs)]

//! A utility for recursively measuring the size of an object
//!
//! This contains the [`DeepSizeOf`](DeepSizeOf) trait, and re-exports
//! the `DeepSizeOf` derive macro from [`deepsize_derive`](https://docs.rs/deepsize_derive)
//!
//! ```rust
//! use deepsize::DeepSizeOf;
//!
//! #[derive(DeepSizeOf)]
//! struct Test {
//!     a: u32,
//!     b: Box<u8>,
//! }
//!
//! fn main() {
//!     let object = Test {
//!         a: 15,
//!         b: Box::new(255),
//!     };
//!
//!     assert_eq!(object.deep_size_of(), 17);
//! }
//! ```
//!


pub use deepsize_derive::*;

use std::collections::HashSet;

mod default_impls;
#[cfg(test)]
mod test;


/// A trait for measuring the size of an object and its children
///
/// In many cases this is just `std::mem::size_of::<T>()`, but if
/// the struct contains a `Vec`, `String`, `Box`, or other allocated object or
/// reference, then it is the size of the struct, plus the size of the contents
/// of the object.
pub trait DeepSizeOf {
    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    fn deep_size_of(&self) -> usize {
        self.recurse_deep_size_of(&mut Context::new())
    }
    
    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    ///
    /// This is an internal function, and requires a [`Context`](Context)
    fn recurse_deep_size_of(&self, context: &mut Context) -> usize {
        self.stack_size() + self.deep_size_of_children(context)
    }

    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    fn deep_size_of_children(&self, context: &mut Context) -> usize;

    /// Returns the size of the memory the object uses itself
    ///
    /// This method is generally equivalent to [`size_of_val`](std::mem::size_of_val)
    fn stack_size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}


/// The context of which references have already been seen
///
/// Keeps track of the [`Arc`](std::sync::Arc)s, [`Rc`](std::rc::Rc)s, and references
/// that have been visited, so that [`Arc`](std::sync::Arc)s and other references
/// aren't double counted.
///
/// Currently this counts each reference once, although there are arguments for
/// only counting owned data, and ignoring partial ownership, or for counting
/// partial refernces like Arc as its size divided by the strong reference count.
/// [Github Issue discussion here](https://github.com/dtolnay/request-for-implementation/issues/22)
#[derive(Debug)]
pub struct Context {
    /// A set of all [`Arcs`](std::sync::Arc) that have already been counted
    arcs: HashSet<usize>,
    /// A set of all [`Rcs`](std::sync::Arc) that have already been counted
    rcs: HashSet<usize>,
    /// A set of all normal references that have already been counted
    refs: HashSet<usize>,
}

impl Context {
    /// Creates a new empty context for use in the deep_size functions
    pub fn new() -> Context {
        Context {
            arcs: HashSet::new(),
            rcs:  HashSet::new(),
            refs: HashSet::new(),
        }
    }
    
    /// Adds an [`Arc`](std::sync::Arc) to the list of visited [`Arc`](std::sync::Arc)s
    fn add_arc<T>(&mut self, arc: &std::sync::Arc<T>) {
        // Somewhat unsafe way of getting a pointer to the inner `ArcInner`
        // object without changing the count
        let pointer: usize = *unsafe { std::mem::transmute::<&std::sync::Arc<T>, &usize>(arc) };
        self.arcs.insert(pointer);
    }
    /// Checks if an [`Arc`](std::sync::Arc) is in the list visited [`Arc`](std::sync::Arc)s
    fn contains_arc<T>(&self, arc: &std::sync::Arc<T>) -> bool {
        let pointer: usize = *unsafe { std::mem::transmute::<&std::sync::Arc<T>, &usize>(arc) };
        self.arcs.contains(&pointer)
    }

    /// Adds an [`Rc`](std::rc::Rc) to the list of visited [`Rc`](std::rc::Rc)s
    fn add_rc<T>(&mut self, rc: &std::rc::Rc<T>) {
        // Somewhat unsafe way of getting a pointer to the inner `RcBox`
        // object without changing the count
        let pointer: usize = *unsafe { std::mem::transmute::<&std::rc::Rc<T>, &usize>(rc) };
        self.rcs.insert(pointer);
    }
    /// Checks if an [`Rc`](std::rc::Rc) is in the list visited [`Rc`](std::rc::Rc)s
    fn contains_rc<T>(&self, rc: &std::rc::Rc<T>) -> bool {
        let pointer: usize = *unsafe { std::mem::transmute::<&std::rc::Rc<T>, &usize>(rc) };
        self.rcs.contains(&pointer)
    }

    /// Adds a [`reference`](std::reference) to the list of visited [`reference`](std::reference)s
    fn add_ref<T>(&mut self, reference: &T) {
        let pointer: usize = reference as *const _ as usize;
        self.refs.insert(pointer);
    }
    /// Checks if a [`reference`](std::reference) is in the list of visited [`reference`](std::reference)s
    fn contains_ref<T>(&self, reference: &T) -> bool {
        let pointer: usize = reference as *const _ as usize;
        self.refs.contains(&pointer)
    }
}

impl<T> DeepSizeOf for std::vec::Vec<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, child| sum + child.recurse_deep_size_of(context))
         + (self.capacity() - self.len()) * std::mem::size_of::<T>()
        // Size of unused capacity
    }
}

impl<T> DeepSizeOf for std::collections::VecDeque<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, child| sum + child.recurse_deep_size_of(context))
         + (self.capacity() - self.len()) * std::mem::size_of::<T>()
        // Size of unused capacity
    }
}

impl<T> DeepSizeOf for std::collections::LinkedList<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter().fold(0, |sum, child| {
            sum + child.recurse_deep_size_of(context)
             + std::mem::size_of::<usize>() * 2 // overhead of each node
        })
    }
}

impl<K, V, S> DeepSizeOf for std::collections::HashMap<K, V, S>
where
    K: DeepSizeOf + Eq + std::hash::Hash, V: DeepSizeOf, S: std::hash::BuildHasher
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, (key, val)| {
                sum + key.recurse_deep_size_of(context)
                    + val.recurse_deep_size_of(context)
            })
         + (self.capacity() - self.len()) * (std::mem::size_of::<K>() + std::mem::size_of::<V>())
        // Size of unused capacity
    }
}

impl<T, S> DeepSizeOf for std::collections::HashSet<T, S>
where
    T: DeepSizeOf + Eq + std::hash::Hash, S: std::hash::BuildHasher
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, item| {
                sum + item.recurse_deep_size_of(context)
            })
         + (self.capacity() - self.len()) * std::mem::size_of::<T>()
        // Size of unused capacity
    }
}

impl<T> DeepSizeOf for std::boxed::Box<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        // May cause inacuracies, measures size of the value, but not the allocation size
        let val: &T = &*self;
        val.recurse_deep_size_of(context)
    }
}

impl<T> DeepSizeOf for std::sync::Arc<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        let val: &T = &*self;
        val.recurse_deep_size_of(context)
    }

    fn recurse_deep_size_of(&self, context: &mut Context) -> usize {
        if context.contains_arc(self) {
            self.stack_size()
        } else {
            context.add_arc(self);
            self.stack_size() + self.deep_size_of_children(context)
        }
    }
}

impl<T> DeepSizeOf for std::rc::Rc<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        let val: &T = &*self;
        val.recurse_deep_size_of(context)
    }

    fn recurse_deep_size_of(&self, context: &mut Context) -> usize {
        if context.contains_rc(self) {
            self.stack_size()
        } else {
            context.add_rc(self);
            self.stack_size() + self.deep_size_of_children(context)
        }
    }
}

impl<T: ?Sized> DeepSizeOf for &T
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        if context.contains_ref(&self) {
            0
        } else {
            context.add_ref(&self);
            (*self).recurse_deep_size_of(context)
        }
    }

    fn recurse_deep_size_of(&self, context: &mut Context) -> usize {
        if context.contains_ref(&self) {
            self.stack_size()
        } else {
            context.add_ref(&self);
            self.stack_size() + self.deep_size_of_children(context)
        }
    }
}

impl<T> DeepSizeOf for [T]
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, child| sum + child.recurse_deep_size_of(context))
    }

    fn stack_size(&self) -> usize {
        0
    }
}

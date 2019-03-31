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
//!     // The stack size of the struct:
//!     //  - The size of a u32 (4)
//!     //  - 4 bytes padding (64 bit only)
//!     //  - The stack size of the Box (a usize pointer, 32 or 64 bits: 4 or 8 bytes)
//!     // + the size of a u8 (1), the Box's heap storage
//!     #[cfg(target_pointer_width = "64")]
//!     assert_eq!(object.deep_size_of(), 17);
//!     #[cfg(target_pointer_width = "32")]
//!     assert_eq!(object.deep_size_of(), 9);
//! }
//! ```
//!

// Hack so that #[derive(DeepSizeOf)] is usable within the crate
// until [this](https://github.com/rust-lang/rust/pull/57407) stabalizes
// Also means that both crates need to be on the 2015 edition
mod deepsize { pub use super::*; }
extern crate deepsize_derive;
pub use deepsize_derive::*;

use std::collections::HashSet;
use std::mem::{size_of, size_of_val};

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
    ///
    /// ```rust
    /// use deepsize::DeepSizeOf;
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<Box<u32>, Vec<String>> = HashMap::new();
    ///
    /// map.insert(Box::new(42u32), vec![String::from("Hello World")]);
    /// map.insert(Box::new(20u32), vec![String::from("Something")]);
    /// map.insert(Box::new(0u32),  vec![String::from("A string")]);
    /// map.insert(Box::new(255u32), vec![String::from("Dynamically Allocated!")]);
    ///
    /// assert_eq!(map.deep_size_of(), 1312);
    /// ```
    fn deep_size_of(&self) -> usize {
        size_of_val(self) + self.deep_size_of_children(&mut Context::new())
    }
    
    /// Deprecated: equivalent to `std::mem::size_of_val(val) + val.deep_size_of_children()`
    #[deprecated(since="0.1.1", note="use `std::mem::size_of_val(val) + val.deep_size_of_children()` instead")]
    fn recurse_deep_size_of(&self, context: &mut Context) -> usize {
        size_of_val(self) + self.deep_size_of_children(context)
    }

    /// Returns an estimation of the heap-managed storage of this object.
    /// This does not include the size of the object itself.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    ///
    /// This is an internal function, and requires a [`Context`](Context)
    /// to track visited references.  Implementations of this function
    /// should only call `deep_size_of_children`, and not `deep_size_of`
    /// so that they reference tracking is not reset.
    ///
    /// If a struct and all of its children do not allocate or have references,
    /// this method should return `0`, as it cannot have any heap allocated
    /// children.  There is a shortcut macro for this implementation, [`known_size_of`](known_size_of),
    /// used like `known_deep_size!(0, (), u32, u64);` which generates the impls.
    ///
    /// The most common way to use this method, and how the derive works,
    /// is to call this method on each of the structs members and sum the
    /// results, which works as long as all members of the struct implmeent
    /// DeepSizeOf.
    ///
    /// To implement this for a collection type, you should sum the deep sizes of
    /// the items of the collection and then add the size of the allocation of the
    /// collection itself.  This can become much more complicated if the collection
    /// has multiple seperate allocations.
    ///
    /// Here is an example from the implementation of `DeepSizeOf` for `Vec<T>`
    /// ```rust, ignore
    /// # use deepsize::{DeepSizeOf, Context};
    /// impl<T> DeepSizeOf for std::vec::Vec<T> where T: DeepSizeOf {
    ///     fn deep_size_of_children(&self, context: &mut Context) -> usize {
    ///         // Size of heap allocations for each child
    ///         self.iter().map(|child| child.deep_size_of_children(context)).sum()
    ///          + self.capacity() * std::mem::size_of::<T>()  // Size Vec's heap allocation
    ///     }
    /// }
    /// ```
    fn deep_size_of_children(&self, context: &mut Context) -> usize;
}


/// The context of which references have already been seen.
/// This should only be used in the implementation of the
/// `deep_size_of_children` function, and (eventually, when this
/// reaches 0.2) will not be able to be constructed, and only obtained
/// from inside the function.
///
/// Keeps track of the [`Arc`](std::sync::Arc)s, [`Rc`](std::rc::Rc)s, and references
/// that have been visited, so that [`Arc`](std::sync::Arc)s and other references
/// aren't double counted.
///
/// Currently this counts each reference once, although there are arguments for
/// only counting owned data and ignoring partial ownership, or for counting
/// partial refernces like Arc as its size divided by the strong reference count.
///
/// [Github Issue discussion here](https://github.com/dtolnay/request-for-implementation/issues/22)
///
/// This must be passed between `deep_size_of_children` calls when
/// recursing, so that references are not double-counted.
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
    /// Adds an [`Rc`](std::rc::Rc) to the list of visited [`Rc`](std::rc::Rc)s
    fn contains_rc<T>(&self, rc: &std::rc::Rc<T>) -> bool {
        let pointer: usize = *unsafe { std::mem::transmute::<&std::rc::Rc<T>, &usize>(rc) };
        self.rcs.contains(&pointer)
    }

    /// Adds a [`reference`](std::reference) to the list of visited [`reference`](std::reference)s
    /// Adds an [`Rc`](std::rc::Rc) to the list of visited [`Rc`](std::rc::Rc)s
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
    /// Sums the size of each child object, and then adds the size of
    /// the unused capacity.
    ///
    /// ```rust
    /// use deepsize::DeepSizeOf;
    ///
    /// let mut vec: Vec<u8> = vec![];
    /// for i in 0..13 {
    ///     vec.push(i);
    /// }
    ///
    /// // The capacity (16) plus three usizes (len, cap, pointer)
    /// assert_eq!(vec.deep_size_of(), 16 + 24);
    /// ```
    /// With allocated objects:
    /// ```rust
    /// use deepsize::DeepSizeOf;
    ///
    /// let mut vec: Vec<Box<u64>> = vec![];
    /// for i in 0..13 {
    ///     vec.push(Box::new(i));
    /// }
    ///
    /// // The capacity (16?) * size (8) plus three usizes (len, cap, pointer)
    /// // and length (13) * the allocated size of each object
    /// assert_eq!(vec.deep_size_of(), 24 + vec.capacity() * 8 + 13 * 8);
    /// ```
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, child| sum + child.deep_size_of_children(context))
         + self.capacity() * size_of::<T>()
        // Size of unused capacity
    }
}

impl<T> DeepSizeOf for std::collections::VecDeque<T>
where
    T: DeepSizeOf,
{
    /// Sums the size of each child object, and then adds the size of
    /// the unused capacity.
    ///
    /// ```rust
    /// use deepsize::DeepSizeOf;
    /// use std::collections::VecDeque;
    ///
    /// let mut vec: VecDeque<u8> = VecDeque::new();
    /// for i in 0..12 {
    ///     vec.push_back(i);
    /// }
    /// vec.push_front(13);
    ///
    /// // The capacity (15?) plus four usizes (start, end, cap, pointer)
    /// assert_eq!(vec.deep_size_of(), vec.capacity() * 1 + 32);
    /// ```
    /// With allocated objects:
    /// ```rust
    /// use deepsize::DeepSizeOf;
    /// use std::collections::VecDeque;
    ///
    /// let mut vec: VecDeque<Box<u64>> = VecDeque::new();
    /// for i in 0..12 {
    ///     vec.push_back(Box::new(i));
    /// }
    /// vec.push_front(Box::new(13));
    ///
    /// // The capacity (15?) * size (8) plus four usizes (start, end, cap, pointer)
    /// // and length (13) * the allocated size of each object
    /// assert_eq!(vec.deep_size_of(), 32 + vec.capacity() * 8 + 13 * 8);
    /// ```
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        // Deep size of children
        self.iter().map(|child| child.deep_size_of_children(context)).sum::<usize>()
         + self.capacity() * size_of::<T>()  // Size of Vec's heap allocation
    }
}

impl<T> DeepSizeOf for std::collections::LinkedList<T>
where
    T: DeepSizeOf,
{
    /// Sums the size of each child object, assuming the overhead of
    /// each node is 2 usize (next, prev)
    ///
    /// ```rust
    /// use deepsize::DeepSizeOf;
    /// use std::collections::LinkedList;
    ///
    /// let mut list: LinkedList<u8> = LinkedList::new();
    /// for i in 0..12 {
    ///     list.push_back(i);
    /// }
    /// list.push_front(13);
    ///
    /// assert_eq!(list.deep_size_of(), std::mem::size_of::<LinkedList<u8>>()
    ///                                + 13 * 1 + 13 * 2 * 8);
    /// ```
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter().fold(0, |sum, child| {
            sum + size_of_val(child) + child.deep_size_of_children(context)
             + size_of::<usize>() * 2 // overhead of each node
        })
    }
}

impl<K, V, S> DeepSizeOf for std::collections::HashMap<K, V, S>
where
    K: DeepSizeOf + Eq + std::hash::Hash, V: DeepSizeOf, S: std::hash::BuildHasher
{
    // FIXME
    // How much more overhead is there to a hashmap? The docs say it is
    // essensially just a Vec<Option<(u64, K, V)>>
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, (key, val)| {
                sum + key.deep_size_of_children(context)
                    + val.deep_size_of_children(context)
            })
         + self.capacity() * size_of::<Option<(u64, K, V)>>()
        // Size of container capacity
    }
}

impl<T, S> DeepSizeOf for std::collections::HashSet<T, S>
where
    T: DeepSizeOf + Eq + std::hash::Hash, S: std::hash::BuildHasher
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter()
            .fold(0, |sum, item| {
                sum + item.deep_size_of_children(context)
            })
         + self.capacity() * size_of::<Option<(u64, T, ())>>()
        // Size container storage
    }
}

impl<T> DeepSizeOf for std::boxed::Box<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        // May cause inacuracies, measures size of the value, but not the allocation size
        let val: &T = &*self;
        size_of_val(val) + val.deep_size_of_children(context)
    }
}

impl<T> DeepSizeOf for std::sync::Arc<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        if context.contains_arc(self) {
            0
        } else {
            context.add_arc(self);
            let val: &T = &*self;
            // Size of the Arc, size of the value, size of the allocations of the value
            size_of_val(val) + val.deep_size_of_children(context)
        }
    }
}

impl<T> DeepSizeOf for std::rc::Rc<T>
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        if context.contains_rc(self) {
            0
        } else {
            context.add_rc(self);
            let val: &T = &*self;
            size_of_val(val) + val.deep_size_of_children(context)
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
            size_of_val(*self) + (*self).deep_size_of_children(context)
        }
    }
}

impl<T> DeepSizeOf for [T]
where
    T: DeepSizeOf,
{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.iter().map(|child| child.deep_size_of_children(context)).sum()
    }
}

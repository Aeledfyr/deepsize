pub use deepsize_derive::*;

mod default_impls;
#[cfg(test)]
mod test;

pub trait DeepSizeOf {
    fn deep_size_of(&self) -> usize {
        self.recurse_deep_size_of(&mut Context::new())
    }

    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    fn recurse_deep_size_of(&self, context: &mut Context) -> usize {
        self.stack_size() + self.deep_size_of_children(context)
    }

    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    fn deep_size_of_children(&self, context: &mut Context) -> usize;

    /// Returns the size of the memory the object uses on the stack,
    /// assuming that it is on
    ///
    /// This method is directly equivalent to [`size_of_val`](std::mem::size_of_val)
    fn stack_size(&self) -> usize {
        std::mem::size_of_val(self)
    }
}

use std::collections::HashSet;

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
    fn new() -> Context {
        Context {
            arcs: HashSet::new(),
            rcs:  HashSet::new(),
            refs: HashSet::new(),
        }
    }

    fn add_arc<T>(&mut self, arc: &std::sync::Arc<T>) {
        // Somewhat unsafe way of getting a pointer to the inner `ArcInner`
        // object without changing the count
        let pointer: usize = *unsafe { std::mem::transmute::<&std::sync::Arc<T>, &usize>(arc) };
        self.arcs.insert(pointer);
    }
    fn contains_arc<T>(&self, arc: &std::sync::Arc<T>) -> bool {
        let pointer: usize = *unsafe { std::mem::transmute::<&std::sync::Arc<T>, &usize>(arc) };
        self.arcs.contains(&pointer)
    }

    fn add_rc<T>(&mut self, rc: &std::rc::Rc<T>) {
        // Somewhat unsafe way of getting a pointer to the inner `RcBox`
        // object without changing the count
        let pointer: usize = *unsafe { std::mem::transmute::<&std::rc::Rc<T>, &usize>(rc) };
        self.rcs.insert(pointer);
    }
    fn contains_rc<T>(&self, rc: &std::rc::Rc<T>) -> bool {
        let pointer: usize = *unsafe { std::mem::transmute::<&std::rc::Rc<T>, &usize>(rc) };
        self.rcs.contains(&pointer)
    }

    fn add_ref<T>(&mut self, reference: &T) {
        let pointer: usize = reference as *const _ as usize;
        self.refs.insert(pointer);
    }
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


#[cfg(test)]
mod test;


pub trait DeepSizeOf {
    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    ///
    /// This method is directly equivalent to [`size_of`](std::mem::size_of) + [`deep_size_of_children`](DeepSizeOf::deep_size_of_children)
    fn deep_size_of(&self) -> usize where Self: Sized {
        Self::stack_size() + self.deep_size_of_children()
    }
    
    /// Returns an estimation of a total size of memory owned by the
    /// object, including heap-managed storage.
    ///
    /// This is an estimation and not a precise result, because it
    /// doesn't account for allocator's overhead.
    fn deep_size_of_children(&self) -> usize;
    
    /// Returns the size of the memory the object uses on the stack,
    /// assuming that it is on
    ///
    /// This method is directly equivalent to [`size_of`](std::mem::size_of)
    fn stack_size() -> usize where Self: Sized {
        std::mem::size_of::<Self>()
    }
}


impl<T> DeepSizeOf for std::vec::Vec<T> where T: DeepSizeOf {
    fn deep_size_of_children(&self) -> usize {
        self.iter().fold(0, |sum, child| sum + child.deep_size_of())
    }
}

impl<T> DeepSizeOf for std::boxed::Box<T> where T: DeepSizeOf {
    fn deep_size_of_children(&self) -> usize {
        // May cause inacuracies, measures size of the value, but not the allocation size
        let val: &T = &*self;
        std::mem::size_of_val(val)
    }
}


macro_rules! non_recursive {
    ($type:ty) => {
        impl DeepSizeOf for $type {
            fn deep_size_of_children(&self) -> usize { 0 }
        }
    }
}

non_recursive!(u8);
non_recursive!(u16);
non_recursive!(u32);
non_recursive!(u64);
non_recursive!(u128);

non_recursive!(i8);
non_recursive!(i16);
non_recursive!(i32);
non_recursive!(i64);
non_recursive!(i128);

non_recursive!(f32);
non_recursive!(f64);

non_recursive!(());


impl<T: ?Sized> DeepSizeOf for std::marker::PhantomData<T> {
    fn deep_size_of_children(&self) -> usize { 0 }
}

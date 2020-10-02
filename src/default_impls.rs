use crate::{Context, DeepSizeOf};

/// A macro to generate an impl for types with known inner allocation sizes.
///
/// Repurposed from the `heapsize` crate
///
/// Usage:
/// ```rust
/// # #[macro_use] extern crate deepsize; fn main() {
/// struct A(u32);
/// struct B(A, char);
/// struct C(Box<u32>);
///
/// known_deep_size!(0, A, B); // A and B do not have any allocation
/// known_deep_size!(4, C); // C will always have an allocation of 4 bytes
/// # }
/// ```
#[macro_export]
macro_rules! known_deep_size(
    ($size:expr, $($type:ty),+) => (
        $(
            impl $crate::DeepSizeOf for $type {
                #[inline(always)]
                fn deep_size_of_children(&self, _: &mut $crate::Context) -> usize {
                    $size
                }
            }
        )+
    );
    ($size:expr, $($type:ident<$($gen:ident),+>),+) => (
        $(
            impl<$($gen: $crate::HeapSizeOf),+> $crate::DeepSizeOf for $type<$($gen),+> {
                #[inline(always)]
                fn deep_size_of_children(&self, _: &mut $crate::Context) -> usize {
                    $size
                }
            }
        )+
    );
);

known_deep_size!(0, bool, char, str);
known_deep_size!(0, u8, u16, u32, u64, usize);
known_deep_size!(0, i8, i16, i32, i64, isize);
known_deep_size!(0, f32, f64);
known_deep_size!(0, ());
known_deep_size!(
    0,
    core::sync::atomic::AtomicBool,
    core::sync::atomic::AtomicIsize,
    core::sync::atomic::AtomicUsize
);

impl<T: ?Sized> DeepSizeOf for core::marker::PhantomData<T> {
    fn deep_size_of_children(&self, _: &mut Context) -> usize {
        0
    }
}

impl DeepSizeOf for alloc::string::String {
    fn deep_size_of_children(&self, _: &mut Context) -> usize {
        // Size of the allocation of the string
        self.capacity()
    }
}

impl<T: DeepSizeOf> DeepSizeOf for core::option::Option<T> {
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        match &self {
            Some(t) => t.deep_size_of_children(context),
            None => 0,
        }
    }
}

impl<R: DeepSizeOf, E: DeepSizeOf> DeepSizeOf for core::result::Result<R, E> {
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        match &self {
            Ok(r) => r.deep_size_of_children(context),
            Err(e) => e.deep_size_of_children(context),
        }
    }
}

macro_rules! deep_size_array {
    ($num:expr) => {
        impl<T: DeepSizeOf> DeepSizeOf for [T; $num] {
            fn deep_size_of_children(&self, context: &mut Context) -> usize {
                self.as_ref().deep_size_of_children(context)
            }
        }
    };
}

// Can't wait for const generics
// A year and a half later, still waiting
deep_size_array!(1);
deep_size_array!(2);
deep_size_array!(3);
deep_size_array!(4);
deep_size_array!(5);
deep_size_array!(6);
deep_size_array!(7);
deep_size_array!(8);
deep_size_array!(9);
deep_size_array!(10);
deep_size_array!(11);
deep_size_array!(12);
deep_size_array!(13);
deep_size_array!(14);
deep_size_array!(15);
deep_size_array!(16);
deep_size_array!(17);
deep_size_array!(18);
deep_size_array!(19);
deep_size_array!(20);
deep_size_array!(21);
deep_size_array!(22);
deep_size_array!(23);
deep_size_array!(24);
deep_size_array!(25);
deep_size_array!(26);
deep_size_array!(27);
deep_size_array!(28);
deep_size_array!(29);
deep_size_array!(30);
deep_size_array!(31);
deep_size_array!(32);

macro_rules! deep_size_tuple {
    ($(($n:tt, $T:ident)),+ ) => {
        impl<$($T,)+> DeepSizeOf for ($($T,)+)
            where $($T: DeepSizeOf,)+
        {
            fn deep_size_of_children(&self, context: &mut Context) -> usize {
                0 $( + self.$n.deep_size_of_children(context))+
            }
        }
    };
}

deep_size_tuple!((0, A));
deep_size_tuple!((0, A), (1, B));
deep_size_tuple!((0, A), (1, B), (2, C));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D), (4, E));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
deep_size_tuple!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));

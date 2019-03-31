use crate::{Context, DeepSizeOf};

/// For use on types defined in external crates
/// with known heap sizes.
///
/// Repurposed from the `heapsize` crate
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
    std::sync::atomic::AtomicBool,
    std::sync::atomic::AtomicIsize,
    std::sync::atomic::AtomicUsize
);

impl<T: ?Sized> DeepSizeOf for std::marker::PhantomData<T> {
    fn deep_size_of_children(&self, _: &mut Context) -> usize {
        0
    }
}

impl DeepSizeOf for String {
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        self.as_str().deep_size_of_children(context)
    }
}

impl<T: DeepSizeOf> DeepSizeOf for Option<T> {
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        match &self {
            Some(t) => t.deep_size_of_children(context),
            None => 0,
        }
    }
}

impl<R: DeepSizeOf, E: DeepSizeOf> DeepSizeOf for Result<R, E> {
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        match &self {
            Ok(r)  => r.deep_size_of_children(context),
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
    }
}

// Can't wait for const generics
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

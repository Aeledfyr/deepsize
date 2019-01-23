use crate::{Context, DeepSizeOf};

/// For use on types defined in external crates
/// with known heap sizes.
///
/// Stolen from `heapsize` crate
#[macro_export]
macro_rules! known_deep_size(
    ($size:expr, $($type:ty),+) => (
        $(
            impl $crate::DeepSizeOf for $type {
                #[inline(always)]
                fn deep_size_of_children(&self, _: &mut Context) -> usize {
                    $size
                }
            }
        )+
    );
    ($size:expr, $($type:ident<$($gen:ident),+>),+) => (
        $(
            impl<$($gen: $crate::HeapSizeOf),+> $crate::DeepSizeOf for $type<$($gen),+> {
                #[inline(always)]
                fn deep_size_of_children(&self, _: &mut Context) -> usize {
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

use crate::{Context, DeepSizeOf};

#[cfg(features = "slotmap")]
mod slotmap_impl {
    use super::*;
    
    known_deep_size!(0, slotmap::KeyData, slotmap::DefaultKey);
    
    impl<K, V> DeepSizeOf for slotmap::SlotMap<K, V>
    where
        K: DeepSizeOf + slotmap::Key, V: DeepSizeOf + slotmap::Slottable,
    {
        fn deep_size_of_children(&self, context: &mut Context) -> usize {
            self.iter()
                .fold(0, |sum, (key, val)| {
                    sum + key.deep_size_of_children(context)
                        + val.deep_size_of_children(context)
                })
            + self.capacity() * size_of::<(u32, V)>>()
        }
    }
}
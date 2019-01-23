
use crate::DeepSizeOf;

#[test]
fn primitive_types() {
    assert_eq!(0u8.deep_size_of(),  1);
    assert_eq!(0u16.deep_size_of(), 2);
    assert_eq!(0u32.deep_size_of(), 4);
    assert_eq!(0u64.deep_size_of(), 8);
    
    assert_eq!(0i8.deep_size_of(),  1);
    assert_eq!(0i16.deep_size_of(), 2);
    assert_eq!(0i32.deep_size_of(), 4);
    assert_eq!(0i64.deep_size_of(), 8);
    
    assert_eq!(0f32.deep_size_of(), 4);
    assert_eq!(0f64.deep_size_of(), 8);
}

#[test]
fn boxed_integers() {
    let boxed = Box::new(0u32);
    assert_eq!(boxed.deep_size_of(), 4 + 8);
}



use crate::DeepSizeOf;

#[test]
fn primitive_types() {
    assert_eq!(0u8.deep_size_of(),  1);
    assert_eq!(0u16.deep_size_of(), 2);
    assert_eq!(0u32.deep_size_of(), 4);
    assert_eq!(0u64.deep_size_of(), 8);
    assert_eq!(0usize.deep_size_of(), std::mem::size_of::<usize>());
    
    assert_eq!(0i8.deep_size_of(),  1);
    assert_eq!(0i16.deep_size_of(), 2);
    assert_eq!(0i32.deep_size_of(), 4);
    assert_eq!(0i64.deep_size_of(), 8);
    assert_eq!(0isize.deep_size_of(), std::mem::size_of::<isize>());
    
    assert_eq!(0f32.deep_size_of(), 4);
    assert_eq!(0f64.deep_size_of(), 8);
    
    assert_eq!('f'.deep_size_of(), 4);
    assert_eq!("Hello World!".deep_size_of(), 12);
    assert_eq!(true.deep_size_of(), 1);
}

#[test]
fn boxes() {
    let boxed = Box::new(0u32);
    assert_eq!(boxed.deep_size_of(), 4 + 8);
}

#[test]
fn slices() {
    let array: [u32; 64] = [0; 64];
    assert_eq!(array[5..10].deep_size_of(), 4 * 5);
    assert_eq!(array[..32].deep_size_of(), 4 * 32);
    assert_eq!(DeepSizeOf::deep_size_of(&&array[..8]), 4 * 8 + std::mem::size_of::<usize>() * 2);
}


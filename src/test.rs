use crate::DeepSizeOf;
use crate::known_deep_size;

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
    assert_eq!(boxed.deep_size_of(), 4 + std::mem::size_of::<usize>());
}

#[test]
fn slices() {
    let array: [u32; 64] = [0; 64];
    assert_eq!(array[5..10].deep_size_of(), 4 * 5);
    assert_eq!(array[..32].deep_size_of(), 4 * 32);
    assert_eq!(
        DeepSizeOf::deep_size_of(&&array[..8]),
        4 * 8 + std::mem::size_of::<usize>() * 2
    );
}

// TODO: find edge cases
#[test]
fn alignment() {
    #[repr(align(256))]
    struct Test(u8);
    known_deep_size!(0, Test);
    
    struct Test2(Test, u8);
    known_deep_size!(0, Test2);
    
    let array: [Test; 3] = [Test(5), Test(16), Test(2)];
    assert_eq!(std::mem::size_of::<[Test; 3]>(), array.deep_size_of());
    
    let vec = vec![Test(5), Test(16), Test(2)];
    assert_eq!(vec.deep_size_of(), 256 * 3 + 24);
    
    let vec = vec![Test2(Test(5), 0), Test2(Test(16), 0), Test2(Test(2), 0)];
    assert_eq!(vec.deep_size_of(), 512 * 3 + 24);
}

mod context_tests {
    use crate::Context;

    #[test]
    fn context_arc_test() {
        let mut context = Context::new();

        let arc = std::sync::Arc::new(15);
        assert_eq!(context.contains_arc(&arc), false);
        context.add_arc(&arc);
        assert_eq!(context.contains_arc(&arc), true);
    }

    #[test]
    fn context_rc_test() {
        let mut context = Context::new();

        let rc = std::rc::Rc::new(15);
        assert_eq!(context.contains_rc(&rc), false);
        context.add_rc(&rc);
        assert_eq!(context.contains_rc(&rc), true);
    }

    #[test]
    fn context_ref_test() {
        let mut context = Context::new();

        let number = &42;
        assert_eq!(context.contains_ref(number), false);
        context.add_ref(number);
        assert_eq!(context.contains_ref(number), true);
    }
}

#[test]
fn test_derive() {
    
    #[derive(DeepSizeOf)]
    enum Example {
        One,
        Two(),
        Three(u32, Box<u8>),
        Four { name: Box<u32> },
        Five { },
    }
}

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
    assert_eq!(
        DeepSizeOf::deep_size_of(&&array[..8]),
        4 * 8 + std::mem::size_of::<usize>() * 2
    );
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


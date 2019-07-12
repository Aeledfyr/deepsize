use deepsize::DeepSizeOf;

#[derive(DeepSizeOf)]
struct Test {
    a: u32,
    b: Box<u8>,
}

fn main() {
    let object = Test {
        a: 15,
        b: Box::new(255),
    };

    assert_eq!(object.deep_size_of(), std::mem::size_of::<Test>() + 1);
}


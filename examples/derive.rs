
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

#[test]
fn test() {
    #[derive(DeepSizeOf)]
    struct Example<'a>(&'a u32, &'a u32);
    
    let number = &42;
    let example = Example(number, number);
    
    let size = example.deep_size_of();
    
    assert_eq!(size, 2 * std::mem::size_of::<usize>() + 4);
}
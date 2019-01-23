
# deepsize
![](https://img.shields.io/crates/v/deepsize.svg) [![](https://img.shields.io/badge/docs-deepsize-blue.svg)](https://docs.rs/deepsize)

A trait and derive macro to recursively find the size of an object (heap and stack).


## Example Code

```rust
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
    
    assert_eq!(object.deep_size_of(), 17);
}
```


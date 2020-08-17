# ruststreams
An extremely simple stream implementation in Rust

## Usage
```rust
fn main() {
    let mut stream = Stream::new();
    stream.write_item(42);
    assert_eq!(stream.read_item().expect(""), 42);
}
```

## 
Just a simple first in, first out stream.
Write x and then y and when reading you will get x first and then y.

> Yeah, that what that means

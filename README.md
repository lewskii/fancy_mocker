# Fancy mocker

A Rust library for transforming text into alternating uppercase and lowercase letters, resembling the Spongebob Mock Meme.

Inspired by my love for the meme, as well as [a similar library](https://crates.io/crates/mocker) by inxanedev, which unfortunately does not support the way I use the meme. To keep the output readable, I always set the letters 'L' and 'i' to uppercase and lowercase, respectively.

## Example

### printing "Hello, world!" in an alternating case:

```Rust
extern crate fancy_mocker;

fn main() {
    let hello = "Hello, world!";
    let ironic_hello = fancy_mocker::mock(hello);

    println!("{}", ironic_hello);
}
```
`> cargo run`  
`HeLLo, WoRLd!`
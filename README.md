# dedent

A Rust procedural macro for stripping whitespace from multi-line string literals while preserving relative indentation.

## Features

- 📐 Preserves relative indentation between lines
- 🧹 Trims leading and trailing empty lines

## Usage

```rust
use dedent::dedent;

fn main() {
    // Basic usage
    let code = dedent!(r#"
        fn main() {
            println!("Hello, world!");
        }
    "#);
    
    println!("{}", code);
    // Output:
    // fn main() {
    //     println!("Hello, world!");
    // }

    // Works with varying indentation
    let text = dedent!(r#"
        First line
          Indented line
            More indented
        Back to start
    "#);
    
    println!("{}", text);
    // Output:
    // First line
    //   Indented line
    //     More indented
    // Back to start
}
```

## How It Works

The `dedent!` macro:

1. Takes a string literal as input
1. Splits it into lines
1. Calculates the minimum indentation level across all non-empty lines
1. Removes that amount of whitespace from the start of each line
1. Removes leading and trailing empty lines
1. Preserves relative indentation between lines

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Credits

This crate is inspired by the [`dedent` npm package](https://www.npmjs.com/package/dedent).

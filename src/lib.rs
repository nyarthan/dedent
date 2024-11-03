//! A procedural macro for automatically removing common leading whitespace from multi-line strings.
//!
//! # Usage
//!
//! ```rust
//! use dedent::dedent;
//!
//! let formatted = dedent!(r#"
//!     Hello
//!       World
//!         !"#);
//!
//! assert_eq!(formatted, "Hello\n  World\n    !");
//! ```
//!
//! # How it works
//!
//! The macro:
//! 1. Splits the input string into lines
//! 2. Finds the minimum indentation level among non-empty lines
//! 3. Removes that amount of whitespace from the start of each non-empty line
//! 4. Removes leading and trailing empty lines

use std::cmp;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// Removes common leading whitespace from a multi-line string literal.
///
/// This macro is useful for maintaining readable indentation in your source code
/// while removing unnecessary leading whitespace in the final string.
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// use dedent::dedent;
///
/// let code = dedent!(r#"
///     fn main() {
///         println!("Hello, world!");
///     }
///     "#);
///
/// assert_eq!(code, "fn main() {\n    println!(\"Hello, world!\");\n}");
/// ```
///
/// Handling empty lines and varying indentation:
/// ```rust
/// use dedent::dedent;
///
/// let text = dedent!(r#"
///     First line
///       Indented line
///     
///         More indented
///     Last line
///     "#);
/// ```
#[proc_macro]
pub fn dedent(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let input_str = input.value();

    let lines: Vec<&str> = input_str.lines().collect();
    if lines.is_empty() {
        return quote! { String::new() }.into();
    }

    let min_indent = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.len() - line.trim_start().len())
        .min()
        .unwrap_or(0);

    let mut result = String::new();
    let mut first = true;

    for line in lines {
        if !first {
            result.push('\n');
        }
        first = false;

        if line.trim().is_empty() {
            continue;
        }

        let start = cmp::min(min_indent, line.len() - line.trim_start().len());
        result.push_str(&line[start..]);
    }

    let result = result.trim_matches('\n');

    let result_str = LitStr::new(result, input.span());

    quote! { #result_str.to_string() }.into()
}

# Snippets

## Strings

### Allocation
If a `string` is allocated with a capacity that is larger than the elements being added to the buffer
then the buffer reallocates. This means all sized strings that are known beforehand should be 
initialized with a capacity to reduce allocations.

## Traits

### Operator Overloading
Operators that can be be implemented as traits are found in `std::ops`.

## Testing

### Documentation testing
``` rust
/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// `\``
/// let result = doccomments::add(2, 3);
/// assert_eq!(result, 5);
/// `\``
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
[Source](https://doc.rust-lang.org/rust-by-example/testing/doc_testing.html#documentation-testing)



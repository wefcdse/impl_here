# Description
This crate is a simple macro to help impl method for foreign type.
This macro converts a impl block to a trait and impl it for the foreign type.

# Example
```rust
use impl_here::impl_here;

#[impl_here(ArrayTrait)]
impl<T, const L: usize> [T; L] {
    const TOTAL_SIZE: usize = size_of::<Self>();
    fn length(&self) -> usize {
        L
    }
}

assert_eq!([0.3; 125].length(), 125);
assert_eq!(<[String; 2024]>::TOTAL_SIZE, 2024 * 24);

#[impl_here(I32Square)]
impl i32 {
    pub fn square(self) -> i32 {
        self * self
    }
}
// I32Square is public because fn square is public

assert_eq!(13.square(), 13 * 13);

```
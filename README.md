# box_slice
[![downloads](https://img.shields.io/crates/d/box_slice)](https://crates.io/crates/box_slice)
[![dependents](https://img.shields.io/librariesio/dependents/cargo/box_slice)](https://crates.io/crates/box_slice/reverse_dependencies)
![license](https://img.shields.io/crates/l/box_slice)
[![Latest version](https://img.shields.io/crates/v/box_slice.svg)](https://crates.io/crates/box_slice)
[![Documentation](https://docs.rs/box_slice/badge.svg)](https://docs.rs/box_slice)

`box_slice` is a utility crate that implements several helpers to
work with slices in [`Box`](https://doc.rust-lang.org/alloc/boxed/struct.Box.html).

## Basic usage

This package exposes the macro [`boxed`](https://docs.rs/box_slice/latest/box_slice/macro.boxed.html), to rapidely create a boxed
slice from a value and a length.

```rust
use box_slice::boxed;

let boxed_slice = boxed!(45, 3);
let expected_slice = vec![45, 45, 45].into_boxed_slice();

assert_eq!(boxed_slice, expected_slice);
```

It also exposes several functions to create boxed slices from different inputs:
 * From a type's [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) value: [`new_boxed_slice`](https://docs.rs/box_slice/latest/box_slice/fn.new_boxed_slice.html)
 * From a value that implements [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html): [`new_boxed_slice_with_value`](https://docs.rs/box_slice/latest/box_slice/fn.new_boxed_slice_with_value.html)
 * From a parameterless function: [`new_boxed_slice_with_initializer`](https://docs.rs/box_slice/latest/box_slice/fn.new_boxed_slice_with_initializer.html)
 * From a function that takes as parameter the index of the element being created:[`new_boxed_slice_with_indexed_initializer`](https://docs.rs/box_slice/latest/box_slice/fn.new_boxed_slice_with_indexed_initializer.html)

```rust
use box_slice::new_boxed_slice_with_indexed_initializer;

let boxed_slice = new_boxed_slice_with_indexed_initializer(|i| i * 2, 5);
let expected_slice = vec![0, 2, 4, 6, 8].into_boxed_slice();

assert_eq!(boxed_slice, expected_slice);
```

## Helper type

The crate also exposes the helper wrapper [`BoxedSlice<T>`](https://docs.rs/box_slice/latest/box_slice/struct.BoxedSlice.html) that wraps around
[`Box<[T]>`](https://doc.rust-lang.org/latest/alloc/boxed/struct.Box.html) and allows fluent constructors.

## no_std

This crate is `no_std` compatible, but it requires the `alloc` crate.

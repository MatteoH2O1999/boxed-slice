#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

//! `box_slice` is a utility crate that implements several helpers to
//! work with slices in [`Box`](`alloc::boxed::Box`).
//!
//! # Basic usage
//!
//! This package exposes the macro [`boxed!`], to rapidely create a boxed
//! slice from a value and a length.
//!
//! ```
//! use box_slice::boxed;
//!
//! let boxed_slice = boxed!(45, 3);
//! let expected_slice = vec![45, 45, 45];
//!
//! assert_eq!(boxed_slice.as_slice(), expected_slice);
//! ```
//!
//! It also exposes several functions to create boxed slices from different inputs:
//! * From a type's [`Default`] value: [`new_boxed_slice`]
//! * From a value that implements [`Clone`]: [`new_boxed_slice_with_value`]
//! * From a parameterless function: [`new_boxed_slice_with_initializer`]
//! * From a function that takes as parameter the index of the element being created:
//!   [`new_boxed_slice_with_indexed_initializer`]
//!
//! ```
//! use box_slice::new_boxed_slice_with_indexed_initializer;
//!
//! let boxed_slice = new_boxed_slice_with_indexed_initializer(|i| i * 2, 5);
//! let expected_slice = vec![0, 2, 4, 6, 8].into_boxed_slice();
//!
//! assert_eq!(boxed_slice, expected_slice);
//! ```
//!
//! # Helper type
//!
//! The crate also exposes the helper wrapper [`BoxedSlice<T>`] that wraps around
//! [`Box<[T]>`](`alloc::boxed::Box`) and allows fluent constructors.
//!
//! # no_std
//!
//! This crate is `no_std` compatible, but it requires the `alloc` crate.

mod boxed_slice;
pub use boxed_slice::*;

mod boxed_macro;

extern crate alloc;

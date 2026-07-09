use alloc::boxed::Box;
use core::ops::{Deref, DerefMut};

pub fn new_boxed_slice<T: Default>(len: usize) -> Box<[T]> {
    new_boxed_slice_with_initializer(T::default, len)
}

pub fn new_boxed_slice_with_value<T: Clone>(value: T, len: usize) -> Box<[T]> {
    todo!()
}

pub fn new_boxed_slice_with_initializer<T>(mut func: impl FnMut() -> T, len: usize) -> Box<[T]> {
    new_boxed_slice_with_indexed_initializer(|_| func(), len)
}

pub fn new_boxed_slice_with_indexed_initializer<T>(
    func: impl FnMut(usize) -> T,
    len: usize,
) -> Box<[T]> {
    todo!()
}

#[repr(transparent)]
pub struct BoxedSlice<T>(alloc::boxed::Box<[T]>);

impl<T> BoxedSlice<T> {
    #[inline]
    pub fn new(len: usize) -> Self
    where
        T: Default,
    {
        Self(new_boxed_slice(len))
    }

    #[inline]
    pub fn with_value(value: T, len: usize) -> Self
    where
        T: Clone,
    {
        Self(new_boxed_slice_with_value(value, len))
    }

    #[inline]
    pub fn with_initializer(func: impl FnMut() -> T, len: usize) -> Self {
        Self(new_boxed_slice_with_initializer(func, len))
    }

    #[inline]
    pub fn with_indexed_initializer(func: impl FnMut(usize) -> T, len: usize) -> Self {
        Self(new_boxed_slice_with_indexed_initializer(func, len))
    }

    #[inline]
    pub fn into_inner(self) -> Box<[T]> {
        self.0
    }
    #[inline]
    pub fn into_box(self) -> Box<[T]> {
        self.into_inner()
    }
}

impl<T> From<BoxedSlice<T>> for Box<[T]> {
    #[inline]
    fn from(value: BoxedSlice<T>) -> Self {
        value.into_inner()
    }
}

impl<T> Deref for BoxedSlice<T> {
    type Target = Box<[T]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for BoxedSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use alloc::boxed::Box;
use core::{
    mem::{forget, MaybeUninit},
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
struct PanicGuard<'a, T> {
    slice: &'a mut [MaybeUninit<T>],
    initialized: usize,
}

impl<'a, T> Drop for PanicGuard<'a, T> {
    fn drop(&mut self) {
        let initialized_part = &mut self.slice[..self.initialized];
        unsafe {
            // SAFETY: this raw sub-slice will contain only initialized objects.
            initialized_part.assume_init_drop();
        }
    }
}

pub fn new_boxed_slice<T: Default>(len: usize) -> Box<[T]> {
    new_boxed_slice_with_initializer(T::default, len)
}

pub fn new_boxed_slice_with_value<T: Clone>(value: T, len: usize) -> Box<[T]> {
    let mut uninit_box = Box::new_uninit_slice(len);
    {
        let mut guard = PanicGuard {
            slice: &mut uninit_box,
            initialized: 0,
        };
        if let Some((last, elems)) = guard.slice.split_last_mut() {
            for elem in elems {
                elem.write(value.clone());
                guard.initialized += 1;
            }
            last.write(value);
        }
        forget(guard);
    }
    unsafe {
        // Safety: we just wrote len valid elements
        uninit_box.assume_init()
    }
}

pub fn new_boxed_slice_with_initializer<T>(mut func: impl FnMut() -> T, len: usize) -> Box<[T]> {
    new_boxed_slice_with_indexed_initializer(|_| func(), len)
}

pub fn new_boxed_slice_with_indexed_initializer<T>(
    mut func: impl FnMut(usize) -> T,
    len: usize,
) -> Box<[T]> {
    let mut uninit_box = Box::new_uninit_slice(len);
    {
        let mut guard = PanicGuard {
            slice: &mut uninit_box,
            initialized: 0,
        };
        for elem in guard.slice.iter_mut() {
            elem.write(func(guard.initialized));
            guard.initialized += 1;
        }
        forget(guard);
    }
    unsafe {
        // Safety: we just wrote len valid elements
        uninit_box.assume_init()
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BoxedSlice<T>(alloc::boxed::Box<[T]>);

impl<T> BoxedSlice<T> {
    pub fn new_empty() -> Self {
        Self(Box::new([]))
    }

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

#[cfg(test)]
mod test {
    use alloc::vec;

    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct TestType {
        value: usize,
    }

    impl Default for TestType {
        fn default() -> Self {
            Self { value: 42 }
        }
    }

    #[test]
    fn test_empty_default() {
        let slice = BoxedSlice::<TestType>::new(0);

        assert!(slice.is_empty());
    }

    #[test]
    fn test_empty_value() {
        let slice = BoxedSlice::with_value(TestType { value: 69 }, 0);

        assert!(slice.is_empty());
    }

    #[test]
    fn test_empty_init() {
        let slice = BoxedSlice::with_indexed_initializer(|i| TestType { value: i }, 0);

        assert!(slice.is_empty());
    }

    #[test]
    fn test_default() {
        let slice = BoxedSlice::<TestType>::new(5);
        let expected = vec![TestType::default(); 5].into_boxed_slice();

        assert_eq!(slice.into_inner(), expected);
    }

    #[test]
    fn test_value() {
        let slice = BoxedSlice::with_value(TestType { value: 69 }, 5);
        let expected = vec![TestType { value: 69 }; 5].into_boxed_slice();

        assert_eq!(slice.into_inner(), expected);
    }

    #[test]
    fn test_init() {
        let slice = BoxedSlice::with_indexed_initializer(|i| TestType { value: i }, 5);
        let expected = vec![
            TestType { value: 0 },
            TestType { value: 1 },
            TestType { value: 2 },
            TestType { value: 3 },
            TestType { value: 4 },
        ]
        .into_boxed_slice();

        assert_eq!(slice.into_inner(), expected);
    }
}

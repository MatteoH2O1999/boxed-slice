use alloc::boxed::Box;
use core::{
    borrow::{Borrow, BorrowMut},
    mem::{MaybeUninit, forget},
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

/// Creates a new empty boxed slice of the specified type.
///
/// # Examples
/// Usually the type must be specified with turbofish:
/// ```
/// use box_slice::new_empty_boxed_slice;
///
/// let boxed_slice = new_empty_boxed_slice::<f64>();
/// assert!(boxed_slice.is_empty());
/// ```
/// It can also be specified with the variable declaration:
/// ```
/// use box_slice::new_empty_boxed_slice;
///
/// let boxed_slice: Box<[f64]> = new_empty_boxed_slice();
/// assert!(boxed_slice.is_empty());
/// ```
#[inline]
pub fn new_empty_boxed_slice<T>() -> Box<[T]> {
    Box::new([])
}

/// Creates a boxed slice from the given array.
///
/// # Examples
/// The type is usually inferred for non-empty arrays:
/// ```
/// use box_slice::new_boxed_slice_from_array;
///
/// let boxed_slice = new_boxed_slice_from_array([1.0f64, 3.0]); // Inferred: Box<[f64]>
/// let expected = vec![1.0f64, 3.0].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
#[inline]
pub fn new_boxed_slice_from_array<T, const N: usize>(array: [T; N]) -> Box<[T]> {
    Box::new(array)
}

/// Creates a boxed slice of length `len` initialized using the type's [`Default`].
///
/// # Examples
/// Usually the type must be specified with turbofish:
/// ```
/// use box_slice::new_boxed_slice;
///
/// let boxed_slice = new_boxed_slice::<f64>(5);
/// let expected = vec![f64::default(); 5].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
/// It can also be specified with the variable declaration:
/// ```
/// use box_slice::new_boxed_slice;
///
/// let boxed_slice: Box<[f64]> = new_boxed_slice(5);
/// let expected = vec![f64::default(); 5].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
///
/// # Panic
/// This function panics if any call to [`Default`] panics.
/// Elements already written to the slice are dropped.
#[inline]
pub fn new_boxed_slice<T: Default>(len: usize) -> Box<[T]> {
    new_boxed_slice_with_initializer(T::default, len)
}

/// Creates a boxed slice of length `len` initialized to `value`.
///
/// The passed value is inserted as the last slice element, while the others
/// are obtained through [`Clone`].
///
/// # Examples
/// The type can usually be inferred:
/// ```
/// use box_slice::new_boxed_slice_with_value;
///
/// let boxed_slice = new_boxed_slice_with_value(1.5f64, 5); // Inferred: Box<[f64]>
/// let expected = vec![1.5f64; 5].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
///
/// # Panic
/// This function panics if any call to [`Clone`] panics.
/// Elements already written to the slice are dropped.
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

/// Creates a boxed slice of length `len` initialized using `func`.
///
/// # Examples
/// The type can usually be inferred from the passed closure:
/// ```
/// use box_slice::new_boxed_slice_with_initializer;
///
/// let boxed_slice = new_boxed_slice_with_initializer(|| 15.0f64, 5); // Inferred: Box<[f64]>
/// let expected = vec![15.0f64; 5].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
///
/// # Panic
/// This function panics if any call to `func` panics.
/// Elements already written to the slice are dropped.
#[inline]
pub fn new_boxed_slice_with_initializer<T>(mut func: impl FnMut() -> T, len: usize) -> Box<[T]> {
    new_boxed_slice_with_indexed_initializer(|_| func(), len)
}

/// Creates a boxed slice of length `len` initialized using `func`,
///
/// `func` takes an [`usize`] from `0` to `len - 1` and returns the `n-th`
/// element of the boxed slice.
///
/// # Examples
/// The type can usually be inferred from the passed closure:
/// ```
/// use box_slice::new_boxed_slice_with_indexed_initializer;
///
/// let boxed_slice = new_boxed_slice_with_indexed_initializer(|i| i, 5); // Inferred: Box<[usize]>
/// let expected = vec![0, 1, 2, 3, 4].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
///
/// # Panic
/// This function panics if any call to `func` panics.
/// Elements already written to the slice are dropped.
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

/// Convenience struct to handle boxed slices. Equivalent to [`Box<[T]>`](`Box`).
///
/// # Examples
/// ```
/// use box_slice::BoxedSlice;
///
/// let mut boxed_slice = BoxedSlice::with_value(5usize, 3); // Type: BoxedSlice<usize>
///
/// assert_eq!(boxed_slice.len(), 3);
/// assert_eq!(boxed_slice.as_slice(), [5, 5, 5]);
///
/// boxed_slice[0] = 3;
/// assert_eq!(boxed_slice.as_slice(), [3, 5, 5]);
///
/// let inner = boxed_slice.into_inner(); // Type: Box<[usize]>
/// ```
/// The [`boxed!`](`crate::boxed`) macro provides convenient initalization methods:
/// ```
/// use box_slice::boxed;
///
/// let mut boxed_slice = boxed!(5usize, 3); // Type: BoxedSlice<usize>
///
/// assert_eq!(boxed_slice.len(), 3);
/// assert_eq!(boxed_slice.as_slice(), [5, 5, 5]);
///
/// boxed_slice[0] = 3;
/// assert_eq!(boxed_slice.as_slice(), [3, 5, 5]);
///
/// let inner = boxed_slice.into_inner(); // Type: Box<[usize]>
/// ```
///
/// # Wrapper
/// This is just a wrapper around [`Box<[T]>`](`Box`).
/// As such it implements [`Deref`] targeting [`Box<[T]>`](`Box`).
///
/// Convenience methods like [`as_slice`](`Self::as_slice`) and [`as_mut_slice`](`Self::as_mut_slice`),
/// are provided for a more fluent syntax.
///
/// # Constructors
/// [`BoxedSlice`] wraps around all the functions in this crate to create [`Box<[T]>`](`Box`):
/// * [`new_empty`](`Self::new_empty`): [`new_empty_boxed_slice`]
/// * [`from_array`](`Self::from_array`): [`new_boxed_slice_from_array`]
/// * [`new`](`Self::new`): [`new_boxed_slice`]
/// * [`with_value`](`Self::with_value`): [`new_boxed_slice_with_value`]
/// * [`with_initializer`](`Self::with_initializer`): [`new_boxed_slice_with_initializer`]
/// * [`with_indexed_initializer`](`Self::with_indexed_initializer`): [`new_boxed_slice_with_indexed_initializer`]
///
/// In addition, it exposes an additional constructor that takes a [`Box<[T]>`](`Box`):[`from_box`](`Self::from_box`).
///
/// # Repr
/// This wrapper uses `repr(transparent)` to guarantee the same ABIs as [`Box<[T]>`](`Box`).
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoxedSlice<T>(alloc::boxed::Box<[T]>);

impl<T> BoxedSlice<T> {
    #[inline]
    pub fn new_empty() -> Self {
        Self(new_empty_boxed_slice())
    }

    #[inline]
    pub fn from_box(b: Box<[T]>) -> Self {
        Self(b)
    }

    #[inline]
    pub fn from_array<const N: usize>(array: [T; N]) -> Self {
        Self(new_boxed_slice_from_array(array))
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

    #[inline]
    pub fn into_box(self) -> Box<[T]> {
        self.into_inner()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

impl<T> Default for BoxedSlice<T> {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl<T> From<BoxedSlice<T>> for Box<[T]> {
    #[inline]
    fn from(value: BoxedSlice<T>) -> Self {
        value.into_inner()
    }
}

impl<T> From<Box<[T]>> for BoxedSlice<T> {
    #[inline]
    fn from(value: Box<[T]>) -> Self {
        Self::from_box(value)
    }
}

impl<T> AsRef<[T]> for BoxedSlice<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T> AsMut<[T]> for BoxedSlice<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> Borrow<[T]> for BoxedSlice<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        self
    }
}

impl<T> BorrowMut<[T]> for BoxedSlice<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> AsRef<Box<[T]>> for BoxedSlice<T> {
    #[inline]
    fn as_ref(&self) -> &Box<[T]> {
        self
    }
}

impl<T> AsMut<Box<[T]>> for BoxedSlice<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut Box<[T]> {
        self
    }
}

impl<T> Borrow<Box<[T]>> for BoxedSlice<T> {
    #[inline]
    fn borrow(&self) -> &Box<[T]> {
        self
    }
}

impl<T> BorrowMut<Box<[T]>> for BoxedSlice<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut Box<[T]> {
        self
    }
}

impl<T> Deref for BoxedSlice<T> {
    type Target = Box<[T]>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for BoxedSlice<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;

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
    fn empty_default() {
        let slice = BoxedSlice::<TestType>::new(0);

        assert!(slice.is_empty());
    }

    #[test]
    fn empty_value() {
        let slice = BoxedSlice::with_value(TestType { value: 69 }, 0);

        assert!(slice.is_empty());
    }

    #[test]
    fn empty_init() {
        let slice = BoxedSlice::with_indexed_initializer(|i| TestType { value: i }, 0);

        assert!(slice.is_empty());
    }

    #[test]
    fn default() {
        let slice = BoxedSlice::<TestType>::new(5);
        let expected = vec![TestType::default(); 5].into_boxed_slice();

        assert_eq!(slice.into_inner(), expected);
    }

    #[test]
    fn value() {
        let slice = BoxedSlice::with_value(TestType { value: 69 }, 5);
        let expected = vec![TestType { value: 69 }; 5].into_boxed_slice();

        assert_eq!(slice.into_inner(), expected);
    }

    #[test]
    fn init() {
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

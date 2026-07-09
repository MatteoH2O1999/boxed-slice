/// Creates a [`Box`](`alloc::boxed::Box`) containing the arguments.
///
/// `boxed!` allows boxed slices to be defined either enumerating the elements or
/// with a value and a length.
/// There are three forms to this macro:
/// * Create an empty boxed slice:
/// ```
/// # use box_slice::boxed;
/// let boxed_slice: Box<[f64]> = boxed!();
/// let expected = Box::new([]) as Box<[f64]>;
/// assert_eq!(boxed_slice, expected);
/// ```
/// * Create a boxed slice from an array:
/// ```
/// # use box_slice::boxed;
/// let boxed_slice = boxed!([42, 69, 2]);
/// let expected = vec![42, 69, 2].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
/// * Create a boxed slice from a given element and size:
/// ```
/// # use box_slice::boxed;
/// let boxed_slice = boxed!(42, 3);
/// let expected = vec![42; 3].into_boxed_slice();
/// assert_eq!(boxed_slice, expected);
/// ```
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the number of elements doesn't have to be
/// a constant.
///
/// This will use `clone` to duplicate an expression, so one should be careful
/// using this with types having a nonstandard `Clone` implementation. For
/// example, `boxed!(Rc::new(1), 5)` will create a boxed slice of five references
/// to the same boxed integer value, not five references pointing to independently
/// boxed integers.
///
/// Also, note that `boxed!(expr, 0)` is allowed, and produces an empty boxed slice.
/// This will still evaluate `expr`, however, and immediately drop the resulting value, so
/// be mindful of side effects.
#[macro_export]
macro_rules! boxed {
    () => {
        $crate::new_empty_boxed_slice()
    };
    ($elem:expr) => {
        $crate::new_boxed_slice_from_array($elem)
    };
    ($elem:expr, $n:expr) => {
        $crate::new_boxed_slice_with_value($elem, $n)
    };
}

#[cfg(test)]
mod test {
    use alloc::{boxed::Box, vec};

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
    fn empty() {
        let slice: Box<[usize]> = boxed!();

        assert!(slice.is_empty());
    }

    #[test]
    fn value() {
        let slice = boxed!(TestType { value: 69 }, 5);
        let expected = vec![TestType { value: 69 }; 5].into_boxed_slice();

        assert_eq!(slice, expected);
    }

    #[test]
    fn two_values() {
        let slice = boxed!([TestType { value: 420 }, TestType { value: 690 }]);
        let expected = vec![TestType { value: 420 }, TestType { value: 690 }].into_boxed_slice();

        assert_eq!(slice, expected);
    }

    #[test]
    fn values() {
        let slice = boxed!([
            TestType { value: 0 },
            TestType { value: 1 },
            TestType { value: 2 },
            TestType { value: 3 },
            TestType { value: 4 }
        ]);
        let expected = vec![
            TestType { value: 0 },
            TestType { value: 1 },
            TestType { value: 2 },
            TestType { value: 3 },
            TestType { value: 4 },
        ]
        .into_boxed_slice();

        assert_eq!(slice, expected);
    }
}

#[macro_export]
macro_rules! boxed {
    () => {
        alloc::boxed::Box::new([]) as alloc::boxed::Box<[_]>
    };
    ($elem:expr) => {
        alloc::boxed::Box::new($elem) as alloc::boxed::Box<[_]>
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

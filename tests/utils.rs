#![allow(unused_macros)]

macro_rules! assert_iter {
    ($it:expr, $expected:expr $(,)?) => {{
        let mut it = $it;
        for (i, e) in $expected.iter().enumerate() {
            assert_eq!(
                &it.next().unwrap_or_else(|| panic!("unwrap failed on {i}")),
                e,
                "assert failed on {i}"
            );
        }
        assert!(it.next().is_none());
    }};
}

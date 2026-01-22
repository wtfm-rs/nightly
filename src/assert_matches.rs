//! This should only work in nightly.
//!
//! ```sh
//! % make test
//! rustup run nightly rustc --test src/assert_matches.rs && ./assert_matches
//!
//! running 1 test
//! test tests::test_assert_matches - should panic ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//! ```
#![feature(assert_matches)]
#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_assert_matches() {
        use std::assert_matches::assert_matches;
        let a = Some(345);
        let b = Some(56);
        assert_matches!(a, Some(_));
        assert_matches!(b, Some(_));

        assert_matches!(a, Some(345));
        assert_matches!(a, Some(345) | None);

        assert_matches!(a, None); // panics
        assert_matches!(b, Some(345)); // panics
        assert_matches!(b, Some(345) | None); // panics
        assert_matches!(a, Some(x) if x > 100);
    }
}

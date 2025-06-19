/// module for testing functions
pub mod testing{
    /// assert a number according a persistion value
    #[macro_export] macro_rules! assert_aprx {
        ($left:expr, $right:expr, $epsilon:expr $(,)?) => {{
            let (left_val, right_val, epsilon_val) = ($left, $right, $epsilon);
            if (left_val - right_val).abs() > epsilon_val {
                panic!(
                    "assertion failed: `(left ≈ right)` \
                    (left: `{}`, right: `{}`, epsilon: `{}`)",
                    left_val, right_val, epsilon_val
                );
            }
        }};
    }
}
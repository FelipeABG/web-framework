/// A macro to check if a given string contains any of the specified substrings.
///
/// # Examples
///
/// ```
/// // Import the macro
/// use your_crate_name::contains_any;
///
/// let my_string = "Hello, world!";
///
/// // Check if the string contains any of the specified substrings
/// assert!(contains_any!(my_string, "Hello", "world")); // true
/// assert!(contains_any!(my_string, "world"));          // true
/// assert!(!contains_any!(my_string, "Rust", "macro")); // false
/// ```
///
/// # How it works
///
/// This macro takes a string and a list of string literals as input.
/// It checks if the input string contains **any** of the specified literals
/// and returns `true` if at least one match is found, otherwise it returns `false`.
///
/// # Parameters
/// - `$target`: The string to search within.
/// - `$patterns`: A comma-separated list of string literals to search for.
///
/// # Returns
/// A boolean value:
/// - `true` if any of the specified literals are found in the string.
/// - `false` otherwise.
#[macro_export]
macro_rules! contains_any {
    ($target:expr, $($pattern:literal),+) => {{
        let target_str: &str = $target;
        let mut matches = Vec::new();
        $(
            matches.push(target_str.contains($pattern));
        )+
        matches.contains(&true)
    }};
}

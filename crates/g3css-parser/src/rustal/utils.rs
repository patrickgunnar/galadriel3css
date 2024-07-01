use crate::Rule;

/// Removes leading and trailing whitespace from the input string.
///
/// Splits the input string by whitespace and returns the first segment without leading or trailing whitespace.
/// Returns an empty string if the input is empty or consists only of whitespace.
///
/// # Arguments
///
/// - `input` - A reference to a string slice (`&str`) from which whitespace should be removed.
///
/// # Returns
///
/// A new `String` with leading and trailing whitespace removed from the input, or an empty string if no non-whitespace characters are found.
pub fn remove_whitespace(input: &str) -> String {
    // Split the input string by whitespace and retrieve the first segment without leading or trailing whitespace
    input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches('"')
        .to_string()
}

/// Generates a vector of strings representing an alias/variable from a Pest `Pair`.
///
/// Constructs a vector of strings based on inner pairs of the provided `Pair`.
/// Returns `Some(Vec<String>)` with the constructed alias/variable vector, or `None` if
/// parsing or construction fails.
///
/// # Arguments
///
/// - `pair` - A `Pair` from the Pest parser representing alias/variable components.
///
/// # Returns
///
/// An `Option<Vec<String>>` containing the constructed vector of strings
/// representing an alias/variable, or `None` if parsing fails.
pub fn generates_string_vec(pair: pest::iterators::Pair<Rule>) -> Option<Vec<String>> {
    // Create an empty vector to hold the alias/variable components
    let mut alias_var = vec![];

    // Iterate over each inner pair within the provided pair
    for inner_pair in pair.into_inner() {
        // Match the rule of the inner pair to determine the alias/variable component type
        match inner_pair.as_rule() {
            // If it matches Rule::leading, push the trimmed string to alias/variable vector
            Rule::leading => {
                alias_var.push(remove_whitespace(inner_pair.as_str()));
            }
            // If it matches Rule::importance, push the trimmed string to alias vector
            Rule::importance => {
                alias_var.push(remove_whitespace(inner_pair.as_str()));
            }
            // If it matches Rule::worth, push the trimmed string to variable vector
            Rule::worth => {
                alias_var.push(remove_whitespace(inner_pair.as_str()));
            }
            // Ignore other rules
            _ => (),
        }
    }

    // Return the alias vector wrapped in `Some`, indicating successful construction
    Some(alias_var)
}

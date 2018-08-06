use std::sync::Arc;

use regex::Regex;

/// Strips all duplicate whitespace from the `input` string.
pub fn strip_whitespace(input: Arc<String>) -> Arc<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\s{2,}|[\r\n]")
            .expect("Failed to compile white-space stripping regex!");
    }
    let content: String = RE.replace_all(&input, " ").into_owned();
    Arc::new(content)
}

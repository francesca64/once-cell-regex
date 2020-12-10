pub mod exports {
    pub use once_cell;
    pub use regex;
}

#[macro_export]
macro_rules! regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::exports::once_cell::sync::OnceCell<$crate::exports::regex::Regex> =
            $crate::exports::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| $crate::exports::regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! regex_multi_line {
    ($re:expr $(,)?) => {{
        static RE: $crate::exports::once_cell::sync::OnceCell<$crate::exports::regex::Regex> =
            $crate::exports::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| {
            $crate::exports::regex::RegexBuilder::new($re)
                .multi_line(true)
                .build()
                .unwrap()
        })
    }};
}

#[macro_export]
macro_rules! byte_regex {
    ($re:expr $(,)?) => {{
        static RE: $crate::exports::once_cell::sync::OnceCell<
            $crate::exports::regex::bytes::Regex,
        > = $crate::exports::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| $crate::exports::regex::bytes::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! byte_regex_multi_line {
    ($re:expr $(,)?) => {{
        static RE: $crate::exports::once_cell::sync::OnceCell<
            $crate::exports::regex::bytes::Regex,
        > = $crate::exports::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| {
            $crate::exports::regex::bytes::RegexBuilder::new($re)
                .multi_line(true)
                .build()
                .unwrap()
        })
    }};
}

#[cfg(test)]
mod tests {
    use regex::{bytes, Regex};

    #[test]
    fn regex_macro() {
        let _: &Regex = regex!(r"\w?");
    }

    #[test]
    fn regex_multi_line_macro() {
        let _: &Regex = regex_multi_line!(r"\w?");
    }

    #[test]
    fn byte_regex_macro() {
        let _: &bytes::Regex = byte_regex!(r"\w?");
    }

    #[test]
    fn byte_regex_multi_line_macro() {
        let _: &bytes::Regex = byte_regex_multi_line!(r"\w?");
    }
}

pub mod exports {
    pub use once_cell;
    pub use regex;
}

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: $crate::exports::once_cell::sync::OnceCell<$crate::exports::regex::Regex> =
            $crate::exports::once_cell::sync::OnceCell::new();
        RE.get_or_init(|| $crate::exports::regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! regex_multi_line {
    ($re:literal $(,)?) => {{
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

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn regex_macro() {
        let _: &Regex = regex!(r"\w?");
    }

    #[test]
    fn regex_multi_line_macro() {
        let _: &Regex = regex_multi_line!(r"\w?");
    }
}

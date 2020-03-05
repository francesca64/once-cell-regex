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

#[cfg(test)]
mod tests {
    use regex::Regex;

    #[test]
    fn regex_macro_lit() {
        let _: &Regex = regex!(r"\w?");
    }

    #[test]
    fn regex_macro_expr() {
        let n = 5;
        let re: &Regex = regex!(&format!(r"\w{{{}}}", n));
        let s = "hotdogs";
        assert_eq!(re.find(s).map(|m| m.as_str()), Some(&s[0..n]));
    }

    #[test]
    fn regex_multi_line_macro_lit() {
        let _: &Regex = regex_multi_line!(r"\w?");
    }
}

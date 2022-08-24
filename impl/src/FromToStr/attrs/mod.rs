use syn::{spanned::Spanned, Error, Lit, Meta, MetaList, NestedMeta};

use crate::utils;

macro_rules! try_get {
    ($attr:ident; $($field:ident => $typ:ty),*) => {
        if !$attr.path.is_ident("fromtostr") {
            continue;
        };
        let list: ::syn::MetaList = $attr.parse_args()?;

        $(if list.path.is_ident(stringify![$field]) {
            if $field.is_none() {
                $field = Some(<$typ>::from_attr(list)?);
                continue;
            };
            return Err(::syn::Error::new_spanned(list, "duplicate attribute!"));
        })*
        return Err(::syn::Error::new_spanned(list, "unknown attribute!"));
    };
}

macro_rules! malformed_err {
    ($span:ident, $($l:literal)*) => {
        move || {
            ::syn::Error::new(
                $span,
                concat!["malformed attribute, expected #[fromtostr(", $($l),*, ")]"],
            )
        }
    };
}

pub mod inner;
pub mod outer;

#[allow(dead_code)] // triggers before 1.45
pub enum FormatCase {
    /// keep it as is
    None,
    /// "TwoWords" => "twowords"
    Lower,
    /// "TwoWords" => "twowords"
    Upper,
    /// "TwoWords" => "twoWords"
    Camel,
    /// "TwoWords" => "Two{separator}Words"
    Delimited { sep: String },
    /// "TwoWords" => "two{separator}words"
    DelimitedLower { sep: String },
    /// "TwoWords" => "TWO{separator}WORDS"
    DelimitedUpper { sep: String },
    /// "TwoWords" => "two_words"
    Snake,
    /// "TwoWords" => "TWO_WORDS"
    ScreamingSnake,
    /// "TwoWords" => "two-words"
    Kebab,
    /// "TwoWords" => "TWO-WORDS"
    ScreamingKebab,
    /// "TwoWords" => "Two-Words"
    Train,
    /// "TwoWords" => "Two_Words"
    PascalSnake,
    /// "TwoWords" => "two_Words"
    CamelSnake,
}

impl FormatCase {
    pub fn from_attr(attr: &MetaList, malformed_err: impl FnOnce() -> Error) -> syn::Result<Self> {
        let attr_span = attr.span();
        let mut separator = None;
        let mut style = None;

        for nested in &attr.nested {
            let nv = match nested {
                NestedMeta::Meta(Meta::NameValue(nv)) => nv,
                _ => return Err(malformed_err()),
            };

            match (
                utils::opt_as_deref(&nv.path.get_ident().map(ToString::to_string)),
                &nv.lit,
            ) {
                (Some("separator"), Lit::Str(sep)) => separator = Some(sep.value()),
                (Some("style"), Lit::Str(sty)) => style = Some((sty.value(), sty.span())),
                _ => return Err(malformed_err()),
            }
        }

        let (style, not_style_error) = match style {
            Some(a) => {
                let e = a.1;
                (a.0, move |s| {
                    Error::new(e, format!("{} not an avaliable style!", s))
                })
            }
            None => return Err(malformed_err()),
        };
        let no_sep_error =
            || Error::new(attr_span, r#"style = "delimited" needs a separator value"#);
        Ok(match style.trim() {
            "none" | "PascalCase" => Self::None,
            "lower" => Self::Lower,
            "UPPER" => Self::Upper,
            "snake" => Self::Snake,
            "SCREAMING_SNAKE" => Self::ScreamingSnake,
            "kebab" => Self::Kebab,
            "SCREAMING-KEBAB" => Self::ScreamingKebab,
            "camel" => Self::Camel,
            "camel_Snake" => Self::CamelSnake,
            "Pascal_Snake" => Self::PascalSnake,
            "Train" => Self::Train,
            "delimited" => {
                if let Some(sep) = separator {
                    Self::Delimited { sep }
                } else {
                    return Err(no_sep_error());
                }
            }
            "delimitedlower" => {
                if let Some(sep) = separator {
                    Self::DelimitedLower { sep }
                } else {
                    return Err(no_sep_error());
                }
            }
            "DELIMITEDUPPER" => {
                if let Some(sep) = separator {
                    Self::DelimitedUpper { sep }
                } else {
                    return Err(no_sep_error());
                }
            }
            s => return Err(not_style_error(s)),
        })
    }

    pub fn format(&self, s: &str) -> String {
        match self {
            Self::None => s.to_owned(),
            Self::Lower => s.to_lowercase(),
            Self::Upper => s.to_uppercase(),
            Self::Camel => s[..1].to_lowercase() + &s[1..],
            Self::Delimited { sep } => Self::delimit(sep, s),
            Self::DelimitedLower { sep } => Self::delimit(sep, s).to_lowercase(),
            Self::DelimitedUpper { sep } => Self::delimit(sep, s).to_uppercase(),
            Self::Train => Self::delimit("-", s),
            Self::PascalSnake => Self::delimit("_", s),
            Self::CamelSnake => {
                let s = Self::delimit("_", s);

                s[..1].to_lowercase() + &s[1..]
            }
            Self::Snake => Self::delimit("_", s).to_lowercase(),
            Self::Kebab => Self::delimit("-", s).to_lowercase(),
            Self::ScreamingSnake => Self::delimit("_", s).to_uppercase(),
            Self::ScreamingKebab => Self::delimit("-", s).to_uppercase(),
        }
    }

    pub fn delimit(sep: &str, s: &str) -> String {
        let mut string = String::with_capacity(s.len() + (4 * sep.len()));
        let mut chars = s.chars().peekable();
        // don't try to add an separator at the start
        string.push(chars.next().unwrap());

        let mut prev_is_upper = None;
        while let Some(ch) = chars.next() {
            let is_upper = ch.is_uppercase();
            let next_is_upper = chars.peek().map(|ch| ch.is_uppercase());

            // if this character is upper, and neither the previous and next is upper, add separator
            if is_upper && (next_is_upper == Some(false) || prev_is_upper == Some(false)) {
                string.push_str(sep);
            }
            string.push(ch);
            prev_is_upper = Some(is_upper);
        }

        string.shrink_to_fit();
        string
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        use FormatCase::*;
        let orig = "TwoWords";

        assert_eq!(Lower.format(orig), "twowords");
        assert_eq!(Upper.format(orig), "TWOWORDS");
        assert_eq!(Camel.format(orig), "twoWords");
        assert_eq!(Snake.format(orig), "two_words");
        assert_eq!(ScreamingSnake.format(orig), "TWO_WORDS");
        assert_eq!(Kebab.format(orig), "two-words");
        assert_eq!(ScreamingKebab.format(orig), "TWO-WORDS");
        assert_eq!(Train.format(orig), "Two-Words");
        assert_eq!(PascalSnake.format(orig), "Two_Words");
        assert_eq!(CamelSnake.format(orig), "two_Words");

        // check if initialisms work
        assert_eq!(Train.format("HTTPRequest"), "HTTP-Request");
    }
}

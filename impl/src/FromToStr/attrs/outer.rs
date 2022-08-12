use crate::utils;

pub use super::super::Impl;
use syn::{spanned::Spanned, Error, Lit, Meta, MetaList, NestedMeta};

pub struct Attrs {
    pub format: Option<FormatCase>,
    pub skip: Option<Skip>,
}

impl Attrs {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut format = None;
        let mut skip = None;

        for attr in attrs {
            try_get! {attr;
                format => FormatCase,
                skip => Skip
            }
        }
        Ok(Self { format, skip })
    }
    #[inline]
    pub fn should_skip(&self, imp: Impl) -> bool {
        self.skip.as_ref().map_or(false, |s| s.should_skip(imp))
    }
}

pub struct Skip(pub Vec<Impl>);

impl Skip {
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let mut vec = Vec::with_capacity(attr.nested.len());
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"skip("..."*)"#);

        for nested in attr.nested {
            if let NestedMeta::Meta(Meta::Path(path)) = nested {
                let ident = path.get_ident().ok_or_else(malformed_err)?;

                vec.push(match &*ident.to_string() {
                    "TryFromString" => Impl::TryFromString,
                    "FromStr" => Impl::FromStr,
                    "AsRefStr" => Impl::AsRefStr,
                    "IntoString" => Impl::IntoString,
                    "Display" => Impl::Display,
                    "Serialize" => Impl::Serialize,
                    "Deserialize" => Impl::Deserialize,
                    _ => return Err(Error::new_spanned(&ident, "not an avaliable skip!")),
                });
            } else {
                return Err(malformed_err());
            }
        }

        Ok(Self(vec))
    }
    pub fn should_skip(&self, imp: Impl) -> bool {
        self.0.contains(&imp)
    }
}

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
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"format(style = "...")"#);

        let mut separator = None;
        let mut style = None;
        for nested in attr.nested {
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
        Ok(match &*style {
            "none" | "PascalCase" => FormatCase::None,
            "lower" => FormatCase::Lower,
            "UPPER" => FormatCase::Upper,
            "snake_case" => FormatCase::Snake,
            "SCREAMING_SNAKE_CASE" => FormatCase::ScreamingSnake,
            "kebab-case" => FormatCase::Kebab,
            "SCREAMING-KEBAB-CASE" => FormatCase::ScreamingKebab,
            "camelCase" => FormatCase::Camel,
            "camel_Snake_Case" => FormatCase::CamelSnake,
            "Pascal_Snake_Case" => FormatCase::PascalSnake,
            "Train-Case" => FormatCase::Train,
            "delimited" => {
                if let Some(sep) = separator {
                    FormatCase::Delimited { sep }
                } else {
                    return Err(no_sep_error());
                }
            }
            "delimitedlower" => {
                if let Some(sep) = separator {
                    FormatCase::DelimitedLower { sep }
                } else {
                    return Err(no_sep_error());
                }
            }
            "DELIMITEDUPPER" => {
                if let Some(sep) = separator {
                    FormatCase::DelimitedUpper { sep }
                } else {
                    return Err(no_sep_error());
                }
            }
            s => return Err(not_style_error(s)),
        })
    }

    pub fn format(&self, s: &str) -> String {
        match self {
            FormatCase::None => s.to_owned(),
            FormatCase::Lower => s.to_lowercase(),
            FormatCase::Upper => s.to_uppercase(),
            FormatCase::Camel => s[..1].to_lowercase() + &s[1..],
            FormatCase::Delimited { sep } => FormatCase::delimit(sep, s),
            FormatCase::DelimitedLower { sep } => FormatCase::delimit(sep, s).to_lowercase(),
            FormatCase::DelimitedUpper { sep } => FormatCase::delimit(sep, s).to_uppercase(),
            FormatCase::Train => FormatCase::delimit("-", s),
            FormatCase::PascalSnake => FormatCase::delimit("_", s),
            FormatCase::CamelSnake => {
                let s = FormatCase::delimit("_", s);

                s[..1].to_lowercase() + &s[1..]
            }
            FormatCase::Snake => FormatCase::delimit("_", s).to_lowercase(),
            FormatCase::Kebab => FormatCase::delimit("-", s).to_lowercase(),
            FormatCase::ScreamingSnake => FormatCase::delimit("_", s).to_uppercase(),
            FormatCase::ScreamingKebab => FormatCase::delimit("-", s).to_uppercase(),
        }
    }

    fn delimit(sep: &str, s: &str) -> String {
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

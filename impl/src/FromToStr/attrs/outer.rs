pub use super::super::Impl;
use syn::{spanned::Spanned, Error, Meta, MetaList, NestedMeta};

pub struct Attrs {
    pub format: Option<Format>,
    pub skip: Option<Skip>,
}

impl Attrs {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut format = None;
        let mut skip = None;

        for attr in attrs {
            try_get! {attr;
                format => Format,
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
                    _ => return Err(Error::new_spanned(ident, "not an avaliable skip!")),
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

pub struct Format(pub super::FormatCase);

impl Format {
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"format(style = "...")"#);

        Ok(Self(super::FormatCase::from_attr(&attr, malformed_err)?))
    }
    #[inline]
    pub fn format(&self, s: &str) -> String {
        self.0.format(s)
    }
}

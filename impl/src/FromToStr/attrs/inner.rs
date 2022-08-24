use syn::{spanned::Spanned, Lit, MetaList, NestedMeta};

use crate::utils::SpannedString;

pub struct Attrs {
    pub aliases: Option<Aliases>,
    pub rename: Option<Rename>,
}

impl Attrs {
    pub fn from_attrs(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut rename = None;
        let mut aliases = None;

        for attr in attrs {
            try_get! {attr;
                rename => Rename,
                aliases => Aliases
            }
        }
        Ok(Self { aliases, rename })
    }
}

pub enum Rename {
    Renamed(SpannedString),
    Format(super::FormatCase),
}

impl Rename {
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"rename("...") or rename(style = "...")"#);

        if let Ok(format) = super::FormatCase::from_attr(&attr, malformed_err) {
            return Ok(Self::Format(format));
        }
        let mut string = None;

        let mut nested = attr.nested.into_iter();
        if let Some(NestedMeta::Lit(Lit::Str(s))) = nested.next() {
            string = Some(s.into());
        }

        // if there's more than one thing inside the attr, error
        if nested.next().is_some() {
            return Err(malformed_err());
        }

        Ok(Self::Renamed(string.ok_or_else(malformed_err)?))
    }
}

pub struct Aliases(pub Vec<SpannedString>);

impl Aliases {
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"aliases("...",*)"#);

        let mut vec = Vec::with_capacity(attr.nested.len());
        for nested in attr.nested {
            if let NestedMeta::Lit(Lit::Str(s)) = nested {
                vec.push(s.into());
            } else {
                return Err(malformed_err());
            }
        }

        if vec.is_empty() {
            return Err(malformed_err());
        }
        Ok(Self(vec))
    }
}

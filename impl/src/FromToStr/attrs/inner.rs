use proc_macro2::Span;
use syn::{spanned::Spanned, Lit, MetaList, NestedMeta};

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

pub struct Rename(pub (String, Span));

impl Rename {
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"rename("...")"#);
        let mut string = None;

        let mut nested = attr.nested.into_iter();
        if let Some(NestedMeta::Lit(Lit::Str(s))) = nested.next() {
            string = Some((s.value(), s.span()));
        }

        // if there's more than one thing inside the attr, error
        if nested.next().is_some() {
            return Err(malformed_err());
        }

        Ok(Self(string.ok_or_else(malformed_err)?))
    }
}

pub struct Aliases(pub Vec<(String, Span)>);

impl Aliases {
    pub fn from_attr(attr: MetaList) -> syn::Result<Self> {
        let attr_span = attr.span();
        let malformed_err = malformed_err!(attr_span, r#"aliases("...",*)"#);

        let mut vec = Vec::with_capacity(attr.nested.len());
        for nested in attr.nested {
            if let NestedMeta::Lit(Lit::Str(s)) = nested {
                vec.push((s.value(), s.span()));
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

use proc_macro2::{Span, TokenStream};
use syn::{Attribute, Data, DataEnum, Error, Fields, LitStr};

#[inline]
pub fn check_if_fieldless_enum(name: &'static str, data: Data) -> syn::Result<DataEnum> {
    // if item not enum, error
    let dataenum = match data {
        Data::Enum(e) => e,
        Data::Struct(e) => {
            return Err(Error::new_spanned(
                e.struct_token,
                format!(
                    "#[derive({})] expected an enum, yet a struct was provided",
                    name
                ),
            ))
        }
        Data::Union(e) => {
            return Err(Error::new_spanned(
                e.union_token,
                format!(
                    "#[derive({})] expected an enum, yet an union was provided",
                    name
                ),
            ))
        }
    };

    for var in &dataenum.variants {
        // if variant is not fieldless, error
        match var.fields {
		Fields::Unit => (),
        _ => return Err(Error::new_spanned(
                var,
                format!("enum variant {} has data on it! #[derive({}) can only be run on fieldless enums", var.ident, name),
            )),
        }
    }

    Ok(dataenum)
}

#[inline]
pub fn check_enum_lt_variants(
    name: &'static str,
    variants: usize,
    data: &DataEnum,
) -> syn::Result<()> {
    if data.variants.len() >= variants {
        Ok(())
    } else {
        Err(Error::new_spanned(
            &data.variants,
            format!("#[derive({}) needs at least {} variants", name, variants),
        ))
    }
}

pub fn try_get_doc(attrname: &'static str, attrs: &[Attribute]) -> syn::Result<Option<String>> {
    let mut docstr: Option<String> = None;
    for attr in attrs {
        if attr.path.is_ident(attrname) {
            let doc = attr.parse_args::<LitStr>()?.value();
            match docstr {
                Some(ref mut str) => str.push_str(&doc),
                None => docstr = Some(doc),
            }
        }
    }
    Ok(docstr)
}

// Option::as_deref introduced in 1.40, msrv is 1.36
pub fn opt_as_deref<T: std::ops::Deref>(this: &Option<T>) -> Option<&T::Target> {
    match this.as_ref() {
        Some(t) => Some(&**t),
        None => None,
    }
}

#[derive(Clone)]
pub struct SpannedString {
    pub string: String,
    pub span: Span,
}
impl SpannedString {
    #[inline]
    pub fn new(string: String, span: Span) -> Self {
        Self { string, span }
    }
}
impl quote::ToTokens for SpannedString {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.string.to_tokens(tokens)
    }
}
impl From<syn::LitStr> for SpannedString {
    #[inline]
    fn from(litstr: syn::LitStr) -> Self {
        Self {
            string: litstr.value(),
            span: litstr.span(),
        }
    }
}
impl From<syn::Ident> for SpannedString {
    #[inline]
    fn from(ident: syn::Ident) -> Self {
        Self {
            string: ident.to_string(),
            span: ident.span(),
        }
    }
}

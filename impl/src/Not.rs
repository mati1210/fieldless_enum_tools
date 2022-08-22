use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, Error, Variant};

use crate::utils;

pub fn main(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let data = utils::check_if_fieldless_enum("Not", input.data)?;
    utils::check_enum_lt_variants("Not", 1, &data)?;

    let typ = input.ident;
    let fun = match data.variants.len() {
        // since there's only one variant, just return self again
        1 => {
            quote! {
                self
            }
        }

        2 => {
            let mut has_not_attr = false;
            for attr in data.variants.iter().flat_map(|v| &v.attrs) {
                if attr.path.is_ident("not") {
                    has_not_attr = true;
                    break;
                }
            }

            if has_not_attr {
                get_with_attrs(&typ, data.variants)?
            } else {
                let mut iter = data.variants.iter().map(|p| &p.ident);
                let first = iter.next().unwrap();
                let second = iter.next().unwrap();

                quote! {
                    match self {
                        #typ::#first => #typ::#second,
                        #typ::#second => #typ::#first
                    }
                }
            }
        }
        _ => get_with_attrs(&typ, data.variants)?,
    };

    Ok(quote! {
        impl ::core::ops::Not for #typ {
            type Output = Self;
            fn not(self) -> Self::Output {
                #fun
            }
        }
    })
}

pub fn get_with_attrs(
    typ: &Ident,
    vars: Punctuated<Variant, syn::token::Comma>,
) -> syn::Result<TokenStream> {
    let mut idents: Vec<(Ident, Ident)> = Vec::with_capacity(vars.len());
    // for every variant, check if theres a #[not] attribute, and if there is,
    // add both the variant ident and the ident in the #[not] on the vec
    for var in vars {
        let var_span = var.span();

        let mut attr_ident: Option<Ident> = None;
        for attr in var.attrs {
            if attr.path.is_ident("not") {
                attr_ident = Some(attr.parse_args()?);
                break;
            }
        }
        idents.push((
            var.ident,
            attr_ident
                .ok_or_else(|| Error::new(var_span, "variant doesn't have an #[not] attribute!"))?,
        ));
    }

    let one = idents.iter().map(|a| &a.0);
    let two = idents.iter().map(|a| &a.1);

    Ok(quote! {
        match self {
            #(#typ::#one => #typ::#two),*
        }
    })
}

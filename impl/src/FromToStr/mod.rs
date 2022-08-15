mod attrs;
use std::collections::HashMap;

use attrs::{inner::Attrs as InnerAttrs, outer::Attrs as OuterAttrs};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::Error;

pub fn main(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    let data = crate::utils::check_if_fieldless_enum("FromToStr", input.data)?;
    let outer_attr = OuterAttrs::from_attrs(&input.attrs)?;

    let mut fmtd = Vec::with_capacity(data.variants.len());
    for var in data.variants {
        let inner_attr = InnerAttrs::from_attrs(&var.attrs)?;

        fmtd.push(FormattedVariant::new(var.ident, inner_attr, &outer_attr));
    }
    check_if_duplicate(&fmtd)?;

    let typ = input.ident;
    let mut tree = Impl::default_impl(&typ, fmtd.as_slice());

    for imp in &Impl::default() {
        let imp = *imp;
        if !outer_attr.should_skip(imp) {
            tree.extend(imp.quote_impl(&typ));
        }
    }

    Ok(tree)
}

fn check_if_duplicate(fmtd: &[FormattedVariant]) -> syn::Result<()> {
    let mut hashes = HashMap::new();

    for (str, span1) in fmtd.iter().flat_map(FormattedVariant::iter) {
        // if there was already this string on the hashmap, error
        if let Some(span2) = hashes.insert(str, span1) {
            let mut error = Error::new(*span1, format!("duplicate value! both are [{}]", str));
            error.combine(Error::new(
                *span2,
                format!("duplicate value! both are [{}]", str),
            ));

            return Err(error);
        }
    }
    Ok(())
}

pub struct FormattedVariant {
    original: Ident,
    formatted: (String, Span),
    aliases: Vec<(String, Span)>,
}

impl FormattedVariant {
    pub fn new(ident: Ident, inner_attr: InnerAttrs, outer_attr: &OuterAttrs) -> Self {
        Self {
            formatted: {
                let span = ident.span();
                let ident = ident.to_string();
                if let Some(ren) = inner_attr.rename {
                    use attrs::inner::Rename;
                    match ren {
                        Rename::Renamed(ren) => ren,
                        Rename::Format(f) => (f.format(&ident), span),
                    }
                } else if let Some(ref f) = outer_attr.format {
                    (f.format(&ident), span)
                } else {
                    (ident, span)
                }
            },
            original: ident,
            aliases: inner_attr.aliases.map(|a| a.0).unwrap_or_default(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &(String, Span)> {
        self.aliases.iter().chain(std::iter::once(&self.formatted))
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Impl {
    AsRefStr,
    IntoString,
    Display,
    FromStr,
    TryFromString,
    Serialize,
    Deserialize,
}

impl Impl {
    pub const fn default() -> [Impl; 7] {
        [
            Impl::AsRefStr,
            Impl::IntoString,
            Impl::Display,
            Impl::FromStr,
            Impl::TryFromString,
            Impl::Serialize,
            Impl::Deserialize,
        ]
    }

    pub fn default_impl(typ: &Ident, formatted: &[FormattedVariant]) -> TokenStream {
        let mut as_str = TokenStream::new();
        let mut from_str = TokenStream::new();

        for f in formatted {
            let ident = &f.original;
            let formatted = &f.formatted.0;

            as_str.extend(quote! {
                #typ::#ident => #formatted,
            });

            let strings = f.iter().map(|s| &s.0);
            from_str.extend(quote! {
                #(#strings )|* => #typ::#ident,
            });
        }

        quote! {
            impl #typ {
                #[doc(hidden)]
                fn __as_str(&self) -> &'static str {
                    match self {
                        #as_str
                    }
                }
                #[doc(hidden)]
                fn __from_str(s: &str) -> Result<Self, ()> {
                    Ok(match s {
                        #from_str
                        _ => return Err(())
                    })
                }
            }
        }
    }

    pub fn quote_impl(self, typ: &Ident) -> TokenStream {
        match self {
            Impl::AsRefStr => quote! {
                impl ::core::convert::AsRef<str> for #typ {
                    #[inline]
                    fn as_ref(&self) -> &'static str {
                        self.__as_str()
                    }
                }
            },

            Impl::IntoString => {
                let string = quote! {::fieldless_enum_tools::__internal::String};
                quote! {
                    fieldless_enum_tools::if_alloc_enabled! {
                        impl ::core::convert::Into<#string> for #typ {
                            #[inline]
                            fn into(self) -> #string {
                                self.__as_str().to_owned()
                            }
                        }
                    }
                }
            }

            Impl::Display => quote! {
                impl ::core::fmt::Display for #typ {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.write_str(self.__as_str())
                    }
                }
            },

            Impl::FromStr => quote! {
                impl ::core::str::FromStr for #typ {
                    type Err = ();

                    #[inline]
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                       Self::__from_str(s)
                    }
                }
            },

            Impl::TryFromString => {
                let string = quote! {::fieldless_enum_tools::__internal::String};
                quote! {
                    ::fieldless_enum_tools::if_alloc_enabled! {
                        impl ::core::convert::TryFrom<#string> for #typ {
                            type Error = ();

                            #[inline]
                            fn try_from(s: #string) -> Result<Self, Self::Error> {
                                Self::__from_str(&s)
                            }
                        }
                    }
                }
            }

            Impl::Serialize => {
                let serde = quote! {::fieldless_enum_tools::__internal::serde};
                quote! {
                    ::fieldless_enum_tools::if_serde_enabled! {
                        impl #serde::Serialize for #typ {
                            fn serialize<S: #serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
                                self.__as_str().serialize(ser)
                            }
                        }
                    }
                }
            }

            Impl::Deserialize => {
                let visitor = quote::format_ident!("__{}Visitor", typ);
                let serde = quote! {::fieldless_enum_tools::__internal::serde};
                quote! {
                    ::fieldless_enum_tools::if_serde_enabled! {
                        impl<'de> #serde::Deserialize<'de> for #typ {
                            fn deserialize<D: #serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
                                struct #visitor;
                                impl<'de> #serde::de::Visitor<'de> for #visitor {
                                    type Value = #typ;

                                    fn expecting(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                                        f.write_str(concat!["an ", stringify!(#typ)])
                                    }

                                    fn visit_str<E: #serde::de::Error>(self, v: &str) -> Result<Self::Value, E>{
                                        <#typ>::__from_str(v).map_err(|_| E::invalid_value(#serde::de::Unexpected::Str(v), &self))
                                    }
                                }

                                de.deserialize_str(#visitor)
                            }
                        }
                    }
                }
            }
        }
    }
}

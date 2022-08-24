use crate::utils;

pub fn main(input: syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let variants = utils::check_if_fieldless_enum("All", input.data)?.variants;
    let doc = utils::try_get_doc("all_doc", &input.attrs)?;

    let typ = input.ident;
    let doc = utils::opt_as_deref(&doc).unwrap_or("Returns an array of all elements on this enum.");
    let len = variants.len();
    let idents = variants.into_iter().map(|p| p.ident);
    Ok(quote::quote! {
        impl #typ {
            #[doc = #doc]
            pub const fn all() -> [Self; #len] {
                [#(Self::#idents ,)*]
            }
        }
    })
}

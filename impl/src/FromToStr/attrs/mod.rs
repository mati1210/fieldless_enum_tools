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

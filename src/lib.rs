#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

/// Adds an function `all` to enum, returning an array with all variants of the enum
///
/// ```rust
/// # use fieldless_enum_tools::All;
/// #[derive(All, Debug, PartialEq, Eq)]
/// enum CoolEnum {
///   CoolVariantOne,
///   CoolVariantTwo
/// }
///
/// fn main() {
///   assert_eq!{
///     CoolEnum::all(),
///     [CoolEnum::CoolVariantOne, CoolEnum::CoolVariantTwo]
///   }
/// }
/// ```
///
/// # Attributes
/// `#[all_doc = r#"..."#)]` => sets the doc of the function. default being "Returns an array of all elements on this enum."
pub use fieldless_enum_tools_impl::All;

/// TODO
pub use fieldless_enum_tools_impl::Not;

/// Implements
/// [`AsRef<str>`],
/// [`Into<String>`][^alloc],
/// [`Display`] (therefore [`ToString`][^alloc]), [`FromStr`],
/// [`TryFrom<String>`][^alloc],
/// [`Serialize`][^serde] and [`Deserialize`][^serde] for enum.
///
///```
/// # use fieldless_enum_tools::FromToStr;
/// #[derive(FromToStr, Debug, PartialEq, Eq, Clone, Copy)]
/// #[fromtostr(format(style = "delimited", separator = "😎"))]
/// enum CoolEnum {
///     #[fromtostr(aliases("cool_variant_one"))]
///     CoolVariantOne,
///     #[fromtostr(rename("Very😎Cool😎Variant😎Two"))]
///     CoolVariantTwo
/// }
///
/// fn main() {
///     let cool = CoolEnum::CoolVariantOne;
///
///     assert_eq!(cool.as_ref(), "Cool😎Variant😎One");
///     assert_eq!("cool_variant_one".parse(), Ok(cool));
///     assert_eq!("Cool😎Variant😎One".parse(), Ok(cool));
///
///     assert_eq!("uncool variant :(".parse::<CoolEnum>(), Err(()));
///
///     let cool = CoolEnum::CoolVariantTwo;
///
///     assert_eq!("Cool😎Variant😎Two".parse::<CoolEnum>(), Err(()));
///     assert_eq!("Very😎Cool😎Variant😎Two".parse(), Ok(cool));
/// }
///```
///
/// # Attributes
/// ## Outer
///
/// `#[fromtostr(skip(...*))]`
/// Skips implementing specified traits
///
///   **Possible Values**
///
/// >| Value           | Skips               |
/// >|-----------------|---------------------|
/// >| `TryFromString` | [`TryFrom<String>`] |
/// >| `FromStr`       | [`FromStr`]         |
/// >| `AsRefStr`      | [`AsRef<str>`]      |
/// >| `IntoString`    | [`Into<String>`]    |
/// >| `Display`       | [`Display`]         |
/// >| `Serialize`     | [`Serialize`]       |
/// >| `Deserialize`   | [`Deserialize`]     |
///
/// ---
///
/// `#[fromtostr(format(style = "...", separator = "..."?))]`
/// Format variants using specified style
///
/// **Possible Values**
///
/// >| Style Name        | Description                                           | Example               | Note                                                    |
/// >|-------------------|-------------------------------------------------------|-----------------------|---------------------------------------------------------|
/// >| `none`            | keep it as is                                         | `TwoWords`            |                                                         |
/// >| `lower`           | to lowercase                                          | `twowords`            |                                                         |
/// >| `UPPER`           | to uppercase                                          | `TWOWORDS`            |                                                         |
/// >| `snake`           | to snake case                                         | `two_words`           | alias to `delimitedlower` style with a `_` separator    |
/// >| `kebab`           | to kebab case                                         | `two-words`           | alias to `delimitedlower` style with a `-` separator    |
/// >| `SCREAMING_SNAKE` | to screaming snake case                               | `TWO_WORDS`           | alias to `DELIMITEDUPPER` with a `_` separator          |
/// >| `SCREAMING-KEBAB` | to screaming kebab case                               | `TWO-WORDS`           | alias it to `DELIMITEDUPPER` style with a `-` separator |
/// >| `camel`           | to camel case                                         | `twoWords`            |                                                         |
/// >| `camel_Snake`     | to camel snake case                                   | `two_Words`           |                                                         |
/// >| `Pascal`          | to pascal case                                        | `TwoWords`            |                                                         |
/// >| `Pascal_Snake`    | to pascal snake case                                  | `Two_Words`           |                                                         |
/// >| `Train`           | to train case                                         | `Two-Words`           |                                                         |
/// >| `delimited`       | delimits every word with separator                    | `Two{separator}Words` | needs to specify a separator value                      |
/// >| `delimitedlower`  | delimits every word with separator, then to lowercase | `two{separator}words` | needs to specify a separator value                      |
/// >| `DELIMITEDUPPER`  | delimits every word with separator, then to uppercase | `TWO{SEPARATOR}WORDS` | needs to specify a separator value                      |
///
/// ## Inner
///
/// `#[fromtostr(aliases("..."*))]`
///
/// Specifies one (or more aliases) for this variant
///
/// ---
///
/// `#[fromtostr(rename("..."))]` or `#[fromtostr(rename(style = "...", separator = "..."?))]`
///
/// Renames this variant with specified string or specified format style

///
/// [^alloc]: if crate feature `std` or `alloc` avaliable.
///
/// [^serde]: if crate feature `serde` avaliable.
///
/// [`Display`]: `core::fmt::Display`
/// [`TryFrom<String>`]: `core::convert::TryFrom`
/// [`FromStr`]: `core::str::FromStr`
/// [`Serialize`]: https://serde.rs/
/// [`Deserialize`]: https://serde.rs/
pub use fieldless_enum_tools_impl::FromToStr;

#[doc(hidden)]
pub mod __internal {
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(not(feature = "serde"))]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! if_serde_enabled {
        ($($t:tt)*) => {};
    }

    #[cfg(feature = "serde")]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! if_serde_enabled {
        ($($t:tt)*) => { $($t)* };
    }

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! if_alloc_enabled {
        ($($t:tt)*) => {};
    }

    #[cfg(any(feature = "alloc", feature = "std"))]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! if_alloc_enabled {
        ($($t:tt)*) => { $($t)* };
    }

    #[cfg(not(feature = "std"))]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! if_std_enabled {
        ($($t:tt)*) => {};
    }

    #[cfg(feature = "std")]
    #[macro_export]
    #[doc(hidden)]
    macro_rules! if_std_enabled {
        ($($t:tt)*) => { $($t)* };
    }
}

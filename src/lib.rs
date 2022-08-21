#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

/// Adds an function `all` to enum, returning an array with all variants of the enum
///
/// ```rust
/// use fieldless_enum_tools::All;
///
/// #[derive(All, Debug, PartialEq, Eq)]
/// enum CoolEnum {
///     CoolVariantOne,
///     CoolVariantTwo
/// }
///
/// assert_eq!(
///     CoolEnum::all(),
///     [CoolEnum::CoolVariantOne, CoolEnum::CoolVariantTwo]
/// );
/// ```
///
/// # Attributes
///
/// ## Outer attributes
///
/// `#[all_doc = r#"..."#)]`
///
/// Sets the doc of the function, with the default being "Returns an array of all elements on this enum."
pub use fieldless_enum_tools_impl::All;

/// Implements [`Not`] for enum.
///
/// ```rust
/// use fieldless_enum_tools::Not;
///
/// // works without any attributes on enum with up to two variants
/// #[derive(Not, Debug, PartialEq, Eq)]
/// enum One {
///     One
/// }
///
/// assert_eq!(!One::One, One::One);
///
/// #[derive(Not, Debug, PartialEq, Eq)]
/// enum More {
///     One,
///     Two
/// }
///
/// assert_eq!(!More::One, More::Two);
/// assert_eq!(!More::Two, More::One);
///
/// // requires attributes after
/// #[derive(Not, Debug, PartialEq, Eq)]
/// enum Multiple {
///     #[not(OppositeOfOne)]
///     One,
///     #[not(OppositeOfTwo)]
///     Two,
///     #[not(One)]
///     OppositeOfOne,
///     #[not(Two)]
///     OppositeOfTwo,
///     #[not(Trap)]
///     Trap
/// }
///
/// assert_eq!(!Multiple::One, Multiple::OppositeOfOne);
/// assert_eq!(!Multiple::OppositeOfTwo, Multiple::Two);
/// assert_eq!(!Multiple::Trap, Multiple::Trap);
/// ```
///
/// # Attributes
///
/// ## Variant attributes
///
/// `#[not(...)]`
///
/// Specifies which variant this variant will return when [`Not`]'ed
///
/// [`Not`]: `core::ops::Not`
pub use fieldless_enum_tools_impl::Not;

/// Implements
/// [`AsRef<str>`],
/// [`Into<String>`][^alloc],
/// [`Display`] (therefore [`ToString`][^alloc]), [`FromStr`],
/// [`TryFrom<String>`][^alloc],
/// [`Serialize`][^serde] and [`Deserialize`][^serde] for enum.
///
///```rust
/// use fieldless_enum_tools::FromToStr;
///
/// #[derive(FromToStr, Debug, PartialEq, Eq, Clone, Copy)]
/// #[fromtostr(format(style = "delimited", separator = "😎"))]
/// enum CoolEnum {
///     #[fromtostr(aliases("cool_variant_one"))]
///     CoolVariantOne,
///     #[fromtostr(rename("Very😎Cool😎Variant😎Two"))]
///     CoolVariantTwo
/// }
///
///
/// let cool = CoolEnum::CoolVariantOne;
///
/// assert_eq!(cool.as_ref(), "Cool😎Variant😎One");
/// assert_eq!("cool_variant_one".parse(), Ok(cool));
/// assert_eq!("Cool😎Variant😎One".parse(), Ok(cool));
///
/// let cool = CoolEnum::CoolVariantTwo;
/// assert_eq!("Very😎Cool😎Variant😎Two".parse(), Ok(cool));
///
/// assert_eq!("uncool variant :(".parse::<CoolEnum>(), Err(()));
/// // errors because we renamed it to Very😎Cool😎Variant😎Two
/// assert_eq!("Cool😎Variant😎Two".parse::<CoolEnum>(), Err(()));
///```
///
/// # Attributes
/// ## Outer attributes
///
/// `#[fromtostr(skip(...*))]`
///
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
///
/// Format variants using specified [style](Self#possible-styles)
///
///
/// ## Variant attributes
///
/// `#[fromtostr(aliases("..."*))]`
///
/// Specifies one (or more aliases) for this variant
///
/// ---
///
/// `#[fromtostr(rename("..."))]` or `#[fromtostr(rename(style = "...", separator = "..."?))]`
///
/// Renames this variant with specified string or specified [format style](Self#possible-styles)
///
/// # Possible Styles
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

#[cfg(not(doc))]
pub mod __internal {
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::String;
    #[cfg(feature = "std")]
    pub use std::string::String;

    #[cfg(feature = "serde")]
    pub use serde;

    #[cfg(not(feature = "serde"))]
    #[macro_export]
    macro_rules! if_serde_enabled {
        ($($t:tt)*) => {};
    }

    #[cfg(feature = "serde")]
    #[macro_export]
    macro_rules! if_serde_enabled {
        ($($t:tt)*) => { $($t)* };
    }

    #[cfg(not(any(feature = "alloc", feature = "std")))]
    #[macro_export]
    macro_rules! if_alloc_enabled {
        ($($t:tt)*) => {};
    }

    #[cfg(any(feature = "alloc", feature = "std"))]
    #[macro_export]
    macro_rules! if_alloc_enabled {
        ($($t:tt)*) => { $($t)* };
    }

    #[cfg(not(feature = "std"))]
    #[macro_export]
    macro_rules! if_std_enabled {
        ($($t:tt)*) => {};
    }

    #[cfg(feature = "std")]
    #[macro_export]
    macro_rules! if_std_enabled {
        ($($t:tt)*) => { $($t)* };
    }
}

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use fieldless_enum_tools_impl::*;

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

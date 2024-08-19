pub mod error;
pub mod inference;

pub mod prelude {
    pub use crate::inference::*;

    /// Shorthand function to use for `..default()`
    pub fn default<D: Default>() -> D {
        <D as Default>::default()
    }

    #[cfg(feature = "ril")]
    pub extern crate ril;

    #[cfg(feature = "image")]
    pub extern crate image;
}

#[cfg(all(feature = "image", feature = "ril"))]
compile_error!("Both ril and image enabled at once");

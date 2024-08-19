pub mod error;
pub mod inference;

pub mod prelude {
    pub use crate::inference::*;

    /// Shorthand function to use for `..default()`
    pub fn default<D: Default>() -> D {
        <D as Default>::default()
    }
}
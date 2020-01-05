#[macro_use]
pub mod mutate_return;
#[macro_use]
pub mod remember_cond;
pub mod prelude {
    pub use crate::mutate_return::*;
    pub use crate::remember_cond::*;
}

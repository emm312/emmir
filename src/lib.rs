pub mod ir;
pub mod values;
pub mod interpreter;
pub mod builder;

pub mod prelude {
    pub use crate::ir;
    pub use crate::interpreter::Runner;
    pub use crate::values::Value;
    pub use crate::builder::*;
}
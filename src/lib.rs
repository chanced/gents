mod error;
pub mod schema;
pub use error::*;
pub use schema::{Schema, SchemaOrSchemas};

macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}
pub(crate) use tri;

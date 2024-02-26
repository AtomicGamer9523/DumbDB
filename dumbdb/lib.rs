#[cfg(all(
    not(feature = "embed"),
    not(feature = "standalone"),
))]
compile_error!("At least one of the features 'embed' or 'standalone' must be enabled");

pub use ::ddb::*;

#[cfg(feature = "standalone")]
pub use bin;

#[doc(hidden)]
#[cfg(feature = "embed-python")]
#[path = "__bindings__/python.rs"]
pub mod python_embed;
#[doc(hidden)]
#[cfg(feature = "embed-python")]
pub use python_embed::dumbdb;

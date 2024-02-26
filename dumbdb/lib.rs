#[cfg(all(
    not(feature = "embed"),
    not(feature = "standalone"),
))]
compile_error!("At least one of the features 'embed' or 'standalone' must be enabled");

pub use ::ddb::*;

#[cfg(feature = "standalone")]
pub use bin;

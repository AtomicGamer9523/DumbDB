use ::base::{DumbDBBase, Result};

#[repr(transparent)]
pub struct DumbDB<S>(DumbDBBase<S>);

impl<S> DumbDB<S> {
    pub fn new<Name: ToString>(name: Name) -> Result<Self> {
        Ok(Self(DumbDBBase::new(name)?))
    }
}

use pyo3::*;

impl crate::DumbValue for Py<PyAny> {
    fn serialize(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
    fn deserialize(vec: &Vec<u8>) -> crate::DumbResult<Self> {
        Python::with_gil(|py| {
            let s = String::from_utf8(vec.clone()).map_err(|_|
                crate::DumbError::ParsingError("invalid utf-8")
            )?;
            Ok(s.into_py(py))
        })
    }
}

type IoErr = exceptions::PyIOError;

impl IntoPy<Py<PyAny>> for crate::DumbError {
    #[inline]
    fn into_py(self, py: Python) -> Py<PyAny> {
        self.to_string().into_py(py)
    }
}

impl From<crate::DumbError> for PyErr {
    #[inline]
    fn from(e: crate::DumbError) -> Self {
        IoErr::new_err(e)
    }
}

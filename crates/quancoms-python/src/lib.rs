use pyo3::{exceptions::PyValueError, prelude::*};
use quancoms_core::qubit::QuantumRegister as CoreRegister;

#[pyclass]
struct QuantumRegister {
    inner: CoreRegister,
}

#[pymethods]
impl QuantumRegister {
    #[new]
    fn new(n: usize) -> PyResult<Self> {
        match CoreRegister::new(n) {
            Ok(core) => Ok(QuantumRegister { inner: core }),
            Err(err_msg) => Err(PyValueError::new_err(err_msg)),
        }
    }

    fn h(mut slf: PyRefMut<'_, Self>, target: usize) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .H(target)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;

        Ok(slf)
    }
}

#[pymodule]
fn quancoms(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<QuantumRegister>()?;

    Ok(())
}

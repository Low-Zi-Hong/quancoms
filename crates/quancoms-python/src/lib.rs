use pyo3::{exceptions::PyValueError, prelude::*, types::PyComplex};
use quancoms_core::complex::Complex;
use quancoms_core::qubit::{DiracKet, QuantumRegister as CoreRegister};

#[pyclass]
struct QuantumRegister {
    inner: CoreRegister,
}

#[pyclass]
pub struct PyDiracKet {
    pub inner: DiracKet,
}

#[pymethods]
impl PyDiracKet {
    fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
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

    fn observe(mut slf: PyRefMut<'_, Self>) -> PyResult<PyDiracKet> {
        let result = slf
            .inner
            .observe()
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(PyDiracKet { inner: result })
    }

    fn observe_one(mut slf: PyRefMut<'_, Self>, target: usize) -> PyResult<PyDiracKet> {
        let result = slf
            .inner
            .observe_one(target)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(PyDiracKet { inner: result })
    }

    fn observe_mul(mut slf: PyRefMut<'_, Self>, targets: Vec<usize>) -> PyResult<PyDiracKet> {
        let result = slf
            .inner
            .observe_mul(targets)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(PyDiracKet { inner: result })
    }

    fn god_observe(mut slf: PyRefMut<'_, Self>) -> PyResult<Vec<f64>> {
        let result = slf
            .inner
            .god_observe()
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(result)
    }

    fn x(mut slf: PyRefMut<'_, Self>, target: usize) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .X(target)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn h(mut slf: PyRefMut<'_, Self>, target: usize) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .H(target)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn u<'py>(
        mut slf: PyRefMut<'py, Self>,
        target: usize,
        u00: &Bound<'py, PyComplex>,
        u01: &Bound<'py, PyComplex>,
        u10: &Bound<'py, PyComplex>,
        u11: &Bound<'py, PyComplex>,
    ) -> PyResult<PyRefMut<'py, Self>> {
        let complex = |p: &Bound<'_, PyComplex>| -> Complex {
            Complex {
                re: p.real(),
                im: p.imag(),
            }
        };

        slf.inner
            .U(
                target,
                complex(u00),
                complex(u01),
                complex(u10),
                complex(u11),
            )
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn cnot(
        mut slf: PyRefMut<'_, Self>,
        control: usize,
        target: usize,
    ) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .CNOT(control, target)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn ccnot(
        mut slf: PyRefMut<'_, Self>,
        control1: usize,
        control2: usize,
        target: usize,
    ) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .CCNOT(control1, control2, target)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn rz(mut slf: PyRefMut<'_, Self>, target: usize, theta: f64) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .Rz(target, theta)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn rx(mut slf: PyRefMut<'_, Self>, target: usize, theta: f64) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .Rx(target, theta)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn ry(mut slf: PyRefMut<'_, Self>, target: usize, theta: f64) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .Ry(target, theta)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn swap(mut slf: PyRefMut<'_, Self>, q1: usize, q2: usize) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .SWAP(q1, q2)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn csswap(
        mut slf: PyRefMut<'_, Self>,
        control: usize,
        target_1: usize,
        target_2: usize,
    ) -> PyResult<PyRefMut<'_, Self>> {
        slf.inner
            .CSSWAP(control, target_1, target_2)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }

    fn mcu<'py>(
        mut slf: PyRefMut<'py, Self>,
        control: Vec<usize>,
        target: usize,
        u00: &Bound<'py, PyComplex>,
        u01: &Bound<'py, PyComplex>,
        u10: &Bound<'py, PyComplex>,
        u11: &Bound<'py, PyComplex>,
    ) -> PyResult<PyRefMut<'py, Self>> {
        let complex = |p: &Bound<'_, PyComplex>| -> Complex {
            Complex {
                re: p.real(),
                im: p.imag(),
            }
        };

        slf.inner
            .MCU(
                control,
                target,
                complex(u00),
                complex(u01),
                complex(u10),
                complex(u11),
            )
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        Ok(slf)
    }
}

#[pymodule]
fn quancoms(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<QuantumRegister>()?;
    m.add_class::<PyDiracKet>()?;

    Ok(())
}

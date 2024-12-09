mod draw;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "cd_diagram")]
fn py_module(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(draw::cd_diagram, m)?)?;
    Ok(())
}
// Licensed under the MIT license.

use pyo3::prelude::*;

#[pyfunction]
fn hello_n_times(times: u32) {
    (0..times).for_each(|_| println!("hello!"))
}

/// Not a real module.  I mean, it is, but it doesn't do anything useful, so it isn't.
#[pymodule]
fn angry_dragon_fire(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_n_times, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {}

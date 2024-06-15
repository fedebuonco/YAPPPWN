mod constants;
mod exploit;
mod macaddress;
use crate::exploit::build_fake_ifnet;
use macaddress::MacAddress64;
use pcap::Device;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn py_build_fake_ifnet(pppoe_softc: u64) -> PyResult<Vec<u8>> {
    Ok(build_fake_ifnet(pppoe_softc))
}

#[pymodule]
fn py_yapppwn(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_build_fake_ifnet, m)?)?;
    Ok(())
}

use crate::parser::proto::simple_proto::STxIn;
pub use bitcoin_explorer::*;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub trait ToPy {
    /// Converts self into a Python object.
    fn to_py(&self, py: Python) -> PyResult<PyObject>;
}


impl ToPy for SBlockHeader {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("block_hash", self.block_hash.to_string())?;
        output.set_item("time", self.time)?;
        Ok(output.to_object(py))
    }
}

impl ToPy for STxIn {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("txid", self.txid.to_string())?;
        output.set_item("vout", self.vout)?;
        Ok(output.to_object(py))
    }
}

impl ToPy for STxOut {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("value", self.value)?;
        let addresses: Vec<String> = self.addresses.iter().map(|a| a.to_string()).collect();
        output.set_item("addresses", addresses)?;
        Ok(output.to_object(py))
    }
}

impl ToPy for STransaction {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("txid", self.txid.to_string())?;
        let mut txin: Vec<PyObject> = Vec::with_capacity(self.input.len());
        for i in &self.input {
            txin.push(i.to_py(py)?);
        }
        output.set_item("input", PyList::new(py, txin))?;
        let mut txout: Vec<PyObject> = Vec::with_capacity(self.output.len());
        for o in &self.output {
            txout.push(o.to_py(py)?);
        }
        output.set_item("output", PyList::new(py, txout))?;
        Ok(output.to_object(py))
    }
}

impl ToPy for SBlock {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("header", self.header.to_py(py)?)?;
        let mut txdata: Vec<PyObject> = Vec::new();
        for tx in &self.txdata {
            txdata.push(tx.to_py(py)?);
        }
        output.set_item("txdata", PyList::new(py, txdata))?;
        Ok(output.to_object(py))
    }
}

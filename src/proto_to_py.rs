use crate::parser::proto::simple_proto::STxIn;
pub use bitcoin_explorer::*;
use bitcoin::{TxIn, OutPoint};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use crate::{derive_block_to_py, derive_outpoint_to_py, derive_stx_to_py, derive_ftx_to_py};
use bitcoin_explorer::parser::script::ScriptInfo;
use pyo3::{PyGCProtocol, PyTraverseError, PyVisit};

pub trait ToPy {
    /// Converts self into a Python object.
    fn to_py(&self, py: Python) -> PyResult<PyObject>;
}

derive_block_to_py!(SBlock);
derive_block_to_py!(FBlock);
derive_block_to_py!(SConnectedBlock);
derive_block_to_py!(FConnectedBlock);
derive_outpoint_to_py!(OutPoint);
derive_outpoint_to_py!(STxIn);
derive_stx_to_py!(STransaction);
derive_stx_to_py!(SConnectedTransaction);
derive_ftx_to_py!(FTransaction);
derive_ftx_to_py!(FConnectedTransaction);

#[pyclass(gc)]
pub struct SBlockPy {
    #[pyo3(get)]
    pub header: Py<SBlockHeaderPy>,
    #[pyo3(get)]
    pub txdata: Py<PyTuple>,
}

impl SBlockPy {
    pub(crate) fn from_rust(py: Python, blk: SBlock) -> PyResult<Self> {
        let header = Py::new(py, SBlockHeaderPy::from_rust(blk.header))?;
        let mut txdata = Vec::new();
        for tx in blk.txdata {
            txdata.push(Py::new(py, STransactionPy::from_rust(py, tx)?)?);
        }
        Ok(SBlockPy {
            header,
            txdata: Py::from(PyTuple::new(py, txdata)),
        })
    }
}

#[pyproto]
impl PyGCProtocol for SBlockPy {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        visit.call(&self.header)?;
        visit.call(&self.txdata)?;
        Ok(())
    }

    fn __clear__(&mut self) {}
}

#[pyclass]
pub struct SBlockHeaderPy {
    #[pyo3(get)]
    pub block_hash: String,
    #[pyo3(get)]
    pub time: u32,
}

impl SBlockHeaderPy {
    fn from_rust(header: SBlockHeader) -> Self {
        SBlockHeaderPy {
            block_hash: header.block_hash.to_string(),
            time: header.time,
        }
    }
}

#[pyclass(gc)]
pub struct STransactionPy {
    #[pyo3(get)]
    pub txid: String,
    /// List of inputs
    #[pyo3(get)]
    pub input: Py<PyTuple>,
    /// List of outputs
    #[pyo3(get)]
    pub output: Py<PyTuple>,
}

impl STransactionPy {
    fn from_rust(py: Python, tx: STransaction) -> PyResult<Self> {
        let mut input = Vec::with_capacity(tx.input.len());
        for x in tx.input {
            input.push(Py::new(py, STxInPy::from_rust(x))?);
        }
        let mut output = Vec::with_capacity(tx.output.len());
        for x in tx.output {
            output.push(Py::new(py, STxOutPy::from_rust(py, x))?);
        }
        Ok(STransactionPy {
            txid: tx.txid.to_string(),
            input: Py::from(PyTuple::new(py, input)),
            output: Py::from(PyTuple::new(py, output)),
        })
    }
}

#[pyproto]
impl PyGCProtocol for STransactionPy {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        visit.call(&self.input)?;
        visit.call(&self.output)?;
        Ok(())
    }

    fn __clear__(&mut self) {}
}

#[pyclass]
pub struct STxInPy {
    #[pyo3(get)]
    pub txid: String,
    #[pyo3(get)]
    pub vout: u32,
}

impl STxInPy {
    fn from_rust(tx: STxIn) -> Self {
        STxInPy {
            txid: tx.txid.to_string(),
            vout: tx.vout,
        }
    }
}

#[pyclass(gc)]
pub struct STxOutPy {
    #[pyo3(get)]
    pub value: u64,
    #[pyo3(get)]
    pub addresses: Py<PyTuple>,
}

impl STxOutPy {
    fn from_rust(py: Python, tx: STxOut) -> Self {
        let addresses: Vec<String> = tx.addresses.iter().map(|x| x.to_string()).collect();
        STxOutPy {
            value: tx.value,
            addresses: Py::from(PyTuple::new(py, addresses)),
        }
    }
}

#[pyproto]
impl PyGCProtocol for STxOutPy {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        visit.call(&self.addresses)?;
        Ok(())
    }

    fn __clear__(&mut self) {}
}

impl ToPy for SBlockHeader {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("block_hash", self.block_hash.to_string())?;
        output.set_item("time", self.time)?;
        Ok(output.to_object(py))
    }
}

impl ToPy for STxOut {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("value", self.value)?;
        let addresses: Vec<String> = self.addresses.iter().map(|a| a.to_string()).collect();
        output.set_item("addresses", PyTuple::new(py, addresses))?;
        Ok(output.to_object(py))
    }
}

impl ToPy for FBlockHeader {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("version", self.version)?;
        output.set_item("block_hash", self.block_hash.to_string())?;
        output.set_item("prev_blockhash", self.prev_blockhash.to_string())?;
        output.set_item("merkle_root", self.merkle_root.to_string())?;
        output.set_item("time", self.time)?;
        output.set_item("bits", self.bits)?;
        output.set_item("nonce", self.nonce)?;
        Ok(output.to_object(py))
    }
}

impl ToPy for TxIn {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("previous_output", self.previous_output.to_py(py)?)?;
        output.set_item("script_sig", self.script_sig.to_hex())?;
        output.set_item("sequence", self.sequence)?;
        let witness: Vec<String> = self.witness.iter().map(|w| w.to_hex()).collect();
        output.set_item("witness", PyTuple::new(py, witness))?;
        Ok(output.to_object(py))
    }
}

impl ToPy for FTxOut {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("value", self.value)?;
        output.set_item("script_pubkey", self.script_pubkey.to_hex())?;
        output.set_item("script_type", self.script_type.to_string())?;
        let addresses: Vec<String> = self.addresses.iter().map(|a| a.to_string()).collect();
        output.set_item("addresses", PyTuple::new(py, addresses))?;
        Ok(output.to_object(py))
    }
}

impl ToPy for ScriptInfo {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        let addresses: Vec<String> = self.addresses.iter().map(|a| a.to_string()).collect();
        output.set_item("addresses", PyTuple::new(py, addresses))?;
        output.set_item("pattern", self.pattern.to_string())?;
        Ok(output.to_object(py))
    }
}

impl ToPy for BlockHeader {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("version", self.version)?;
        output.set_item("prev_blockhash", self.prev_blockhash.to_string())?;
        output.set_item("merkle_root", self.merkle_root.to_string())?;
        output.set_item("time", self.time)?;
        output.set_item("bits", self.bits)?;
        output.set_item("nonce", self.nonce)?;
        Ok(output.to_object(py))
    }
}

impl ToPy for BlockIndexRecord {
    fn to_py(&self, py: Python) -> PyResult<PyObject> {
        let output = PyDict::new(py);
        output.set_item("n_version", self.n_version)?;
        output.set_item("n_height", self.n_height)?;
        output.set_item("n_status", self.n_status)?;
        output.set_item("n_tx", self.n_tx)?;
        output.set_item("n_file", self.n_file)?;
        output.set_item("n_data_pos", self.n_data_pos)?;
        output.set_item("n_undo_pos", self.n_undo_pos)?;
        output.set_item("block_header", self.block_header.to_py(py)?)?;
        Ok(output.to_object(py))
    }
}

#[macro_export]
macro_rules! derive_block_to_py {
    ($block_type:ident) => {
        impl ToPy for $block_type {
            fn to_py(&self, py: Python) -> PyResult<PyObject> {
                let output = PyDict::new(py);
                output.set_item("header", self.header.to_py(py)?)?;
                let mut txdata: Vec<PyObject> = Vec::new();
                for tx in &self.txdata {
                    txdata.push(tx.to_py(py)?);
                }
                output.set_item("txdata", PyTuple::new(py, txdata))?;
                Ok(output.to_object(py))
            }
        }
    }
}

#[macro_export]
macro_rules! derive_outpoint_to_py {
    ($outpoint_type:ident) => {
        impl ToPy for $outpoint_type {
            fn to_py(&self, py: Python) -> PyResult<PyObject> {
                let output = PyDict::new(py);
                output.set_item("txid", self.txid.to_string())?;
                output.set_item("vout", self.vout)?;
                Ok(output.to_object(py))
            }
        }
    }
}

#[macro_export]
macro_rules! derive_stx_to_py {
    ($s_tx_type:ident) => {
        impl ToPy for $s_tx_type {
            fn to_py(&self, py: Python) -> PyResult<PyObject> {
                let output = PyDict::new(py);
                output.set_item("txid", self.txid.to_string())?;
                let mut txin: Vec<PyObject> = Vec::with_capacity(self.input.len());
                for i in &self.input {
                    txin.push(i.to_py(py)?);
                }
                output.set_item("input", PyTuple::new(py, txin))?;
                let mut txout: Vec<PyObject> = Vec::with_capacity(self.output.len());
                for o in &self.output {
                    txout.push(o.to_py(py)?);
                }
                output.set_item("output", PyTuple::new(py, txout))?;
                Ok(output.to_object(py))
            }
        }
    }
}

#[macro_export]
macro_rules! derive_ftx_to_py {
    ($f_tx_type:ident) => {
        impl ToPy for $f_tx_type {
            fn to_py(&self, py: Python) -> PyResult<PyObject> {
                let output = PyDict::new(py);
                output.set_item("version", self.version)?;
                output.set_item("lock_time", self.lock_time)?;
                output.set_item("txid", self.txid.to_string())?;
                let mut txin: Vec<PyObject> = Vec::with_capacity(self.input.len());
                for i in &self.input {
                    txin.push(i.to_py(py)?);
                }
                output.set_item("input", PyTuple::new(py, txin))?;
                let mut txout: Vec<PyObject> = Vec::with_capacity(self.output.len());
                for o in &self.output {
                    txout.push(o.to_py(py)?);
                }
                output.set_item("output", PyTuple::new(py, txout))?;
                Ok(output.to_object(py))
            }
        }
    }
}

use crate::parser::proto::simple_proto::STxIn;
pub use bitcoin_explorer::*;
use bitcoin::{TxIn, OutPoint};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use crate::{derive_block_to_py, derive_outpoint_to_py, derive_stx_to_py, derive_ftx_to_py};

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
        output.set_item("addresses", addresses)?;
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
        output.set_item("witness", witness)?;
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
        output.set_item("addresses", addresses)?;
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
                output.set_item("txdata", PyList::new(py, txdata))?;
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
                output.set_item("input", PyList::new(py, txin))?;
                let mut txout: Vec<PyObject> = Vec::with_capacity(self.output.len());
                for o in &self.output {
                    txout.push(o.to_py(py)?);
                }
                output.set_item("output", PyList::new(py, txout))?;
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
                output.set_item("input", PyList::new(py, txin))?;
                let mut txout: Vec<PyObject> = Vec::with_capacity(self.output.len());
                for o in &self.output {
                    txout.push(o.to_py(py)?);
                }
                output.set_item("output", PyList::new(py, txout))?;
                Ok(output.to_object(py))
            }
        }
    }
}

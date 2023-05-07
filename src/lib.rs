#[macro_use]
mod proto_to_py;

#[doc(inline)]
use bitcoin_explorer::*;
use proto_to_py::*;
use pyo3::prelude::*;
use pyo3::Python;
use std::ops::Deref;
use std::path::Path;

#[pyclass(name = "BitcoinDB")]
struct BitcoinDBPy(BitcoinDB);

impl Deref for BitcoinDBPy {
    type Target = BitcoinDB;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[pymethods]
impl BitcoinDBPy {
    #[new]
    fn new(path: &str, tx_index: bool) -> PyResult<Self> {
        let path = Path::new(path);
        match BitcoinDB::new(path, tx_index) {
            Ok(db) => Ok(BitcoinDBPy(db)),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_full(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_block::<FBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_simple(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_block::<SBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_full_connected(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_connected_block::<FConnectedBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_simple_connected(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_connected_block::<SConnectedBlock>(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_block_header(&self, height: usize, py: Python) -> PyResult<PyObject> {
        match self.get_header(height) {
            Ok(block) => block.to_py(py),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, height, /)")]
    fn get_hash_from_height(&self, height: usize) -> PyResult<String> {
        match self.0.get_hash_from_height(height) {
            Ok(b) => Ok(b.to_hex()),
            Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
        }
    }

    #[pyo3(text_signature = "($self, hash, /)")]
    fn get_height_from_hash(&self, hash: String) -> PyResult<usize> {
        if let Ok(blk_hash) = BlockHash::from_hex(&hash) {
            match self.0.get_height_from_hash(&blk_hash) {
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
                Ok(h) => Ok(h),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_height_from_txid(&self, txid: String) -> PyResult<usize> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_height_of_transaction(&txid) {
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
                Ok(h) => Ok(h),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_transaction_full(&self, txid: String, py: Python) -> PyResult<PyObject> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_transaction::<FTransaction>(&txid) {
                Ok(t) => t.to_py(py),
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_transaction_simple(&self, txid: String, py: Python) -> PyResult<PyObject> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_transaction::<STransaction>(&txid) {
                Ok(t) => t.to_py(py),
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_transaction_full_connected(&self, txid: String, py: Python) -> PyResult<PyObject> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_connected_transaction::<FConnectedTransaction>(&txid) {
                Ok(t) => t.to_py(py),
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, txid, /)")]
    fn get_transaction_simple_connected(&self, txid: String, py: Python) -> PyResult<PyObject> {
        if let Ok(txid) = Txid::from_hex(&txid) {
            match self.get_connected_transaction::<SConnectedTransaction>(&txid) {
                Ok(t) => t.to_py(py),
                Err(e) => Err(pyo3::exceptions::PyException::new_err(e.to_string())),
            }
        } else {
            Err(pyo3::exceptions::PyException::new_err(
                "invalid txid format",
            ))
        }
    }

    #[pyo3(text_signature = "($self, stop, /)")]
    fn iter_block_full_arr(&self, heights: Vec<usize>) -> PyResult<FBlockIterArr> {
        Ok(FBlockIterArr::new(&self, heights))
    }

    #[pyo3(text_signature = "($self, stop, /)")]
    fn iter_block_simple_arr(&self, heights: Vec<usize>) -> PyResult<SBlockIterArr> {
        Ok(SBlockIterArr::new(&self, heights))
    }

    #[pyo3(text_signature = "($self, start, stop, /)")]
    fn iter_block_full_seq(&self, start: usize, stop: usize) -> PyResult<FBlockIter> {
        Ok(FBlockIter::new(&self, start, stop))
    }

    #[pyo3(text_signature = "($self, start, stop, /)")]
    fn iter_block_simple_seq(&self, start: usize, stop: usize) -> PyResult<SBlockIter> {
        Ok(SBlockIter::new(&self, start, stop))
    }

    #[pyo3(text_signature = "($self, stop, /)")]
    fn iter_block_full_connected(&self, stop: usize) -> PyResult<FConnBlockIter> {
        Ok(FConnBlockIter::new(&self, stop))
    }

    #[pyo3(text_signature = "($self, stop, /)")]
    fn iter_block_simple_connected(&self, stop: usize) -> PyResult<SConnBlockIter> {
        Ok(SConnBlockIter::new(&self, stop))
    }

    #[pyo3(text_signature = "($self, /)")]
    fn get_max_height(&self) -> usize {
        self.0.get_block_count()
    }

    #[pyo3(text_signature = "($self, /)")]
    fn get_block_count(&self) -> usize {
        self.0.get_block_count()
    }

    #[staticmethod]
    #[pyo3(text_signature = "($self, script_pub_key, /)")]
    fn parse_script(script_pub_key: String, py: Python) -> PyResult<PyObject> {
        let script = get_addresses_from_script(&script_pub_key);
        match script {
            Ok(script) => script.to_py(py),
            Err(_) => Err(pyo3::exceptions::PyException::new_err(
                "failed to parse script_pub_key",
            )),
        }
    }
}

// construct python iterators
derive_py_iter!(
    FBlockIterArr,
    BlockIter,
    FBlock,
    iter_heights,
    heights: Vec<usize>
);
derive_py_iter!(
    SBlockIterArr,
    BlockIter,
    SBlock,
    iter_heights,
    heights: Vec<usize>
);
derive_py_iter!(
    FBlockIter,
    BlockIter,
    FBlock,
    iter_block,
    start: usize,
    end: usize
);
derive_py_iter!(
    SBlockIter,
    BlockIter,
    SBlock,
    iter_block,
    start: usize,
    end: usize
);
derive_py_iter!(
    FConnBlockIter,
    ConnectedBlockIter,
    FConnectedBlock,
    iter_connected_block,
    end: usize
);
derive_py_iter!(
    SConnBlockIter,
    ConnectedBlockIter,
    SConnectedBlock,
    iter_connected_block,
    end: usize
);

#[macro_export]
macro_rules! derive_py_iter {
    ($new_iter_type:ident, $inner_iter_type:ident, $iter_type:ty,
     $new:ident, $( $p:ident: $type:ty ),*) => {
        #[pyclass]
        struct $new_iter_type {
            iter: $inner_iter_type<$iter_type>,
        }

        impl $new_iter_type {
            fn new(db: &BitcoinDB, $($p: $type),*) -> $new_iter_type {
                let inner_iter: $inner_iter_type<$iter_type> = db.$new($($p),*);
                $new_iter_type {
                    iter: inner_iter,
                }
            }
        }

        #[pymethods]
        impl $new_iter_type {
            fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
                slf
            }

            fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
                let option_block: Option<$iter_type> = slf.iter.next();
                if let Some(output) = option_block {
                    Python::with_gil(|py| {
                        if let Ok(py_obj) = output.to_py(py) {
                            Some(py_obj)
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            }
        }
    };
}

#[pymodule]
#[pyo3(name = "bitcoin_explorer")]
fn py_bitcoin_explorer(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<BitcoinDBPy>()?;
    Ok(())
}

# bitcoin-explorer

`bitcoin_explorer` is an efficient library for reading
bitcoin-core binary blockchain file as a database (utilising multi-threading).

This package is ported using pyO3 from rust library `bitcoin-explorer`

### Install Requirements

Currently, only macOS py39 wheels are provided. Other platforms needs building.

For pip to build the package on other OS or python version,
make sure that `rust` toolchain is installed,
and `cmake` is also installed.

## Documentation

This library has a Rust version, go to [Rust Documentation](https://docs.rs/bitcoin-explorer/)

For python documentation, go to [Documentation](https://congyuwang.github.io/Py-Bitcoin-Explorer/bitcoin_explorer.html).

## Compatibility Note

This package deals with the binary file of another software `Bitcoin Core`.
It might not be compatible with older Bitcoin Core versions.

Currently, it is compatible with Bitcoin Core version
`Bitcoin Core version v0.21.1.0-g194b9b8792d9b0798fdb570b79fa51f1d1f5ebaf
Copyright (C) 2009-2020 The Bitcoin Core developers`.

## Examples

It contains one class `BitcoinDB`.

```python
import bitcoin_explorer as bex

# parse the same path as `--datadir` argument for `bitcoind`.
db = bex.BitcoinDB("~/Bitcoin")

# get the length of the longest chain currently on disk.
db.get_max_height()

# get block of a certain height
db.get_block(1000)

# to retrieve the connected outputs of each inputs as well
db.get_block(1000, connected=True)

# get block hash of a certain height.
db.get_hash_from_height(1000)

# a fast method for getting just the header.
# in memory query, no disk access
db.get_block_header(1000)

# get block of height 1000.
db.get_height_from_hash("some hash")

# get transaction from txid.
# This queries the `levelDB` each time, thus it is relatively slow.
db.get_transaction("some txid")

# get the height of the block which this transaction belongs.
db.get_height_from_txid("some txid")

# get the script type and addresses from a script public key
db.parse_script("some hex script pubic key")

# use iterator
for block in db.get_block_iter_range(start=1000, end=2000):
    do_something_with(block)

# use iterator, iterate over heights
for block in db.get_block_iter_array(heights=[1, 3, 5, 7, 9]):
    do_something_with(block)
    
# use iterator, connect outpoints
for block in db.get_block_iter_range(end=700000, connected=True):
    do_something_with(block)
```

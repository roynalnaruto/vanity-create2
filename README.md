# CREATE2 Salt Searcher

Given the contract `deployer`, `contract` and a prefix/suffix matching pattern for the contract address, spits out `salt` that generates a contract address satisfying the pattern.

### Usage

The following command spits out a random `salt` and the corresponding contract address prefixed with `0xdeadbeef` when `0x71C7656EC7ab88b098defB751B7401B5f6d8976F` is the deployer of the `ERC20.sol` contract:

```
cargo run --prefix \
    -c openzeppelin-contracts/contracts/token/ERC20/ERC20.sol \
    -d 71C7656EC7ab88b098defB751B7401B5f6d8976F
    -m deadbeef
```

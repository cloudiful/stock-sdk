# stock-sdk

Shared SDK workspace for stock-goes-stonk stock scripts.

This repository contains:
- `stock-sdk-core`: stable DTOs and contracts
- `stock-sdk`: public re-export crate consumed by guest scripts

Stock-script projects can depend on the SDK directly with:

```toml
[dependencies]
stock-sdk = { git = "https://forgejo.cloud1ful.com/research/stock-sdk.git" }
```

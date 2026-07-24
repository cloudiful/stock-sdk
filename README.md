# stock-sdk

Public SDK workspace for stock-goes-stonk stock scripts.

This repository contains:
- `stock-sdk-core`: stable DTOs and contracts
- `stock-sdk`: public re-export crate consumed by guest scripts

Each screening universe row exposes the full daily factor snapshot through
`row.daily_factor`. Use `row.effective_price` when a strategy can fall back from
a latest quote to the previous daily close, and inspect
`row.effective_price_source` plus the factor trade date or quote fetch time for
provenance.

Stock-script projects can depend on the SDK directly with:

```toml
[dependencies]
stock-sdk = { git = "https://github.com/cloudiful/stock-sdk.git" }
```

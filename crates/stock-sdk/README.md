# stock-sdk

Public stock-script SDK for stock-goes-stonk guest projects.

Screening rows provide complete daily factors through `row.daily_factor`,
including `ma_distance_20` and `turnover_mean_20d`. `row.effective_price`
uses a latest quote when available and otherwise falls back to the daily close.
This crate re-exports the stable screening API from `stock-sdk-core` and is
published to the private Forgejo Cargo registry.

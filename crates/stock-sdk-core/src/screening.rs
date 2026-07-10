use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::DailyFactorSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EffectivePriceSource {
    LatestQuote,
    DailyClose,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ScreeningContext {
    pub universe: Vec<UniverseSnapshotRow>,
}

impl ScreeningContext {
    pub fn universe(&self) -> &[UniverseSnapshotRow] {
        &self.universe
    }

    pub fn by_market<'a>(
        &'a self,
        markets: &'a [&'a str],
    ) -> impl Iterator<Item = &'a UniverseSnapshotRow> + 'a {
        self.universe
            .iter()
            .filter(move |row| row.matches_any_market(markets))
    }

    pub fn by_industry<'a>(
        &'a self,
        industries: &'a [&'a str],
    ) -> impl Iterator<Item = &'a UniverseSnapshotRow> + 'a {
        self.universe
            .iter()
            .filter(move |row| row.matches_any_industry(industries))
    }

    pub fn by_ts_code<'a>(
        &'a self,
        ts_codes: &'a [&'a str],
    ) -> impl Iterator<Item = &'a UniverseSnapshotRow> + 'a {
        self.universe
            .iter()
            .filter(move |row| row.matches_any_ts_code(ts_codes))
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct UniverseSnapshotRow {
    pub ts_code: String,
    pub symbol: Option<String>,
    pub name: Option<String>,
    pub industry: Option<String>,
    pub market: Option<String>,
    pub latest_price: Option<f64>,
    pub effective_price: Option<f64>,
    pub effective_price_source: Option<EffectivePriceSource>,
    pub quote_fetched_at: Option<DateTime<Utc>>,
    pub pct_change: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub volume_ratio: Option<f64>,
    pub total_market_value: Option<f64>,
    pub circulating_market_value: Option<f64>,
    pub pe_ttm: Option<f64>,
    pub pb: Option<f64>,
    pub ps_ttm: Option<f64>,
    pub dv_ttm: Option<f64>,
    pub roe: Option<f64>,
    pub gross_margin: Option<f64>,
    pub debt_asset_ratio: Option<f64>,
    pub ret_5d: Option<f64>,
    pub ret_10d: Option<f64>,
    pub ret_20d: Option<f64>,
    pub ret_60d: Option<f64>,
    pub volatility_20d: Option<f64>,
    pub volatility_60d: Option<f64>,
    pub rsi14: Option<f64>,
    pub ma5: Option<f64>,
    pub ma10: Option<f64>,
    pub ma20: Option<f64>,
    pub ma60: Option<f64>,
    pub peg_ttm: Option<f64>,
    pub dist_to_high_252d: Option<f64>,
    pub dist_to_low_252d: Option<f64>,
    pub financial_stale_days: Option<i32>,
    pub risk_warning: Option<bool>,
    pub data_gaps_detected: Option<bool>,
    #[serde(default)]
    pub daily_factor: DailyFactorSnapshot,
}

#[cfg(test)]
mod tests {
    use super::{EffectivePriceSource, ScreeningContext, UniverseSnapshotRow};

    #[test]
    fn effective_price_source_uses_stable_wire_names() {
        assert_eq!(
            serde_json::to_string(&EffectivePriceSource::LatestQuote).expect("serialize"),
            "\"latest_quote\""
        );
        assert_eq!(
            serde_json::to_string(&EffectivePriceSource::DailyClose).expect("serialize"),
            "\"daily_close\""
        );
    }

    #[test]
    fn missing_daily_factor_defaults_for_older_payloads() {
        let row: UniverseSnapshotRow = serde_json::from_value(serde_json::json!({
            "ts_code": "600036.SH",
            "symbol": null,
            "name": null,
            "industry": null,
            "market": null,
            "latest_price": null,
            "effective_price": null,
            "effective_price_source": null,
            "quote_fetched_at": null,
            "pct_change": null,
            "turnover_rate": null,
            "volume_ratio": null,
            "total_market_value": null,
            "circulating_market_value": null,
            "pe_ttm": null,
            "pb": null,
            "ps_ttm": null,
            "dv_ttm": null,
            "roe": null,
            "gross_margin": null,
            "debt_asset_ratio": null,
            "ret_5d": null,
            "ret_10d": null,
            "ret_20d": null,
            "ret_60d": null,
            "volatility_20d": null,
            "volatility_60d": null,
            "rsi14": null,
            "ma5": null,
            "ma10": null,
            "ma20": null,
            "ma60": null,
            "peg_ttm": null,
            "dist_to_high_252d": null,
            "dist_to_low_252d": null,
            "financial_stale_days": null,
            "risk_warning": null,
            "data_gaps_detected": null
        }))
        .expect("deserialize older row");
        assert_eq!(row.ts_code, "600036.SH");
        assert_eq!(row.daily_factor, crate::DailyFactorSnapshot::default());
    }

    #[test]
    fn full_market_context_stays_below_payload_limit() {
        const MARKET_SIZE: usize = 5_519;
        const MAX_PAYLOAD_BYTES: usize = 24 * 1024 * 1024;

        let universe = (0..MARKET_SIZE)
            .map(|index| UniverseSnapshotRow {
                ts_code: format!("{index:06}.SH"),
                effective_price: Some(10.0),
                effective_price_source: Some(EffectivePriceSource::DailyClose),
                daily_factor: crate::DailyFactorSnapshot {
                    close_raw: Some(10.0),
                    ma_distance_20: Some(0.05),
                    turnover_mean_20d: Some(2.5),
                    computed_at: chrono::DateTime::from_timestamp(0, 0)
                        .expect("valid fixed timestamp"),
                    ..Default::default()
                },
                ..Default::default()
            })
            .collect();
        let payload = rmp_serde::to_vec_named(&ScreeningContext { universe })
            .expect("serialize screening context");
        assert!(
            payload.len() < MAX_PAYLOAD_BYTES,
            "payload is {} bytes",
            payload.len()
        );
    }
}

impl UniverseSnapshotRow {
    pub fn matches_market(&self, market: &str) -> bool {
        self.market.as_deref() == Some(market)
    }

    pub fn matches_any_market(&self, markets: &[&str]) -> bool {
        markets.is_empty() || markets.iter().any(|market| self.matches_market(market))
    }

    pub fn matches_industry(&self, industry: &str) -> bool {
        self.industry.as_deref() == Some(industry)
    }

    pub fn matches_any_industry(&self, industries: &[&str]) -> bool {
        industries.is_empty()
            || industries
                .iter()
                .any(|industry| self.matches_industry(industry))
    }

    pub fn matches_ts_code(&self, ts_code: &str) -> bool {
        self.ts_code == ts_code
    }

    pub fn matches_any_ts_code(&self, ts_codes: &[&str]) -> bool {
        ts_codes.is_empty() || ts_codes.iter().any(|ts_code| self.matches_ts_code(ts_code))
    }
}

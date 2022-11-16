use serde::{Deserialize, Serialize};
use std::env;

use crate::utils::{de_from_str_to_float, de_from_str_to_int};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompanyInformation {
    #[serde(rename = "Symbol")]
    pub symbol: String,
    #[serde(rename = "AssetType")]
    pub asset_type: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "CIK", deserialize_with = "de_from_str_to_int")]
    pub cik: i64,
    #[serde(rename = "Exchange")]
    pub exchange: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Sector")]
    pub sector: String,
    #[serde(rename = "Industry")]
    pub industry: String,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "FiscalYearEnd")]
    pub fiscal_year_end: String,
    #[serde(rename = "LatestQuarter")]
    pub latest_quarter: String,
    #[serde(
        rename = "MarketCapitalization",
        deserialize_with = "de_from_str_to_int"
    )]
    pub market_capitalization: i64,
    #[serde(rename = "EBITDA", deserialize_with = "de_from_str_to_int")]
    pub ebitda: i64,
    #[serde(rename = "PERatio", deserialize_with = "de_from_str_to_float")]
    pub peratio: f32,
    #[serde(rename = "PEGRatio", deserialize_with = "de_from_str_to_float")]
    pub pegratio: f32,
    #[serde(rename = "BookValue", deserialize_with = "de_from_str_to_float")]
    pub book_value: f32,
    #[serde(rename = "DividendPerShare", deserialize_with = "de_from_str_to_float")]
    pub dividend_per_share: f32,
    #[serde(rename = "DividendYield", deserialize_with = "de_from_str_to_float")]
    pub dividend_yield: f32,
    #[serde(rename = "EPS", deserialize_with = "de_from_str_to_float")]
    pub eps: f32,
    #[serde(
        rename = "RevenuePerShareTTM",
        deserialize_with = "de_from_str_to_float"
    )]
    pub revenue_per_share_ttm: f32,
    #[serde(rename = "ProfitMargin", deserialize_with = "de_from_str_to_float")]
    pub profit_margin: f32,
    #[serde(
        rename = "OperatingMarginTTM",
        deserialize_with = "de_from_str_to_float"
    )]
    pub operating_margin_ttm: f32,
    #[serde(
        rename = "ReturnOnAssetsTTM",
        deserialize_with = "de_from_str_to_float"
    )]
    pub return_on_assets_ttm: f32,
    #[serde(
        rename = "ReturnOnEquityTTM",
        deserialize_with = "de_from_str_to_float"
    )]
    pub return_on_equity_ttm: f32,
    #[serde(rename = "RevenueTTM", deserialize_with = "de_from_str_to_int")]
    pub revenue_ttm: i64,
    #[serde(rename = "GrossProfitTTM", deserialize_with = "de_from_str_to_int")]
    pub gross_profit_ttm: i64,
    #[serde(rename = "DilutedEPSTTM", deserialize_with = "de_from_str_to_float")]
    pub diluted_epsttm: f32,
    #[serde(
        rename = "QuarterlyEarningsGrowthYOY",
        deserialize_with = "de_from_str_to_float"
    )]
    pub quarterly_earnings_growth_yoy: f32,
    #[serde(
        rename = "QuarterlyRevenueGrowthYOY",
        deserialize_with = "de_from_str_to_float"
    )]
    pub quarterly_revenue_growth_yoy: f32,
    #[serde(
        rename = "AnalystTargetPrice",
        deserialize_with = "de_from_str_to_float"
    )]
    pub analyst_target_price: f32,
    #[serde(rename = "TrailingPE", deserialize_with = "de_from_str_to_float")]
    pub trailing_pe: f32,
    #[serde(rename = "ForwardPE", deserialize_with = "de_from_str_to_float")]
    pub forward_pe: f32,
    #[serde(
        rename = "PriceToSalesRatioTTM",
        deserialize_with = "de_from_str_to_float"
    )]
    pub price_to_sales_ratio_ttm: f32,
    #[serde(rename = "PriceToBookRatio", deserialize_with = "de_from_str_to_float")]
    pub price_to_book_ratio: f32,
    #[serde(rename = "EVToRevenue", deserialize_with = "de_from_str_to_float")]
    pub evto_revenue: f32,
    #[serde(rename = "EVToEBITDA", deserialize_with = "de_from_str_to_float")]
    pub evto_ebitda: f32,
    #[serde(rename = "Beta", deserialize_with = "de_from_str_to_float")]
    pub beta: f32,
    #[serde(rename = "52WeekHigh", deserialize_with = "de_from_str_to_float")]
    pub n52week_high: f32,
    #[serde(rename = "52WeekLow", deserialize_with = "de_from_str_to_float")]
    pub n52week_low: f32,
    #[serde(
        rename = "50DayMovingAverage",
        deserialize_with = "de_from_str_to_float"
    )]
    pub n50day_moving_average: f32,
    #[serde(
        rename = "200DayMovingAverage",
        deserialize_with = "de_from_str_to_float"
    )]
    pub n200day_moving_average: f32,
    #[serde(rename = "SharesOutstanding", deserialize_with = "de_from_str_to_int")]
    pub shares_outstanding: i64,
    #[serde(rename = "DividendDate")]
    pub dividend_date: String,
    #[serde(rename = "ExDividendDate")]
    pub ex_dividend_date: String,
}

/// env variable API_KEY is required for this function not to panic
pub fn get_external_data(company: &str) -> Result<CompanyInformation, Box<dyn std::error::Error>> {
    let api_key = env::var("API_KEY").unwrap();

    let url = format!(
        "https://www.alphavantage.co/query?function=OVERVIEW&symbol={}&apikey={}",
        company, api_key
    );
    let resp = reqwest::blocking::get(url)?.json::<CompanyInformation>()?;
    return Ok(resp);
}

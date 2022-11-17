use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::{de_from_str_to_float, de_from_str_to_int};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeries {
    #[serde(rename(deserialize = "Meta Data", serialize = "metaData"))]
    pub meta_data: MetaData,
    #[serde(rename(serialize = "timeSeries"), flatten)]
    pub time_series: HashMap<String, HashMap<String, TimeSeriesItem>>,
} 

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    #[serde(rename(deserialize = "1. Information", serialize = "information"))]
    pub information: String,
    #[serde(rename(deserialize = "2. Symbol", serialize = "symbol"))]
    pub symbol: String,
    #[serde(rename(deserialize = "3. Last Refreshed", serialize = "lastRefreshed"))]
    pub last_refreshed: String,
    #[serde(rename(deserialize = "4. Interval", serialize = "interval"))]
    pub interval: String,
    #[serde(rename(deserialize = "5. Output Size", serialize = "outputSize"))]
    pub output_size: String,
    #[serde(rename(deserialize = "6. Time Zone", serialize = "timeZone"))]
    pub time_zone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesItem {
    #[serde(
        rename(deserialize = "1. open", serialize = "open"),
        deserialize_with = "de_from_str_to_float"
    )]
    pub open: f32,
    #[serde(
        rename(deserialize = "2. high", serialize = "high"),
        deserialize_with = "de_from_str_to_float"
    )]
    pub high: f32,
    #[serde(
        rename(deserialize = "3. low", serialize = "low"),
        deserialize_with = "de_from_str_to_float"
    )]
    pub low: f32,
    #[serde(
        rename(deserialize = "4. close", serialize = "close"),
        deserialize_with = "de_from_str_to_float"
    )]
    pub close: f32,
    #[serde(
        rename(deserialize = "5. volume", serialize = "volume"),
        deserialize_with = "de_from_str_to_int"
    )]
    pub volume: i64,
}

fn get_timeseries_data(company: &str) -> Result<TimeSeries, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_INTRADAY&symbol={}&interval=5min&apikey=demo",
        company
    );
    let resp = reqwest::blocking::get(url)?.json::<TimeSeries>()?;
    return Ok(resp);
}

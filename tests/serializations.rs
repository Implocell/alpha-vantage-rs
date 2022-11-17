use alpha_vantage::core_stock::intraday;
use alpha_vantage::fundemental_data::company_information;

#[test]
fn it_serializes_company_information() {
    let company_information_json = r#"
    {
    "Symbol": "IBM",
    "AssetType": "Common Stock",
    "Name": "International Business Machines",
    "Description": "International Business Machines Corporation (IBM) is an American multinational technology company headquartered in Armonk, New York, with operations in over 170 countries. The company began in 1911, founded in Endicott, New York, as the Computing-Tabulating-Recording Company (CTR) and was renamed International Business Machines in 1924. IBM is incorporated in New York. IBM produces and sells computer hardware, middleware and software, and provides hosting and consulting services in areas ranging from mainframe computers to nanotechnology. IBM is also a major research organization, holding the record for most annual U.S. patents generated by a business (as of 2020) for 28 consecutive years. Inventions by IBM include the automated teller machine (ATM), the floppy disk, the hard disk drive, the magnetic stripe card, the relational database, the SQL programming language, the UPC barcode, and dynamic random-access memory (DRAM). The IBM mainframe, exemplified by the System/360, was the dominant computing platform during the 1960s and 1970s.",
    "CIK": "51143",
    "Exchange": "NYSE",
    "Currency": "USD",
    "Country": "USA",
    "Sector": "TECHNOLOGY",
    "Industry": "COMPUTER & OFFICE EQUIPMENT",
    "Address": "1 NEW ORCHARD ROAD, ARMONK, NY, US",
    "FiscalYearEnd": "December",
    "LatestQuarter": "2022-09-30",
    "MarketCapitalization": "123145404000",
    "EBITDA": "12010000000",
    "PERatio": "22.56",
    "PEGRatio": "1.182",
    "BookValue": "22.2",
    "DividendPerShare": "6.59",
    "DividendYield": "0.0477",
    "EPS": "6.09",
    "RevenuePerShareTTM": "67.2",
    "ProfitMargin": "0.0209",
    "OperatingMarginTTM": "0.115",
    "ReturnOnAssetsTTM": "0.0322",
    "ReturnOnEquityTTM": "0.0648",
    "RevenueTTM": "60535001000",
    "GrossProfitTTM": "31486000000",
    "DilutedEPSTTM": "6.09",
    "QuarterlyEarningsGrowthYOY": "0.041",
    "QuarterlyRevenueGrowthYOY": "0.065",
    "AnalystTargetPrice": "139.96",
    "TrailingPE": "22.56",
    "ForwardPE": "14.39",
    "PriceToSalesRatioTTM": "2.034",
    "PriceToBookRatio": "6.23",
    "EVToRevenue": "2.777",
    "EVToEBITDA": "24.13",
    "Beta": "0.877",
    "52WeekHigh": "141.25",
    "52WeekLow": "109.15",
    "50DayMovingAverage": "127.42",
    "200DayMovingAverage": "131.59",
    "SharesOutstanding": "896320000",
    "DividendDate": "2022-12-10",
    "ExDividendDate": "2022-11-09"
        }"#;

    let res: Result<company_information::CompanyInformation, serde_json::Error> =
        serde_json::from_str(company_information_json);

    if res.is_err() {
        res.as_ref().unwrap();
    }

    let u_res = res.unwrap();

    assert_eq!(
        u_res.address,
        String::from("1 NEW ORCHARD ROAD, ARMONK, NY, US")
    );
    assert_eq!(u_res.market_capitalization, 123145404000);
    assert_eq!(u_res.peratio, 22.56);
}

#[test]
fn it_serializes_timeseries() {
    let time_series_5min_daily = r#"
    {
        "Meta Data": {
            "1. Information": "Intraday (5min) open, high, low, close prices and volume",
            "2. Symbol": "IBM",
            "3. Last Refreshed": "2022-11-15 16:15:00",
            "4. Interval": "5min",
            "5. Output Size": "Compact",
            "6. Time Zone": "US/Eastern"
        },
        "Time Series (5min)": {
            "2022-11-15 16:15:00": {
                "1. open": "144.3400",
                "2. high": "144.3400",
                "3. low": "144.3400",
                "4. close": "144.3400",
                "5. volume": "4464"
            },
            "2022-11-15 16:05:00": {
                "1. open": "144.3400",
                "2. high": "144.3400",
                "3. low": "144.3400",
                "4. close": "144.3400",
                "5. volume": "159775"
            },
            "2022-11-15 16:00:00": {
                "1. open": "144.0000",
                "2. high": "144.3600",
                "3. low": "144.0000",
                "4. close": "144.3300",
                "5. volume": "188048"
            }
        }
    }"#;

    let res: Result<intraday::TimeSeries, serde_json::Error> =
        serde_json::from_str(time_series_5min_daily);
    if res.is_err() {
        res.as_ref().unwrap();
    }

    let u_res = res.unwrap();

    assert_eq!(u_res.time_series["Time Series (5min)"]["2022-11-15 16:15:00"].volume, 4464);

    let diff = u_res.time_series["Time Series (5min)"]["2022-11-15 16:15:00"].close - 144.3400;
    assert!(diff.abs() < f32::EPSILON);

    let time_series_60min_daily = r#"
    {
        "Meta Data": {
            "1. Information": "Intraday (60min) open, high, low, close prices and volume",
            "2. Symbol": "IBM",
            "3. Last Refreshed": "2022-11-15 16:15:00",
            "4. Interval": "60min",
            "5. Output Size": "Compact",
            "6. Time Zone": "US/Eastern"
        },
        "Time Series (60min)": {
            "2022-11-15 16:15:00": {
                "1. open": "144.3400",
                "2. high": "144.3400",
                "3. low": "144.3400",
                "4. close": "144.3400",
                "5. volume": "4464"
            },
            "2022-11-15 15:15:00": {
                "1. open": "144.3400",
                "2. high": "144.3400",
                "3. low": "144.3400",
                "4. close": "144.3400",
                "5. volume": "159775"
            },
            "2022-11-15 14:15:00": {
                "1. open": "144.0000",
                "2. high": "144.3600",
                "3. low": "144.0000",
                "4. close": "144.3300",
                "5. volume": "188048"
            }
        }
    }"#;

    let res: Result<intraday::TimeSeries, serde_json::Error> =
    serde_json::from_str(time_series_60min_daily);
if res.is_err() {
    res.as_ref().unwrap();
}

let u_res = res.unwrap();

assert_eq!(u_res.time_series["Time Series (60min)"]["2022-11-15 16:15:00"].volume, 4464);

let diff = u_res.time_series["Time Series (60min)"]["2022-11-15 16:15:00"].close - 144.3400;
assert!(diff.abs() < f32::EPSILON);

}

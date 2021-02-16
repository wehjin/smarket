//! Get prices of assets

use std::collections::HashMap;
use std::error::Error;

use serde::Deserialize;

/// Price of an asset in US dollars
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct UsdPrice(f64);

impl UsdPrice {
	/// The price as a float
	pub fn as_f64(&self) -> f64 { self.0 }
}

/// Result from pricing an asset
#[derive(Debug)]
pub enum PricingResult {
	Priced { usd_price: UsdPrice, symbol: String },
	NoPriceInQuote { quote_result: String, symbol: String },
	NoQuote { request_response: String, symbol: String },
}

/// Acquires prices for various assets from their symbol
pub fn price_assets<S: AsRef<str>>(symbols: &Vec<S>) -> Result<HashMap<String, PricingResult>, Box<dyn Error>> {
	let symbols = symbols.iter().map(|it| it.as_ref().trim().to_uppercase()).collect::<Vec<_>>();
	let mut params = STATIC_PRICING_PARAMS.iter().map(|it| it.to_string()).collect::<Vec<_>>();
	params.extend(vec![
		format!("fields={}", PRICING_FIELDS.join("%2C")),
		format!("symbols={}", symbols.join("%2C"))
	]);
	let url = format!("https://query1.finance.yahoo.com/v7/finance/quote?{}", params.join("&"));
	let request_response = reqwest::blocking::get(&url)?.json::<RequestResponse>()?;
	let quote_results = request_response.quote_response.result
		.iter()
		.map(|it| (it.symbol.to_uppercase(), it.clone()))
		.collect::<HashMap<String, _>>();
	let pricing_results = symbols
		.iter()
		.map(|symbol| {
			let pricing_result = match quote_results.get(symbol) {
				Some(quote_result) => match quote_result.regular_market_price {
					Some(price) => PricingResult::Priced { symbol: symbol.clone(), usd_price: UsdPrice(price) },
					None => PricingResult::NoPriceInQuote { symbol: symbol.clone(), quote_result: format!("{:?}", quote_result) },
				}
				None => PricingResult::NoQuote {
					symbol: symbol.to_string(),
					request_response: format!("{:?}", request_response),
				},
			};
			(symbol.clone(), pricing_result)
		})
		.collect::<HashMap<String, _>>();
	Ok(pricing_results)
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct RequestResponse {
	pub quote_response: QuoteResponse
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct QuoteResponse {
	pub result: Vec<QuoteResult>,
	pub error: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct QuoteResult {
	pub regular_market_price: Option<f64>,
	pub symbol: String,
}

const PRICING_FIELDS: [&str; 7] = [
	"quoteType",
	"symbol",
	"longName",
	"shortName",
	"regularMarketPrice",
	"marketCap",
	"sharesOutstanding"
];

const STATIC_PRICING_PARAMS: [&str; 4] = [
	"lang=en-US",
	"region=US",
	"corsDomain=finance.yahoo.com",
	"formatted=false"
];


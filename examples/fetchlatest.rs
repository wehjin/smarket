extern crate smarket;

use std::error::Error;

use smarket::yf;
use smarket::yf::PricingResult;

pub fn main() -> Result<(), Box<dyn Error>> {
	let pricing_results = yf::price_assets(&vec!["PEP", "FUV", "BTC-USD", "FIL-USD"])?;
	println!("{:?}", pricing_results);
	let pep_result = pricing_results.get("PEP").expect("PEP result");
	match pep_result {
		PricingResult::Priced { usd_price, .. } => {
			println!("Value of 100 PEP shares: {}", usd_price.as_f64() * 100.0)
		}
		_ => {
			println!("No price for PEP: {:?}", pep_result)
		}
	}
	Ok(())
}
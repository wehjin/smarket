extern crate smarket;

use smarket::{Market, SharePrice};

#[test]
fn persist() {
	let diversifier = rand::random();
	{
		let market = setup("persist", diversifier);
		market.update_share_price("nkla", SharePrice::Float(2.0));
	}
	let market = setup("persist", diversifier);
	let share_prices = market.read_share_prices(vec!["nkla".to_string()]);
	assert_eq!(share_prices["nkla"], SharePrice::Float(2.0));
}

#[test]
fn share_prices() {
	let market = setup("share_prices", rand::random());
	let symbols = vec!["tsla".to_string(), "nkla".to_string(), ];
	let share_prices = market.read_share_prices(symbols);
	assert_eq!(share_prices["tsla"], SharePrice::Float(1007.22));
	assert_eq!(share_prices.contains_key("nkla"), false);
}

fn setup(name: &str, diversifier: u64) -> Market {
	let data_folder = {
		let mut dir = std::env::temp_dir();
		dir.push(&format!("smarket-test-{}-{}", name, diversifier));
		dir
	};
	let market = Market::open(&data_folder);
	market.update_share_price("tsla", 1007.22.into());
	market
}

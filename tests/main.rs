extern crate smarket;


use smarket::{Market, SharePrice, TxnPrice};

#[test]
fn persist() {
	{
		let market = setup("persist", rand::random());
		market.update_share_price("nkla".to_string(), SharePrice::Float(4.0));
	}
}

#[test]
fn share_prices() {
	let market = setup("share_prices", rand::random());
	let share_prices = market.read_share_prices(vec![
		"tsla".to_string(),
		"nkla".to_string(),
	]);
	assert_eq!(share_prices["tsla"], Some(SharePrice::Float(1007.22)));
	assert_eq!(share_prices["nkla"], None);
}

#[test]
fn txn_price() {
	let market = setup("txn_price", rand::random());
	let txn_price = market.read_txn_price("tsla".to_string(), 100.into());
	assert_eq!(txn_price, Some(TxnPrice::Float(100722.0)));
}

fn setup(name: &str, diversifier: u64) -> Market {
	let data_folder = {
		let mut dir = std::env::temp_dir();
		dir.push(&format!("smarket-test-{}-{}", name, diversifier));
		dir
	};
	eprintln!("test folder: {:?}", &data_folder);
	let market = Market::open(&data_folder);
	market.update_share_price("tsla".to_string(), 1007.22.into());
	market
}

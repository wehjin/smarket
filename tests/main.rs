extern crate smarket;


use smarket::{Market, SharePrice, TxnPrice};

fn setup() -> Market {
	let market = Market::open();
	market.update_share_price("tsla".to_string(), 1007.22.into());
	market
}

#[test]
fn share_prices() {
	let market = setup();
	let share_prices = market.share_prices(vec![
		"tsla".to_string(),
		"nkla".to_string(),
	]);
	assert_eq!(share_prices["tsla"], Some(SharePrice::Float(1007.22)));
	assert_eq!(share_prices["nkla"], None);
}

#[test]
fn txn_price() {
	let market = setup();
	let txn_price = market.txn_price("tsla".to_string(), 100.into());
	assert_eq!(txn_price, Some(TxnPrice::Float(100722.0)));
}

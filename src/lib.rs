use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::SystemTime;

use echo_lib::Echo;

pub use self::basics::*;

mod basics;
mod sample;

#[derive(Debug, Clone)]
pub struct Market { tx: Sender<Msg> }

impl Market {
	pub fn read_share_prices(&self, symbols: Vec<String>) -> HashMap<String, SharePrice> {
		let (tx, rx) = channel();
		self.tx.send(Msg::GetSharePrices(symbols, tx)).unwrap();
		rx.recv().unwrap()
	}

	pub fn update_share_price(&self, symbol: String, share_price: SharePrice) {
		let (tx, rx) = channel();
		self.tx.send(Msg::SetSharePrice(symbol, share_price, tx)).unwrap();
		rx.recv().unwrap()
	}

	pub fn open(data_folder: &Path) -> Self {
		let echo = Echo::connect("smarket-1", data_folder);
		let (tx, rx) = channel();
		thread::spawn(move || {
			let mut latest = echo.chamber().unwrap();
			for msg in rx {
				match msg {
					Msg::SetSharePrice(symbol, price, out) => {
						let symbol = canonical_symbol(symbol);
						sample::update(&symbol, price, SystemTime::now(), &echo);
						latest = echo.chamber().unwrap();
						out.send(()).unwrap();
					}
					Msg::GetSharePrices(symbols, out) => {
						let mut out_prices = HashMap::new();
						for symbol in symbols {
							let symbol = canonical_symbol(symbol);
							let share_price = sample::read(&symbol, &latest);
							if let Some(share_price) = share_price {
								out_prices.insert(symbol, share_price);
							}
						}
						out.send(out_prices).unwrap();
					}
				}
			}
		});
		Market { tx }
	}
}


enum Msg {
	GetSharePrices(Vec<String>, Sender<HashMap<String, SharePrice>>),
	SetSharePrice(String, SharePrice, Sender<()>),
}


fn canonical_symbol(symbol: String) -> String {
	symbol.trim().to_lowercase()
}

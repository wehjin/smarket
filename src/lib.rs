use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub use self::basics::*;

#[derive(Debug, Clone)]
pub struct Market { tx: Sender<Msg> }

impl Market {
	pub fn read_txn_price(&self, symbol: String, txn_size: TxnSize) -> Option<TxnPrice> {
		let (tx, rx) = channel();
		self.tx.send(Msg::GetTxnPrice(symbol, txn_size, tx)).unwrap();
		rx.recv().unwrap()
	}

	pub fn read_share_prices(&self, symbols: Vec<String>) -> HashMap<String, Option<SharePrice>> {
		let (tx, rx) = channel();
		self.tx.send(Msg::GetSharePrices(symbols, tx)).unwrap();
		rx.recv().unwrap()
	}

	pub fn update_share_price(&self, symbol: String, share_price: SharePrice) {
		self.tx.send(Msg::SetSharePrice(symbol, share_price)).unwrap();
	}

	pub fn open(folder: &Path) -> Self {
		let mut folder = folder.to_path_buf();
		folder.push("smarket");
		std::fs::create_dir_all(&folder).unwrap();
		let (tx, rx) = channel();
		thread::spawn(move || loop_messages(rx));
		Market { tx }
	}
}

fn loop_messages(tx: Receiver<Msg>) {
	let mut share_prices = HashMap::new();
	for msg in tx {
		match msg {
			Msg::GetTxnPrice(symbol, txn_size, out) => {
				let symbol = canonical_symbol(symbol);
				let txn_price = share_prices
					.get(&symbol)
					.map(|it| txn_price(it, &txn_size));
				out.send(txn_price).unwrap();
			}
			Msg::SetSharePrice(symbol, price) => {
				let symbol = canonical_symbol(symbol);
				share_prices.insert(symbol, price);
			}
			Msg::GetSharePrices(symbols, out) => {
				let mut out_prices = HashMap::new();
				for symbol in symbols {
					let symbol = canonical_symbol(symbol);
					let price = share_prices.get(&symbol).cloned();
					out_prices.insert(symbol, price);
				}
				out.send(out_prices).unwrap();
			}
		}
	}
}

fn canonical_symbol(symbol: String) -> String {
	symbol.trim().to_lowercase()
}

enum Msg {
	GetTxnPrice(String, TxnSize, Sender<Option<TxnPrice>>),
	GetSharePrices(Vec<String>, Sender<HashMap<String, Option<SharePrice>>>),
	SetSharePrice(String, SharePrice),
}

mod basics;

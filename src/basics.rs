/// Produces a transaction price from and share price and a transaction size.
pub fn txn_price(share_price: &SharePrice, txn_size: &TxnSize) -> TxnPrice {
	TxnPrice::Float(share_price.as_f64() * txn_size.as_f64())
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum SharePrice { Float(f64) }

impl SharePrice {
	pub fn as_f64(&self) -> f64 { match self { SharePrice::Float(f) => *f, } }
}

impl From<f64> for SharePrice {
	fn from(value: f64) -> Self {
		SharePrice::Float(value)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TxnSize { Float(f64) }

impl From<f64> for TxnSize {
	fn from(value: f64) -> Self {
		TxnSize::Float(value)
	}
}

impl From<u64> for TxnSize {
	fn from(value: u64) -> Self {
		TxnSize::Float(value as f64)
	}
}

impl TxnSize {
	pub fn as_f64(&self) -> f64 { match self { TxnSize::Float(f) => *f, } }
}


#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TxnPrice { Float(f64) }

impl TxnPrice {
	pub fn as_f64(&self) -> f64 { match self { TxnPrice::Float(f) => *f, } }
}



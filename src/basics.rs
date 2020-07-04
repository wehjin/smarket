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

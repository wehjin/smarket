use std::time::SystemTime;

use echo_lib::{Chamber, Echo, Point, Target};

use crate::SharePrice;

const SAMPLE_SYMBOL: &Point = &Point::Static { aspect: "Sample", name: "symbol" };
const SAMPLE_SHARE_PRICE: &Point = &Point::Static { aspect: "Sample", name: "share_price" };
const SAMPLE_TIME: &Point = &Point::Static { aspect: "Sample", name: "time" };

pub fn update(symbol: &str, share_price: SharePrice, time: SystemTime, echo: &Echo) {
	echo.write(|scope| {
		let symbol_target = Target::String(symbol.to_string());
		let (object_id, mut key_properties) =
			if let Some(object_id) = echo.chamber().unwrap().objects_with_property(SAMPLE_SYMBOL, &symbol_target).unwrap().first() {
				(object_id.clone(), vec![])
			} else {
				(scope.new_object_id("saimple"), vec![(SAMPLE_SYMBOL, symbol_target)])
			};
		let mut properties = vec![
			(SAMPLE_SHARE_PRICE, Target::String(format!("{}", share_price.as_f64()))),
			(SAMPLE_TIME, Target::Number(time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()))
		];
		properties.append(&mut key_properties);
		scope.write_object_properties(&object_id, properties);
	}).unwrap();
}

pub fn read(symbol: &str, chamber: &Chamber) -> Option<SharePrice> {
	chamber.objects_with_property(SAMPLE_SYMBOL, &Target::String(symbol.to_string())).unwrap()
		.first()
		.map(|object_id| chamber.target_at_object_point(&object_id, SAMPLE_SHARE_PRICE))
		.map(|target| SharePrice::from(target.as_str().parse::<f64>().unwrap()))
}
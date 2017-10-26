use std::error::Error;
use keyword;
use super::all::All;
use super::ret::Ret;

pub struct Item {
	a:All,
}

pub fn new() -> Item {
	Item {a:All::new()}
}

impl super::Item for Item {
	fn kw(&self) -> &'static keyword::Item {&keyword::PRINT}
	fn a(&mut self) -> &mut All {&mut self.a}
	fn a2(&self) -> &All {&self.a}

	fn z(&self, ret:&mut Ret) -> Result<bool, &Error> {
		let mut a = Vec::new();
		if let Err(e) = super::z2__(&self.a, &mut a) {
			return Err(e)
		}
		for s in a.into_iter() {
			print!("{}", s)
		}
		Ok(true)
	}
}
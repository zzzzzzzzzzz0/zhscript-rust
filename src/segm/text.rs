use std::error::Error;
use keyword;
use super::all::All;
use super::ret::Ret;

pub struct Item {
	val:String,
	a:All,
}

pub fn new(val:String) -> Item {
	Item {val:val, a:All::new()}
}

impl super::Item for Item {
	fn kw(&self) -> &'static keyword::Item {&keyword::KAIYINHAO}
	fn kw2(&self) -> &'static keyword::Item {&keyword::BIYINHAO}
	fn a(&mut self) -> &mut All {&mut self.a}
	fn a2(&self) -> &All {&self.a}
	
	fn s(&self) -> String {
		let mut s = String::new();
		s.push_str(self.kw().val());
		s.push_str(&self.val);
		s.push_str(self.kw2().val());
		s
	}

	fn z(&self, ret:&mut Ret) -> Result<bool, &Error> {
		ret.push(&self.val);
		Ok(true)
	}
}
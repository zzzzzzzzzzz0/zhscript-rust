use std::error::Error;
use keyword;
use super::all::All;
use super::ret::Ret;
use eval;

pub struct Item {
	a:All,
}

pub fn new() -> Item {
	Item {a:All::new()}
}

impl super::Item for Item {
	fn kw(&self) -> &'static keyword::Item {&keyword::EVAL}
	fn a(&mut self) -> &mut All {&mut self.a}
	fn a2(&self) -> &All {&self.a}

	fn z(&self, ret:&mut Ret) -> Result<bool, &Error> {
		let mut s = String::new();
		{
			let mut a = Vec::new();
			if let Err(e) = super::z2__(&self.a, &mut a) {
				return Err(e)
			}
			for s2 in a.into_iter() {
				s.push_str(&s2)
			}
		}
		/*let ref mut a2 = self.a2;
		a2.clear();*/
		let mut a2 = eval::List::new();
		match a2.parse(&s) {
			Ok(_) => {},
			Err(c) => {
				ret.push("");
				ret.push(&format!("非法字符'{}'", c));
				return Ok(true);
			},
		}
		ret.push(&(a2.z().to_string()));
		/*unsafe {
			match cache_ {
				Some(_) => {},
				None => {
					cache_ = Some(cache_new());
				},
			}
			if let Some(ref mut cache) = cache_ {
				//cache.a.push(a2);
			}
		}*/
		Ok(true)
	}
}

/*struct Cache {
	a:Vec<eval::List>
}
fn cache_new() -> &'static mut Cache {
	&mut Cache {a:Vec::new()}
}
static mut cache_:Option<&'static mut Cache> = None;*/
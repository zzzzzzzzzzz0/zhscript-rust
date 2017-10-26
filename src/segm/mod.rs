use std::error::Error;
use keyword;
use self::ret::Ret;
use self::all::All;

mod text;
mod no;
mod print;
mod eval;

mod all;
mod ret;

pub trait Item {
	fn kw(&self) -> &'static keyword::Item;
	fn kw2(&self) -> &'static keyword::Item {&keyword::NO}
	fn a(&mut self) -> &mut All;
	fn a2(&self) -> &All;
	fn z(&self, ret:&mut Ret) -> Result<bool, &Error>;

	fn s(&self) -> String {
		let mut s = String::new();
		s.push_str(self.kw().val());
		s.push_str(self.kw2().val());
		s
	}
}

struct Item1 {
	kw:&'static keyword::Item,
	a:All,
}

impl Item1 {
	fn new(kw:&'static keyword::Item) -> Self {
		Item1 {kw:kw, a:All::new()}
	}
}
impl Item for Item1 {
	fn kw(&self) -> &'static keyword::Item {self.kw}
	fn a(&mut self) -> &mut All {&mut self.a}
	fn a2(&self) -> &All {&self.a}

	fn z(&self, ret:&mut Ret) -> Result<bool, &Error> {
		match *self.kw.val2() {
			keyword::Id::Lf  => ret.push("\n"),
			keyword::Id::Cr  => ret.push("\r"),
			keyword::Id::Tab => ret.push("\t"),
			keyword::Id::Esc => ret.push("\x1b"),
			_ => {}
		}
		Ok(true)
	}
}

pub struct List {
	a:All,
}

impl List {
	pub fn new() -> Self {
		List {a:All::new()}
	}

	pub fn parse(&mut self, s:&str) -> Result<bool, &Error> {
		z__(s, &mut 0, false, &mut self.a)
	}
	
	pub fn z(&mut self, ret:&mut Vec<String>) -> Result<bool, &Error> {
		self.a.ptree();
		let mut buf = String::new();
		self.a.z(&mut Ret::new(ret, &mut buf))
	}
}

fn s2__(s2:&mut String, is_text:i32, a:&mut All) {
	if !s2.is_empty() {
		if is_text > 0 {
			a.push(Box::new(text::new(s2.to_string())))
		} else {
			a.push(Box::new(no::new(s2.to_string())))
		}
		s2.clear();
	}
}
	
fn z__<'a>(s:&str, i:&mut usize, ret_if_juhao:bool, a:&mut All) -> Result<bool, &'a Error> {
	let mut is_text = 0;
	let mut kw_len = 0;
	let mut s2 = String::new();
	loop {
		if *i >= s.chars().count() {
			break
		}
		match keyword::get(s, *i, &mut kw_len) {
			Some(kw) => {
				s2__(&mut s2, is_text, a);
				let ref kw2 = *kw.val2();
				if *kw2 == keyword::Id::Juhao && ret_if_juhao {
					return Ok(true)
				}
				*i += kw_len;
				match *kw2 {
					keyword::Id::Kaiyinhao => is_text += 1,
					keyword::Id::Biyinhao => is_text -= 1,
					keyword::Id::Eval => {
						let mut buf = eval::new();
						match z__(s, i, true, buf.a()) {
							Err(e) => return Err(e),
							Ok(b) => if b {a.push(Box::new(buf))}
						}
					},
					keyword::Id::Print => {
						let mut buf = print::new();
						match z__(s, i, true, buf.a()) {
							Err(e) => return Err(e),
							Ok(b) => if b {a.push(Box::new(buf))}
						}
					},
					keyword::Id::Lf | keyword::Id::Cr | keyword::Id::Tab | keyword::Id::Esc =>
						a.push(Box::new(Item1::new(kw))),
					keyword::Id::Juhao |
					keyword::Id::No => {}
				}
			},
			None => {
				if let Some(c) = s.chars().nth(*i) {
					 s2.push(c)
				}
				*i += 1
			}
		}
	}
	s2__(&mut s2, is_text, a);
	Ok(true)
}

fn z2__<'a>(a:&'a All, ret:&mut Vec<String>) -> Result<bool, &'a Error> {
	let mut buf = String::new();
	let mut ret2 = Ret::new(ret, &mut buf);
	if let Err(e) = a.z(&mut ret2) {
		return Err(e)
	}
	ret2.one(true);
	Ok(true)
}

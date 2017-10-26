use core::fmt;

#[derive(PartialEq)]
enum Typ {
	N,
	O,
}

struct Item {
	typ:Typ,
	val:f64,
	val2:char,
}

impl Item {
	fn new(val:f64) -> Self {
		Item {typ:Typ::N, val:val, val2:' '}
	}
	fn new2(val:char) -> Self {
		Item {typ:Typ::O, val2:val, val:0.0}
	}
}

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.typ {
			Typ::N => write!(f, "Item {{ val: {} }}", self.val),
			Typ::O => write!(f, "Item {{ val: '{}' }}", self.val2),
		}
	}
}

pub struct List {
	a:Vec<Item>,
}

impl List {
	pub fn new() -> Self {
		List {a:Vec::new()}
	}
	fn add(&mut self, s2:&mut String) {
		if !s2.is_empty() {
			self.a.push(Item::new(s2.parse::<f64>().unwrap()));
			s2.clear();
		}
	}
	pub fn clear(&mut self) {
		self.a.clear();
	}
	pub fn pn(&self) {
		for i in self.a.iter() {
			println!("{}", i);
		}
	}
	pub fn parse(&mut self, s:&str) -> Result<bool, char> {
		let mut s2:String = String::new();
		for c in s.chars() {
			match c {
				'0' ... '9' | '.' => s2.push(c),
				'+' | '-' | '*' | '/'  | '%'  | '^'
				| '(' | ')' => {
					self.add(&mut s2);
					self.a.push(Item::new2(c));
				},
				_ => {
					return Err(c);
				},
			}
		}
		self.add(&mut s2);
		Ok(true)
	}
	pub fn z(self) -> f64 {
		let mut ret = 0f64;
		let mut ii:usize = 0;
		self.z2(' ', &mut ii, &mut ret);
		ret
	}
	fn z5(&self, o:char, val:f64, ret:&mut f64) {
		match o {
			'*' => *ret *= val,
			'/' => *ret /= val,
			'%' => *ret %= val,
			'^' => {
				let f = *ret;
				let max:usize = val as usize;
				for _ in 1..max {
					*ret *= f;
				}
			},
			'-' => *ret -= val,
			_ => *ret += val,
		}
	}
	fn z4(&self, i:&Item, ii:&mut usize, o:&mut char, ret:&mut f64) {
		match i.typ {
			Typ::O => {
				match i.val2 {
					'(' => {
							let mut ret2 = 0f64;
							*ii += 1;
							self.z2(')', ii, &mut ret2);
							self.z5(*o, ret2, ret);
						},
					_ => *o = i.val2,
				}
			},
			Typ::N => {
				match *o {
					'*' | '/' => self.z5(*o, i.val, ret),
					_ => {
							let mut ret2 = i.val;
							*ii += 1;
							self.z2('+', ii, &mut ret2);
							self.z5(*o, ret2, ret);
						},
				}
			},
		}
	}
	fn z2(&self, chk:char, ii:&mut usize, ret:&mut f64) {
		let mut o = ' ';
		while *ii < self.a.len() {
			let i = &self.a[*ii];
			if i.typ == Typ::O {
				match chk {
					'+' => {
						match i.val2 {
							'+' | '-' => {
								*ii -= 1;
								break
							},
							_ => {},
						}
					},
					')' => {
						match i.val2 {
							')' => break,
							_ => {},
						}
					},
					_ => {}
				}
			}
			self.z4(i, ii, &mut o, ret);
			*ii += 1;
		}
	}
}

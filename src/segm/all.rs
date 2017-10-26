use std::error::Error;
use super::ret::Ret;

pub struct All {
	a:Vec<Box<super::Item>>,
}

impl All {
	pub fn new() -> Self {
		All {a:Vec::new()}
	}
	
	pub fn push(&mut self, i:Box<super::Item>) {
		self.a.push(i);
	}

	pub fn ptree(&self) {
		ptree2(self, 0)
	}

	pub fn z(&self, ret:&mut Ret) -> Result<bool, &Error> {
		//cannot move out of borrowed content
		for i in self.a.iter() {
			if let Err(e) = i.z(ret) {
				return Err(e)
			}
		}
		ret.one(true);
		Ok(true)
	}
}

fn ptree2(a:&All, lvl:i32) {
	let ref a = a.a;
	let len = a.len();
	//cannot borrow immutable `Box` content `**i` as mutable
	for (i2, i) in a.iter().enumerate() {
		let lvl1 = lvl + 1;
		head(lvl1, if lvl == 0 {0} else if i2 == len - 1 {2} else {1});
		println!("{}", i.s());
		ptree2(i.a2(), lvl1);
	}
}

fn head(lvl:i32, head:u8) {
	let len = 4;
	for _ in 0..(lvl - 2) {
		for _ in 0..len {
			print!(" ")
		}
	}
	match head {
		1 => print!("|"),
		2 => print!("\\"),
		_ => {return}
	}
	for _ in 0..(len - 1) {
		print!("-")
	}
}

#![feature(const_fn)]

extern crate core;

mod eval;
mod keyword;
mod segm;

pub fn z(s:&str, ret:&mut Vec<String>) {
	let mut ls = segm::List::new();
	ls.parse(s);
	ls.z(ret);
}

#[cfg(test)]
mod tests {
	use {keyword, eval};
	
	#[test]
	fn it_works() {
	}

	#[test]
	fn keword() {
		let mut ret = vec![];
		keyword_test(&mut ret);
		for i in ret {
			print!("{} ", i)
		}
		println!("")
	}
	fn keyword_test(ret:&mut Vec<String>) {
		for i in keyword::ALL.iter() {
			ret.push(i.val().to_string());
		}
	}
	
	#[test]
	fn eval() {
		fn test(s:&str) {
			let mut ret = vec![];
			eval_test(s, &mut ret);
			for i in ret {
				println!("{}", i)
			}
		}
		test("1+22-333*4444/55555+1.2*(3+4.5)");
		test("0%2");
		test("1%2");
		test("2%2");
		test("3%2");
		test("10^2");
		test("1.2^2");
		test("-(1+2*3)");
		test("1+2");
	}
	fn eval_test(s:&str, ret:&mut Vec<String>) {
		let mut a = eval::List::new();
		match a.parse(&s) {
			Ok(_) => {},
			Err(c) => {
				ret.push("".to_string());
				ret.push(format!("非法字符'{}'", c));
				return;
			},
		}
		a.pn();
		println!("{}", s);
		for _ in 0..s.len() {
			print!("-");
		}
		print!("\n");
		ret.push(a.z().to_string());
	}
}

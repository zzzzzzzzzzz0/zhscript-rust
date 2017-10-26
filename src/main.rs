use std::io;
use std::env;

extern crate zhscript;

fn main() {
	fn vv() {
		println!("\n////")
	}
	fn z(s:&str) {
		let mut ret = Vec::new();
		zhscript::z(s, &mut ret);
		vv();
		for i in ret {
			println!("{}", i)
		}
	}
	
	let mut use_stdin = true;
	for (i, s) in env::args().enumerate() {
		println!("{}) {}", i, s);
		if i == 0 {
			continue
		}
		use_stdin = false;
		z(&s)
	}
	if use_stdin {
		loop {
			let mut s = String::new();
			vv();
			io::stdin().read_line(&mut s).expect("啊");
			s = s.trim().to_string();
			z(&s)
		}
	}
}
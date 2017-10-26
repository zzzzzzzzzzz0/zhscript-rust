#[derive(PartialEq, Debug)]
pub enum Id {
	No,
	Juhao,
	Kaiyinhao, Biyinhao,
	Eval,
	Print,
	Cr, Lf, Tab, Esc,
}

#[derive(Debug)]
pub struct Item {
	val:&'static str,
	val2:Id,
}

impl Item {
	pub fn val(&self) -> &str {
		self.val
	}
	pub fn val2(&self) -> &Id {
		&self.val2
	}
}

const fn new(val:&'static str, val2:Id) -> Item {
	Item {val:val, val2:val2}
}

pub static NO:Item = new("", Id::No);
pub static JUHAO:Item = new("。", Id::Juhao);
pub static KAIYINHAO:Item = new("“", Id::Kaiyinhao);
pub static BIYINHAO:Item = new("”", Id::Biyinhao);
pub static EVAL:Item = new("算术", Id::Eval);
pub static PRINT:Item = new("显示", Id::Print);
pub static CR:Item = new("回车", Id::Cr);
pub static LF:Item = new("换行", Id::Lf);
pub static TAB:Item = new("制表符", Id::Tab);
pub static ESC:Item = new("ESC", Id::Esc);
pub static ALL:[&'static Item; 9] = [
	&JUHAO,
	&KAIYINHAO, &BIYINHAO,
	&EVAL,
	&PRINT,
	&CR, &LF, &TAB, &ESC,
];

pub fn get(s:&str, i:usize, i3:&mut usize) -> Option<&'static Item> {
	for i2 in ALL.into_iter() {
		*i3 = 0;
		let mut i4 = i;
		loop {
			if *i3 >= i2.val().chars().count() {
				return Some(i2);
			}
			if i4 >= s.chars().count() {
				break;
			}
			if s.chars().nth(i4) != i2.val().chars().nth(*i3) {
				break;
			}
			*i3 += 1;
			i4 += 1;
		}
	}
	None
}
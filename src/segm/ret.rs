pub struct Ret<'a, 'b> {
	a:&'a mut Vec<String>,
	buf:&'b mut String,
	has_push:bool,
}

impl<'a, 'b> Ret<'a, 'b> {
	pub fn new(a:&'a mut Vec<String>, buf:&'b mut String) -> Self {
		Ret {a:a, buf:buf, has_push:false}
	}
	
	pub fn push(&mut self, s:&str) {
		self.has_push = true;
		self.buf.push_str(s)
	}
	
	pub fn one(&mut self, auto:bool) {
		if self.has_push || !auto {
			self.a.push(self.buf.clone());
		}
		self.buf.clear();
		self.has_push = false;
	}
}
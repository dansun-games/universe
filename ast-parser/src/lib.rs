#![feature(let_chains)]

use core::fmt;

struct ParseFail {
	at: usize,
}

pub enum AstNode {
	Value(String),
	Group {
		name: Option<String>,
		nodes: Vec<AstNode>,
	},
}

impl AstNode {
	pub fn len(&self) -> usize {
		match self {
			Self::Group{nodes, ..} => nodes.iter().map(|n| n.len()).sum(),
			Self::Value(v) => v.len(),
		}
	}

	pub fn value(&self) -> String {
		let mut ret = String::with_capacity(self.len());
		self.write_value(&mut ret).expect("Failed writing to string. Should never happen.");
		ret
	}

	pub fn write_value(&self, w: &mut impl fmt::Write) -> fmt::Result {
		match self {
			Self::Group{nodes, ..} => nodes.iter().try_for_each(|n| n.write_value(w))?,
			Self::Value(v) => w.write_str(&v)?,
		};

		fmt::Result::Ok(())
	}
}

trait Rule {
	fn name(&self) -> Option<&str>;
	fn parse(&self, input: &str) -> Result<AstNode, ParseFail>;
}

pub struct MatchStr {
	name: Option<String>,
	value: String,
}

impl MatchStr {
	pub fn new(name: Option<String>, value: String) -> MatchStr {
		assert!(!value.is_empty());
		MatchStr { name, value }
	}
}

impl Rule for MatchStr {
	fn name(&self) -> Option<&str> {
		self.name.as_ref().map(String::as_str)
	}

	fn parse(&self, input: &str) -> Result<AstNode, ParseFail> {
		let test_iter = self.value.char_indices();
		let mut input_iter = input.chars();

		for (i, test_char) in test_iter {
			match input_iter.next() {
				Some(v) if v == test_char => continue,
				_ => return Err(ParseFail { at: i }),
			};
		}

		Ok(AstNode::Value(self.value.clone()))
	}
}


pub struct MatchChar {
	name: Option<String>,
	value: char,
}

impl Rule for MatchChar {
	fn name(&self) -> Option<&str> {
		self.name.as_ref().map(String::as_str)
	}

	fn parse(&self, input: &str) -> Result<AstNode, ParseFail> {
		if input.starts_with(self.value) {
			let str = self.value.to_string();
			Ok(AstNode::Value(str))
		} else {
			Err(ParseFail { at: 0 })
		}
	}
}

pub struct Sequence {
	name: Option<String>,
	rules: Vec<Box<dyn Rule>>,
}

impl Rule for Sequence {
	fn name(&self) -> Option<&str> {
		self.name.as_ref().map(String::as_str)
	}

	fn parse(&self, input: &str) -> Result<AstNode, ParseFail> {
		let mut offset = 0;
		let mut nodes = Vec::with_capacity(self.rules.len());
		for rule in &self.rules {
			match rule.parse(&input[offset..]) {
				Ok(node) => {
					offset += node.len();
					nodes.push(node);
				},
				Err(mut fail) => {
					fail.at += offset;
					return Err(fail);
				},
			};
		}
		
		Ok(AstNode::Group { 
			name: self.name.clone(),
			nodes
		})
	}
}

pub struct Choice {
	name: Option<String>,
	rules: Vec<Box<dyn Rule>>,
}

impl Rule for Choice {
	fn name(&self) -> Option<&str> {
		self.name.as_ref().map(String::as_str)
	}

	fn parse(&self, input: &str) -> Result<AstNode, ParseFail> {
		let mut fails = Vec::with_capacity(self.rules.len());

		for rule in &self.rules {
			match rule.parse(&input) {
				Ok(node) => {
					return Ok(AstNode::Group { 
						name: self.name.clone(),
						nodes: vec![node],
					})
				},
				Err(fail) => fails.push(fail),
			};
		};

		Err(ParseFail {
			at: 0,
			//store the other fails as like a "cause" type of thing.. only for collections of alternates
		})
	}
}

pub struct Repetition {
	name: Option<String>,
	rule: Box<dyn Rule>,
	min_count: usize,
	max_count: Option<usize>,
}

impl Rule for Repetition {
	fn name(&self) -> Option<&str> {
		self.name.as_ref().map(String::as_str)
	}

	fn parse(&self, input: &str) -> Result<AstNode, ParseFail> {
		let mut nodes = Vec::new();
		let stop_fail = loop {
			if let Some(max) = self.max_count && nodes.len() == max {
				break None;
			}
			match self.rule.parse(input) {
				Ok(node) => nodes.push(node),
				Err(fail) => break Some(fail),
			};
		};

		if nodes.len() < self.min_count {
			let at = nodes.iter().map(|n| n.len()).sum();
			Err(ParseFail{ at })
		} else {
			Ok(AstNode::Group {
				name: self.name.clone(),
				nodes,
				//some kind of last error tracking here?
			})
		}
	}
}

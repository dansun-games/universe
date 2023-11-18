use std::ops::Range;

const GROUP_OPEN: char = '(';
const GROUP_CLOSE: char = ')';
const AND_SEPERATOR: char = '&';
const OR_SEPERATOR: char = '|';

pub enum CompOperator {
	Eq,
	Neq,
	Lt,
	Lteq,
	Gt,
	Gteq,
}

pub enum ConditionExpr<T> {
	Comparison(T, CompOperator, T),
	And(Vec<Self>),
	Or(Vec<Self>),
	Value(T),
}

impl<T> ConditionExpr<T> {
	pub fn and(&mut self) {
		// *self = ConditionExpr::And(vec![*self]);
	}
}

impl<'a, T> From<&'a str> for ConditionExpr<T>
where
	T: From<&'a str>,
{
	fn from(expr: &str) -> Self {
		let expr = parse_cond_expr(expr, false);
		unimplemented!()
	}
}


fn parse_cond_expr(input: &str, in_group: bool) -> ConditionExpr<&str> {
	enum ParseMode {
		And,
		Or,
		Value,
	}

	let mode = ParseMode::Value;
	let input = input.trim();

	let mut working: Option<ConditionExpr<&str>> = None;

	let mut start = 0;
	let mut consume_value = |cursor| -> ConditionExpr<&str> {
		let s = &input[start..cursor];
		start = cursor;
		ConditionExpr::Value(s.trim())
	};

	for (pos, char) in input.char_indices() {
		match char {
			GROUP_OPEN => {
				let grouped_cond = parse_cond_expr(input, true);
			},
			GROUP_CLOSE => {},
			AND_SEPERATOR => {
				// match working {
				//     Some(ConditionExpr::And(mut v)) => {
				//         v.push(consume_value(pos))
				//     }
				//     Some(ConditionExpr::Or(v)) => {

				//     }
				//     None => {}
				// };
			},
			OR_SEPERATOR => {},
			_ => {},
		};
	}


	todo!()
}

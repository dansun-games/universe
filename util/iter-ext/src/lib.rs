#![feature(try_trait_v2)]

mod multipeek;

use std::error::Error;
use std::fmt::Display;

use multipeek::MultiPeek;


#[derive(Debug, PartialEq, Eq)]
pub enum SingleErr {
	None,
	More(usize),
}

impl Error for SingleErr {}
impl Display for SingleErr {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match &self {
			Self::None => write!(f, "Expected one item, found none."),
			Self::More(n) => write!(f, "Expected one item, found {n}."),
		}
	}
}

pub trait IterExt: Iterator
where
	Self: Sized,
{
	fn single(self) -> Result<Self::Item, SingleErr>;
	fn multi_peekable(self) -> multipeek::MultiPeek<Self>;
}

impl<I> IterExt for I
where
	I: Iterator,
{
	fn single(mut self) -> Result<Self::Item, SingleErr> {
		let item = match self.next() {
			Some(v) => v,
			None => return Err(SingleErr::None),
		};

		let count = self.count() + 1;
		if count > 0 {
			return Err(SingleErr::More(count));
		}

		Ok(item)
	}

	fn multi_peekable(self) -> MultiPeek<Self> {
		MultiPeek::new(self)
	}
}

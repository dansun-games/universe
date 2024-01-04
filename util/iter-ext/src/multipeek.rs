use std::collections::VecDeque;
use std::ops::Try;

#[derive(Clone, Debug)]
pub struct MultiPeek<I: Iterator> {
	iter: I,
	buffer: VecDeque<I::Item>,
}

impl<I: Iterator> MultiPeek<I> {
	pub fn new(iter: I) -> MultiPeek<I> {
		MultiPeek {
			iter,
			buffer: VecDeque::new(),
		}
	}

	pub fn peek(&mut self, n: usize) -> impl Iterator<Item = &I::Item> {
		let missing = n.saturating_sub(self.buffer.len());
		if missing > 0 {
			self.buffer.extend(self.iter.by_ref().take(missing));
		}
		self.buffer.iter().take(n)
	}
}

// These functions should be the same as the ones in Iterator::peekable
impl<I: Iterator> Iterator for MultiPeek<I> {
	type Item = I::Item;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		self.buffer.pop_front().or_else(|| self.iter.next())
	}

	#[inline]
	fn count(self) -> usize {
		self.buffer.len() + self.iter.count()
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<I::Item> {
		let buff_count = self.buffer.len();
		if n < buff_count {
			self.buffer.drain(0..n);
			self.buffer.pop_front()
		} else {
			self.buffer.clear();
			self.iter.nth(n - buff_count)
		}
	}

	#[inline]
	fn last(mut self) -> Option<I::Item> {
		self.iter.last().or(self.buffer.pop_back())
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		let peek_len = self.buffer.len();
		let (lo, hi) = self.iter.size_hint();
		let lo = lo.saturating_add(peek_len);
		let hi = match hi {
			Some(x) => x.checked_add(peek_len),
			None => None,
		};
		(lo, hi)
	}

	#[inline]
	fn try_fold<B, F, R>(&mut self, init: B, mut f: F) -> R
	where
		Self: Sized,
		F: FnMut(B, Self::Item) -> R,
		R: Try<Output = B>,
	{
		let r = self.buffer.drain(..).try_fold(init, &mut f)?;
		self.iter.try_fold(r, &mut f)
	}

	#[inline]
	fn fold<Acc, Fold>(self, init: Acc, f: Fold) -> Acc
	where
		Fold: FnMut(Acc, Self::Item) -> Acc,
	{
		self.buffer.into_iter().chain(self.iter).fold(init, f)
	}
}

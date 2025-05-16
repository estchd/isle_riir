pub struct MxListEntry<T> {
	pub object: T,
	pub index: usize,
	pub prev: Option<usize>,
	pub next: Option<usize>,
}

impl<T> Default for MxListEntry<T>
where T: Default
{
	fn default() -> Self {
		Self {
			object: T::default(),
			index: 0,
			prev: None,
			next: None,
		}
	}
}

impl<T> MxListEntry<T> {
	pub fn new_prev(object: T, prev: Option<usize>, index: usize) -> Self {
		Self {
			object,
			index,
			prev,
			next: None,
		}
	}

	pub fn new_prev_next(object: T, prev: Option<usize>, next: Option<usize>, index: usize) -> Self {
		Self {
			object,
			index,
			prev,
			next,
		}
	}


}
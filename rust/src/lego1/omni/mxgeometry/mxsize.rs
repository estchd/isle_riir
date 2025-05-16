use std::fmt::{Debug, Formatter};

pub type MxSize16 = MxSize<i16>;
pub type MxSize32 = MxSize<i32>;

pub struct MxSize<T> {
	pub width: T,
	pub height: T,
}

impl<T> Clone for MxSize<T>
where T: Clone
{
	fn clone(&self) -> Self {
		Self {
			width: self.width.clone(),
			height: self.height.clone(),
		}
	}
}

impl<T> Copy for MxSize<T> where T: Copy {}

impl<T> Debug for MxSize<T>
where T: Debug
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MxSize")
			.field("width", &self.width)
			.field("height", &self.height)
			.finish()
	}
}

impl<T> Default for MxSize<T>
where T: Default {
	fn default() -> Self {
		Self {
			width: T::default(),
			height: T::default(),
		}
	}
}

impl<T> MxSize<T> {
	pub fn new(width: T, height: T) -> Self {
		Self {
			width,
			height
		}
	}
}
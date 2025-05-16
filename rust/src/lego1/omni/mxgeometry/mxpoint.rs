use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub type MxPoint16 = MxPoint<i16>;
pub type MxPoint32 = MxPoint<i32>;

pub struct MxPoint<T>
{
	pub x: T,
	pub y: T
}

impl<T> Clone for MxPoint<T>
where T: Clone
{
	fn clone(&self) -> Self {
		Self {
			x: self.x.clone(),
			y: self.y.clone(),
		}
	}
}

impl<T> Copy for MxPoint<T>
where T: Copy {}

impl<T> Debug for MxPoint<T>
where T: Debug
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MxPoint")
			.field("x", &self.x)
			.field("y", &self.y)
			.finish()
	}
}

impl<T> MxPoint<T> {
	pub fn new(x: T, y: T) -> Self {
		Self {
			x,
			y
		}
	}
}

impl<T> Default for MxPoint<T>
where T: Default
{
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}
}

impl<T> Add<&MxPoint<T>> for &MxPoint<T>
where T: Add<Output = T> + Copy
{
	type Output = MxPoint<T>;

	fn add(self, rhs: &MxPoint<T>) -> Self::Output {
		MxPoint::<T> {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> AddAssign<&MxPoint<T>> for MxPoint<T>
where T: AddAssign<T> + Copy
{
	fn add_assign(&mut self, rhs: &MxPoint<T>) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl<T> Sub<&MxPoint<T>> for &MxPoint<T>
where T: Sub<Output = T> + Copy
{
	type Output = MxPoint<T>;

	fn sub(self, rhs: &MxPoint<T>) -> Self::Output {
		MxPoint::<T> {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> SubAssign<&MxPoint<T>> for MxPoint<T>
where T: SubAssign<T> + Copy
{
	fn sub_assign(&mut self, rhs: &MxPoint<T>) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}
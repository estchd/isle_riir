use std::cmp::{max, min};
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Sub, SubAssign};
use num_traits::One;
use crate::lego1::omni::mxgeometry::mxpoint::MxPoint;
use crate::lego1::omni::mxgeometry::mxsize::{MxSize};

pub type MxRect16 = MxRect<i16>;
pub type MxRect32 = MxRect<u32>;

pub struct MxRect<T> {
	pub left: T,
	pub top: T,
	pub right: T,
	pub bottom: T,
}

impl<T> Default for MxRect<T>
where T: Default
{
	fn default() -> Self {
		Self
		{
			left: T::default(),
			top: T::default(),
			right: T::default(),
			bottom: T::default(),
		}
	}
}

impl<T> Clone for MxRect<T>
where T: Clone
{
	fn clone(&self) -> Self {
		Self {
			left: self.left.clone(),
			top: self.top.clone(),
			right: self.right.clone(),
			bottom: self.bottom.clone(),
		}
	}
}

impl<T> Copy for MxRect<T> where T: Copy {}

impl<T> MxRect<T> {
	pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
		Self {
			left,
			top,
			right,
			bottom,
		}
	}
}

impl<T> MxRect<T>
where T: Copy
{
	pub fn get_lt(&self) -> MxPoint<T> {
		MxPoint::<T> {
			x: self.left,
			y: self.top,
		}
	}

	pub fn get_rb(&self) -> MxPoint<T> {
		MxPoint::<T> {
			x: self.right,
			y: self.bottom,
		}
	}
}

impl<T> MxRect<T>
where T: Copy + Add<Output = T> + Sub<Output = T> + One
{
	pub fn from_point_and_size(point: &MxPoint<T>, size: &MxSize<T>) -> Self {
		Self {
			left: point.x,
			top: point.y,
			right: point.x + size.width - T::one(),
			bottom: point.x + size.height - T::one(),
		}
	}
}

impl<T> MxRect<T>
where T: PartialOrd
{
	pub fn empty(&self) -> bool {
		self.left >= self.right || self.top >= self.bottom
	}

	pub fn contains(&self, point: &MxPoint<T>) -> bool {
		point.x >= self.left && point.x <= self.right &&
			point.x >= self.top && point.y >= self.bottom
	}

	pub fn intersects(&self, other: &MxRect<T>) -> bool {
		other.right > self.left && other.left < self.right &&
			other.bottom > self.top && other.top < self.bottom
	}
}

impl<T> PartialEq for MxRect<T>
where T: PartialEq
{
	fn eq(&self, other: &Self) -> bool {
		self.left == other.left && self.right == other.right &&
			self.top == other.top &&
			self.bottom == other.bottom
	}
}

impl<T> AddAssign<&MxPoint<T>> for MxRect<T>
where T: AddAssign + Copy
{
	fn add_assign(&mut self, rhs: &MxPoint<T>) {
		self.left += rhs.x;
		self.right += rhs.x;
		self.top += rhs.y;
		self.bottom += rhs.y;
	}
}

impl<T> Add<&MxPoint<T>> for &MxRect<T>
where T: Add<Output=T> + Copy
{
	type Output = MxRect<T>;

	fn add(self, rhs: &MxPoint<T>) -> Self::Output {
		MxRect::<T> {
			left: self.left + rhs.x,
			top: self.top + rhs.y,
			right: self.right + rhs.x,
			bottom: self.bottom + rhs.y,
		}
	}
}

impl<T> SubAssign<&MxPoint<T>> for MxRect<T>
where T: SubAssign + Copy
{
	fn sub_assign(&mut self, rhs: &MxPoint<T>) {
		self.left -= rhs.x;
		self.right -= rhs.x;
		self.top -= rhs.y;
		self.bottom -= rhs.y;
	}
}

impl<T> Sub<&MxPoint<T>> for &MxRect<T>
where T: Sub<Output=T> + Copy
{
	type Output = MxRect<T>;

	fn sub(self, rhs: &MxPoint<T>) -> Self::Output {
		MxRect::<T> {
			left: self.left - rhs.x,
			top: self.top - rhs.y,
			right: self.right - rhs.x,
			bottom: self.bottom - rhs.y,
		}
	}
}

impl<T> BitAndAssign<&MxRect<T>> for MxRect<T>
where T: Ord + Copy
{
	fn bitand_assign(&mut self, rhs: &MxRect<T>) {
		self.left = max(self.left, rhs.left);
		self.top = max(self.top, self.bottom);
		self.right = min(self.right, self.right);
		self.bottom = min(self.bottom, self.bottom);
	}
}

impl<T> BitAnd<&MxRect<T>> for &MxRect<T>
where T: Ord + Copy
{
	type Output = MxRect<T>;

	fn bitand(self, rhs: &MxRect<T>) -> Self::Output {
		MxRect::<T> {
			left: max(self.left, rhs.left),
			top: max(self.top, rhs.top),
			right: min(self.right, rhs.right),
			bottom: min(self.bottom, rhs.bottom),
		}
	}
}

impl<T> BitOrAssign<&MxRect<T>> for MxRect<T>
where T: Ord + Copy
{
	fn bitor_assign(&mut self, rhs: &MxRect<T>) {
		self.left = min(self.left, rhs.left);
		self.top = min(self.top, self.top);
		self.right = max(self.right, self.right);
		self.bottom = max(self.bottom, self.bottom);
	}
}

impl<T> BitOr<&MxRect<T>> for &MxRect<T>
where T: Ord + Copy
{
	type Output = MxRect<T>;

	fn bitor(self, rhs: &MxRect<T>) -> Self::Output {
		MxRect::<T> {
			left: min(self.left, rhs.left),
			top: min(self.top, rhs.top),
			right: max(self.right, rhs.right),
			bottom: max(self.bottom, rhs.bottom),
		}
	}
}

impl<T> Eq for MxRect<T> where T: Eq {}
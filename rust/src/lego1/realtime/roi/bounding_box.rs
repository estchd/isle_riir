use crate::lego1::realtime::vector::Vector3;

#[derive(Copy, Clone, Debug, Default)]
pub struct BoundingBox {
	pub min: Vector3,
	pub max: Vector3
}

impl BoundingBox {
	pub fn new() -> Self {
		Self {
			min: Vector3::zeros(),
			max: Vector3::zeros(),
		}
	}

	pub fn create(min: Vector3, max: Vector3) -> Self {
		Self {
			min,
			max,
		}
	}

	pub fn min(&self) -> &Vector3 {
		&self.min
	}

	pub fn max(&self) -> &Vector3 {
		&self.max
	}

	pub fn min_mut(&mut self) -> &mut Vector3 {
		&mut self.min
	}

	pub fn max_mut(&mut self) -> &mut Vector3 {
		&mut self.max
	}
}
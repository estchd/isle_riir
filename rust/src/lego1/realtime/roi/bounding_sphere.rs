use crate::lego1::realtime::vector::Vector3;

#[derive(Copy, Clone, Debug, Default)]
pub struct BoundingSphere {
	pub center: Vector3,
	pub radius: f32
}

impl BoundingSphere {
	pub fn new() -> Self {
		Self {
			center: Vector3::zeros(),
			radius: 0.0,
		}
	}
	
	pub fn create(center: Vector3, radius: f32) -> Self {
		Self {
			center,
			radius,
		}
	}

	pub fn center(&self) -> &Vector3 {
		&self.center
	}
	
	pub fn center_mut(&mut self) -> &mut Vector3 {
		&mut self.center
	}
	
	pub fn radius(&self) -> f32 {
		self.radius
	}

	pub fn radius_mut(&mut self) -> &mut f32 {
		&mut self.radius
	}
}
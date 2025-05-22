use std::ops::Not;
use std::sync::Weak;
use nalgebra::{convert_ref, try_convert_ref, Const, Dyn, MatrixView, MatrixViewMut, U1, U3, U4};
use crate::lego1::realtime::matrix::{Matrix4, Matrix4Vec3Ref, Matrix4Vec3RefMut};
use crate::lego1::realtime::roi::{CompoundObject, ROIStruct, ROI};
use crate::lego1::realtime::roi::bounding_box::BoundingBox;
use crate::lego1::realtime::roi::bounding_sphere::BoundingSphere;
use crate::lego1::realtime::roi::lod_object::LODObject;
use crate::lego1::realtime::vector::{Vector3, Vector4};

pub trait OrientableRoi: ROI {
	fn get_world_velocity(&self) -> &Vector3;

	fn get_world_velocity_mut(&mut self) -> &mut Vector3;

	fn set_world_velocity(&mut self, world_velocity: &Vector3);

	fn update_world_velocity(&mut self);

	fn get_world_bounding_box(&self) -> &BoundingBox;
	fn get_world_bounding_box_mut(&mut self) -> &mut BoundingBox;

	fn get_world_bounding_sphere(&self) -> &BoundingSphere;
	fn get_world_bounding_sphere_mut(&mut self) -> &mut BoundingSphere;

	fn get_local_2_world(&self) -> &Matrix4;

	fn get_local_2_world_mut(&mut self) -> &mut Matrix4;

	fn set_local_2_world(&mut self, local_2_world: &Matrix4, update_world_data: bool);

	fn get_world_position(&self) -> Matrix4Vec3Ref;

	fn get_world_position_mut(&mut self) -> Matrix4Vec3RefMut;

	fn get_world_direction(&self) -> Matrix4Vec3Ref;

	fn get_world_direction_mut(&mut self) -> Matrix4Vec3RefMut;

	fn get_world_up(&self) -> Matrix4Vec3Ref;

	fn get_world_up_mut(&mut self) -> Matrix4Vec3RefMut;

	fn get_parent_roi(&self) -> Option<Weak<dyn OrientableRoi>>;

	fn set_parent_roi(&mut self, parent_roi: Option<Weak<dyn OrientableRoi>>);

	fn update_world_data(&mut self, transform: Option<&Matrix4>, with_children: bool);

	fn update_world_bounding_volumes(&mut self);

	fn update_transform_relative_to_parent(&mut self, transform: &Matrix4);

	fn get_local_transform(&self, transform: &mut Matrix4);

	fn set_needs_world_data_update(&mut self, needs_world_data_update: bool);
}

pub struct OrientableRoiStruct {
	pub base: ROIStruct,
	pub local_2_world: Matrix4,
	pub world_bounding_box: BoundingBox,
	pub unknown_0x80: BoundingBox,
	pub world_bounding_sphere: BoundingSphere,
	pub world_velocity: Vector3,
	pub parent_roi: Option<Weak<dyn OrientableRoi>>,
	pub needs_world_data_update: bool
}

impl OrientableRoiStruct {
	pub fn new() -> Self {
		let world_bounding_box = BoundingBox::create(
			Vector3::from_element(888888.8),
			Vector3::from_element(-888888.8)
		);

		let unknown_0x80 = BoundingBox::new();

		let world_bounding_sphere = BoundingSphere::create(
			Vector3::zeros(),
			0.0
		);

		let world_velocity = Vector3::zeros();

		let local_2_world = Matrix4::identity();

		Self {
			base: ROIStruct::new(),
			local_2_world,
			world_bounding_box,
			unknown_0x80,
			world_bounding_sphere,
			world_velocity,
			parent_roi: None,
			needs_world_data_update: true,
		}
	}

	fn get_world_velocity(&self) -> &Vector3 {
		&self.world_velocity
	}

	fn get_world_velocity_mut(&mut self) -> &mut Vector3 {
		&mut self.world_velocity
	}

	fn set_world_velocity(&mut self, world_velocity: &Vector3) {
		self.world_velocity = *world_velocity;
	}

	fn update_world_velocity(&mut self) {

	}

	fn get_world_bounding_box(&self) -> &BoundingBox {
		&self.world_bounding_box
	}
	fn get_world_bounding_box_mut(&mut self) -> &mut BoundingBox {
		&mut self.world_bounding_box
	}

	fn get_world_bounding_sphere(&self) -> &BoundingSphere {
		&self.world_bounding_sphere
	}

	fn get_world_bounding_sphere_mut(&mut self) -> &mut BoundingSphere {
		&mut self.world_bounding_sphere
	}

	fn get_local_2_world(&self) -> &Matrix4 {
		&self.local_2_world
	}

	fn get_local_2_world_mut(&mut self) -> &mut Matrix4 {
		&mut self.local_2_world
	}

	fn set_local_2_world(&mut self, local_2_world: &Matrix4, update_world_data: bool) {
		self.local_2_world = *local_2_world;

		if !update_world_data {
			self.needs_world_data_update = true;
		}
		else {
			self.update_world_bounding_volumes();
			self.update_world_velocity();
		}
	}

	fn get_world_position(&self) -> Matrix4Vec3Ref {
		self.local_2_world.fixed_view::<3, 1>(0, 3)
	}

	fn get_world_position_mut(&mut self) -> Matrix4Vec3RefMut {
		self.local_2_world.fixed_view_mut::<3, 1>(0, 3)
	}

	fn get_world_direction(&self) -> Matrix4Vec3Ref {
		self.local_2_world.fixed_view::<3, 1>(0, 2)
	}

	fn get_world_direction_mut(&mut self) -> Matrix4Vec3RefMut {
		self.local_2_world.fixed_view_mut::<3, 1>(0, 2)
	}

	fn get_world_up(&self) -> Matrix4Vec3Ref {
		self.local_2_world.fixed_view::<3, 1>(0, 1)
	}

	fn get_world_up_mut(&mut self) -> Matrix4Vec3RefMut {
		self.local_2_world.fixed_view_mut::<3, 1>(0, 1)
	}

	fn get_parent_roi(&self) -> Option<Weak<dyn OrientableRoi>> {
		self.parent_roi.clone()
	}

	fn set_parent_roi(&mut self, parent_roi: Option<Weak<dyn OrientableRoi>>) {
		self.parent_roi = parent_roi;
	}

	fn update_world_data(&mut self, transform: Option<&Matrix4>, with_children: bool) {
		if let Some(transform) = transform {
			let matrix = self.local_2_world.clone_owned();

			self.local_2_world = transform * matrix;
		}

		self.update_world_bounding_volumes();
		self.update_world_velocity();
	}

	fn update_world_bounding_volumes(&mut self) {

	}

	fn update_transform_relative_to_parent(&mut self, transform: &Matrix4) {
		let local_2_world = transform.cast::<f64>();
		let local_2_parent = self.local_2_world.cast::<f64>();

		let local_inverse = local_2_parent.try_inverse().unwrap();

		let parent_2_world = local_inverse * local_2_world;

		let mat = parent_2_world.cast::<f32>();

		self.update_world_data(Some(&mat), true);
	}

	fn get_local_transform(&self, transform: &mut Matrix4) {
		if self.parent_roi.is_none() {
			*transform = self.local_2_world;
		}

		let parent = self.parent_roi
			.as_ref()
			.unwrap()
			.upgrade()
			.expect("Parent ROI already dropped");

		let local_2_parent = parent.get_local_2_world().cast::<f64>();

		let local_inverse = local_2_parent.try_inverse().unwrap();

		let mat = local_inverse.cast::<f32>();

		*transform = self.local_2_world * mat;
	}

	fn set_needs_world_data_update(&mut self, needs_world_data_update: bool) {
		self.needs_world_data_update = needs_world_data_update;
	}
}

pub fn calc_world_bounding_volumes(
	modelling_sphere: &BoundingSphere,
	local_2_world: &Matrix4,
	world_bounding_box: &mut BoundingBox,
	world_bounding_sphere: &mut BoundingSphere
) {
	world_bounding_sphere.center = (local_2_world * modelling_sphere.center.push(1.0)).fixed_view::<3, 1>(0, 0).into();

	world_bounding_sphere.radius = modelling_sphere.radius;

	world_bounding_box.min = world_bounding_sphere.center.add_scalar(-world_bounding_sphere.radius);
	world_bounding_box.max = world_bounding_sphere.center.add_scalar(world_bounding_sphere.radius);
}
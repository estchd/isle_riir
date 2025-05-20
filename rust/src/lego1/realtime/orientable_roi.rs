use std::ops::Not;
use std::sync::Weak;
use nalgebra::{convert_ref, try_convert_ref, Const, Dyn, MatrixView, MatrixViewMut, U1, U3, U4};
use crate::lego1::realtime::matrix::{Matrix4, Matrix4Vec3Ref, Matrix4Vec3RefMut};
use crate::lego1::realtime::roi::{CompoundObject, ROIStruct, ROI};
use crate::lego1::realtime::roi::bounding_box::BoundingBox;
use crate::lego1::realtime::roi::bounding_sphere::BoundingSphere;
use crate::lego1::realtime::roi::lod_object::LODObject;
use crate::lego1::realtime::vector::Vector3;

pub trait OrientableRoi: ROI {
	fn v_table_0x14(&mut self);

	fn update_world_bounding_volumes(&mut self);

	fn v_table_0x1c(&mut self);

	fn set_local_transform(&mut self, transform: &Matrix4);

	fn v_table_0x24(&mut self, transform: &Matrix4);

	fn update_world_data(&mut self, transform: &Matrix4);

	fn update_world_velocity(&mut self);

	fn wrapped_set_local_transform(&mut self, transform: &Matrix4);

	fn update_transformation_relative_to_parent(&mut self, transform: &Matrix4);

	fn wrapped_v_table_0x24(&mut self, transform: &Matrix4);

	fn get_local_transform(&self, out: &mut Matrix4);

	fn fun_100a58f0(&mut self, transform: &Matrix4);

	fn fun_100a5a30(&mut self, world_velocity: &Vector3);

	fn get_local_2_world(&self) -> &Matrix4;

	fn get_world_position(&self) -> Vector3;

	fn get_world_direction(&self) -> Vector3;

	fn get_world_up(&self) -> Vector3;

	fn get_parent_roi(&self) -> &dyn OrientableRoi;

	fn set_parent_rio(&mut self, parent_roi: Option<Weak<dyn OrientableRoi>>);

	fn toggle_unknown_0xd8(&mut self, enable: bool);
}

pub struct OrientableRoiStruct {
	base: ROIStruct,
	local_2_world: Matrix4,
	world_bounding_box: BoundingBox,
	unknown_0x80: BoundingBox,
	world_bounding_sphere: BoundingSphere,
	world_velocity: Vector3,
	parent_roi: Option<Weak<dyn OrientableRoi>>,
	unknown_0xd8: i32
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

		let mut new = Self {
			base: ROIStruct::new(),
			local_2_world,
			world_bounding_box,
			unknown_0x80,
			world_bounding_sphere,
			world_velocity,
			parent_roi: None,
			unknown_0xd8: 0,
		};

		new.toggle_unknown_0xd8(true);

		new
	}

	pub fn get_lods(&self) -> &() {
		self.base.get_lods()
	}

	pub fn get_lods_mut(&mut self) -> &mut () {
		self.base.get_lods_mut()
	}

	pub fn get_lod(&self, index: i32) -> &dyn LODObject {
		self.base.get_lod(index)
	}

	pub fn get_lod_mut(&mut self, index: i32) -> &mut dyn LODObject {
		self.base.get_lod_mut(index)
	}

	pub fn get_lod_count(&self) -> i32 {
		self.base.get_lod_count()
	}

	pub fn get_comp(&self) -> &CompoundObject {
		self.base.get_comp()
	}

	pub fn get_comp_mut(&mut self) -> &mut CompoundObject {
		self.base.get_comp_mut()
	}

	pub fn get_visibility(&self) -> bool {
		self.base.get_visibility()
	}

	pub fn set_visibility(&mut self, visible: bool) {
		self.base.set_visibility(visible)
	}

	pub fn get_world_velocity(&self) -> &Vector3 {
		todo!()
	}

	pub fn get_world_bounding_box(&self) -> &BoundingBox {
		todo!()
	}

	pub fn get_world_bounding_sphere(&self) -> &BoundingSphere {
		todo!()
	}

	pub fn v_table_0x14(&mut self) {
		todo!()
	}

	pub fn v_table_0x1c(&mut self) {
		todo!()
	}

	pub fn set_local_transform(&mut self, transform: &Matrix4) {
		todo!()
	}

	pub fn v_table_0x24(&mut self, transform: &Matrix4) {
		todo!()
	}

	pub fn update_world_data(&mut self, transform: &Matrix4) {
		todo!()
	}

	pub fn update_world_velocity(&mut self) {
		todo!()
	}

	pub fn wrapped_set_local_transform(&mut self, transform: &Matrix4) {
		self.set_local_transform(transform);
	}

	pub fn update_transformation_relative_to_parent(&mut self, transform: &Matrix4) {
		let local_2_world = transform.cast::<f64>();
		let local_2_parent = self.local_2_world.cast::<f64>();

		let local_inverse = local_2_parent.try_inverse().unwrap();

		let parent_2_world = local_inverse * local_2_world;

		let mat: Matrix4 = parent_2_world.cast();

		self.update_world_data(&mat);
	}

	pub fn wrapped_v_table_0x24(&mut self, transform: &Matrix4) {
		self.v_table_0x24(transform)
	}

	pub fn get_local_transform(&self, transform: &mut Matrix4) {
		if self.parent_roi.is_none() {
			*transform = self.local_2_world;
			return;
		}

		let parent = self.parent_roi.as_ref().unwrap().upgrade();

		if parent.is_none() {
			panic!("tried to access dropped parent");
		}

		let parent = parent.unwrap();

		let local_2_parent = parent.get_local_2_world().cast::<f64>();

		let local_inverse = local_2_parent.try_inverse().unwrap().cast::<f32>();

		*transform = self.local_2_world * local_inverse;
	}

	pub fn fun_100a58f0(&mut self, transform: &Matrix4) {
		self.local_2_world = transform.clone_owned();
		self.toggle_unknown_0xd8(true);
	}

	pub fn fun_100a5a30(&mut self, world_velocity: &Vector3) {
		todo!()
	}

	pub fn get_local_2_world(&self) -> &Matrix4 {
		&self.local_2_world
	}

	pub fn get_local_2_world_mut(&mut self) -> &mut Matrix4 {
		&mut self.local_2_world
	}

	pub fn get_world_position(&self) -> Matrix4Vec3Ref {
		self.local_2_world.fixed_view::<3, 1>(0, 3)
	}

	pub fn get_world_position_mut(&mut self) -> Matrix4Vec3RefMut {
		self.local_2_world.fixed_view_mut::<3, 1>(0, 3)
	}

	pub fn get_world_direction(&self) -> Matrix4Vec3Ref {
		self.local_2_world.fixed_view::<3, 1>(0, 2)
	}

	pub fn get_world_direction_mut(&mut self) -> Matrix4Vec3RefMut {
		self.local_2_world.fixed_view_mut::<3, 1>(0, 2)
	}

	pub fn get_world_up(&self) -> Matrix4Vec3Ref {
		self.local_2_world.fixed_view::<3, 1>(0, 1)
	}

	pub fn get_world_up_mut(&mut self) -> Matrix4Vec3RefMut {
		self.local_2_world.fixed_view_mut::<3, 1>(0, 1)
	}

	pub fn get_parent_roi(&self) -> Option<Weak<dyn OrientableRoi>> {
		self.parent_roi.clone()
	}

	pub fn set_parent_roi(&mut self, parent_roi: Option<Weak<dyn OrientableRoi>>) {
		self.parent_roi = parent_roi
	}

	pub fn toggle_unknown_0xd8(&mut self, enable: bool) {
		if enable {
			self.unknown_0xd8 |= 0x01 | 0x02;
		}
		else {
			self.unknown_0xd8 &= !0x01;
		}
	}
}
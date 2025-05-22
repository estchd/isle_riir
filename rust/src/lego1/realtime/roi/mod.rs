use std::collections::LinkedList;
use crate::lego1::realtime::lod_list::LodListBase;
use crate::lego1::realtime::roi::bounding_box::BoundingBox;
use crate::lego1::realtime::roi::bounding_sphere::BoundingSphere;
use crate::lego1::realtime::roi::lod_object::LODObject;
use crate::lego1::realtime::vector::Vector3;

pub mod bounding_box;
pub mod bounding_sphere;
pub mod lod_object;

pub type CompoundObject = LinkedList<Box<dyn ROI>>;

pub type ROIList = Vec<Box<dyn ROI>>;

pub type IntList = Vec<i32>;

pub trait ROI {
	fn intrinsic_importance(&self) -> f32;

	fn get_world_velocity(&self) -> &Vector3;

	fn get_world_bounding_box(&self) -> &BoundingBox;
	fn get_world_bounding_sphere(&self) -> &BoundingSphere;

	fn get_lods(&self) -> &LodListBase;

	fn get_lods_mut(&mut self) -> &mut LodListBase;

	fn get_lod(&self, index: i32) -> &dyn LODObject;

	fn get_lod_mut(&mut self, index: i32) -> &mut dyn LODObject;

	fn get_lod_count(&self) -> i32;

	fn get_comp(&self) -> &CompoundObject;

	fn get_comp_mut(&mut self) -> &mut CompoundObject;

	fn get_visibility(&self) -> bool;

	fn set_visibility(&mut self, visible: bool);
}

pub struct ROIStruct {
	pub comp: (),
	pub lods: (),
	pub visible: bool,
}

impl ROIStruct {
	pub fn new() -> Self {
		Self {
			comp: (),
			lods: (),
			visible: true,
		}
	}

	pub fn get_lods(&self) -> &() {
		&self.lods
	}

	pub fn get_lods_mut(&mut self) -> &mut () {
		&mut self.lods
	}

	pub fn get_lod(&self, index: i32) -> &dyn LODObject {
		todo!()
	}

	pub fn get_lod_mut(&mut self, index: i32) -> &mut dyn LODObject {
		todo!()
	}

	pub fn get_lod_count(&self) -> i32 {
		todo!()
	}

	pub fn get_comp(&self) -> &CompoundObject {
		todo!()
	}

	pub fn get_comp_mut(&mut self) -> &mut CompoundObject {
		todo!()
	}

	pub fn get_visibility(&self) -> bool {
		self.visible
	}

	pub fn set_visibility(&mut self, visible: bool) {
		self.visible = visible;
	}
}
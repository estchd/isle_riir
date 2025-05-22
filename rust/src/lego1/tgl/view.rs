use crate::lego1::realtime::vector::{Vector3, Vector4};
use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::{ProjectionType, TglResult};
use crate::lego1::tgl::camera::Camera;
use crate::lego1::tgl::group::Group;
use crate::lego1::tgl::light::Light;

pub trait View: Object {
	fn add(&mut self, light: &dyn Light) -> TglResult;

	fn remove(&mut self, light: &dyn Light) -> TglResult;

	fn set_camera(&mut self, camera: &dyn Camera) -> TglResult;

	fn set_projection(&mut self, projection_type: ProjectionType) -> TglResult;

	fn set_frustrum(&mut self,
	                front_clipping_distance: f32,
	                back_clipping_distance: f32,
	                degrees: f32
	) -> TglResult;

	fn set_background_color(&mut self, r: f32, g: f32, b: f32) -> TglResult;

	fn clear(&mut self) -> TglResult;

	fn render(&mut self, group: &dyn Group) -> TglResult;

	fn force_update(&mut self,
	                x: u32,
	                y: u32,
	                width: u32,
	                height: u32
	) -> TglResult;

	fn transform_world_to_screen(&mut self, world: &Vector3, screen: &mut Vector4) -> TglResult;

	fn transform_screen_to_world(&mut self, screen: &Vector4, world: &mut Vector3) -> TglResult;

	fn pick(&mut self, x: u32, y: u32, groups_to_pick_from: &[&dyn Group],
	        picked_groups: &mut [&dyn Group]
	) -> TglResult;
}
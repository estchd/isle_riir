use crate::lego1::tgl::{DeviceDirect3DCreateData, DeviceDirectDrawCreateData, LightType, PaletteEntry, TglResult};
use crate::lego1::tgl::camera::Camera;
use crate::lego1::tgl::device::Device;
use crate::lego1::tgl::group::Group;
use crate::lego1::tgl::light::Light;
use crate::lego1::tgl::mesh_builder::MeshBuilder;
use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::renderer::Renderer;
use crate::lego1::tgl::texture::Texture;
use crate::lego1::tgl::view::View;

pub struct RendererImpl {
	data:
}

impl Object for RendererImpl {
	fn implementation_data_ptr(&self) -> () {
		todo!()
	}

	fn implementation_data_ptr_mut(&mut self) -> () {
		todo!()
	}
}

impl Renderer for RendererImpl {
	fn create_device_with_draw_create_data(&mut self, data: &DeviceDirectDrawCreateData) -> () {
		todo!()
	}

	fn create_device_with_3d_create_data(&mut self, data: &DeviceDirect3DCreateData) -> () {
		todo!()
	}

	fn create_view(&mut self, device: &dyn Device, camera: &dyn Camera, x: u32, y: u32, width: u32, height: u32) -> Option<Box<dyn View>> {
		todo!()
	}

	fn create_camera(&mut self) -> Option<Box<dyn Camera>> {
		todo!()
	}

	fn create_light(&mut self, light_type: LightType, r: f32, g: f32, b: f32) -> Option<Box<dyn Light>> {
		todo!()
	}

	fn create_group(&mut self, parent: &dyn Group) -> Option<Box<dyn Group>> {
		todo!()
	}

	fn create_mesh_builder(&mut self) -> Option<Box<dyn MeshBuilder>> {
		todo!()
	}

	fn create_texture_with_params(&mut self, width: i32, height: i32, bits_per_texel: i32, texels: &[u8], texels_are_persistent: i32, palette_entry_count: i32, entries: &[PaletteEntry]) -> Option<Box<dyn Texture>> {
		todo!()
	}

	fn create_texture(&mut self) -> Option<Box<dyn Texture>> {
		todo!()
	}

	fn set_texture_default_shade_count(&mut self, value: u32) -> TglResult {
		todo!()
	}

	fn set_texture_default_color_count(&mut self, value: u32) -> TglResult {
		todo!()
	}
}

pub fn create_renderer() -> Option<Box<dyn Renderer>> {
	let test = Direct3

	todo!()
}
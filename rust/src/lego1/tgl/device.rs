use windows::Win32::Graphics::Gdi::HDC;
use crate::lego1::tgl::{ColorModel, ShadingModel, TglResult};
use crate::lego1::tgl::object::Object;

pub trait Device: Object {
	fn get_width(&self) -> u32;

	fn get_height(&self) -> u32;

	fn set_color_model(&mut self, model: ColorModel) -> TglResult;

	fn set_shading_model(&mut self, model: ShadingModel) -> TglResult;

	fn set_shade_count(&mut self, value: u32) -> TglResult;

	fn set_dither(&mut self, value: i32) -> TglResult;

	fn update(&mut self) -> TglResult;

	fn handle_activate(&mut self, value: u16);

	fn handle_paint(&mut self, hdc: HDC);
}
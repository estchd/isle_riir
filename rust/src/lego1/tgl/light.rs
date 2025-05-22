use crate::lego1::realtime::matrix::Matrix4;
use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::TglResult;

pub trait Light: Object {
	fn set_transformation(&mut self, transformation: &Matrix4) -> TglResult;
	
	fn set_color(&mut self, r: f32, g: f32, b: f32) -> TglResult;
}
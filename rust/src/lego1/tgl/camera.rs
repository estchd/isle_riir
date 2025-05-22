use crate::lego1::realtime::matrix::Matrix4;
use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::TglResult;

pub trait Camera: Object {
	fn set_transformation(&mut self, transformation: &Matrix4) -> TglResult;
}
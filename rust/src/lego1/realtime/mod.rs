use crate::lego1::realtime::matrix::Matrix4;
use crate::lego1::realtime::vector::Vector3;

pub mod lod_list;
pub mod roi;
pub mod orientable_roi;
pub mod vector;
pub mod matrix;
pub mod realtime_view;

pub fn calc_local_transform(
	position: &Vector3,
	direction: &Vector3,
	up: &Vector3,
	out: &mut Matrix4
) {
	let x_axis = Vector3::zeros();
	let y_axis = Vector3::zeros();
	let z_axis = Vector3::zeros();

	let z_axis = direction.normalize();
	let y_axis = up.normalize();

	let x_axis = y_axis.cross(&z_axis).normalize();

	let y_axis = z_axis.cross(&x_axis).normalize();

	out.set_column(0, &x_axis.push(0.0));
	out.set_column(1, &y_axis.push(0.0));
	out.set_column(2, &z_axis.push(0.0));
	out.set_column(3, &position.push(1.0));
}

pub use orientable_roi::calc_world_bounding_volumes;
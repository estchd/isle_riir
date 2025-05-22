use crate::lego1::realtime::vector::{Vector2, Vector3};
use crate::lego1::tgl::mesh::Mesh;
use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::{ShadingModel, TglResult};

pub trait MeshBuilder: Object {
	fn create_mesh(
		&mut self,
		face_count: u32,
		vertex_count: u32,
		positions: &[Vector3],
		normals: &[Vector3],
		texture_coordinates: &[Vector2],
		face_indices: &[[u32; 3]],
		texture_indices: &[[u32; 3]],
		shading_model: ShadingModel
	) -> Option<Box<dyn Mesh>>;

	fn get_bounding_box(&self, min: &mut Vector3, max: &mut Vector3) -> TglResult;

	fn clone(&self) -> Option<Box<dyn Mesh>>;
}
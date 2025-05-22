use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::{ShadingModel, TextureMappingMode, TglResult};
use crate::lego1::tgl::mesh_builder::MeshBuilder;
use crate::lego1::tgl::texture::Texture;

pub trait Mesh: Object {
	fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> TglResult;

	fn set_texture(&mut self, texture: &dyn Texture) -> TglResult;

	fn get_texture(&self, texture: &mut dyn Texture) -> TglResult;

	fn set_texture_mapping_mode(&mut self, mode: TextureMappingMode) -> TglResult;

	fn set_shading_model(&mut self, model: ShadingModel) -> TglResult;

	fn deep_clone(&self, mesh_builder: &mut dyn MeshBuilder) -> Option<Box<dyn Mesh>>;

	fn shallow_clone(&self, mesh_builder: &mut dyn MeshBuilder) -> Option<Box<dyn Mesh>>;
}
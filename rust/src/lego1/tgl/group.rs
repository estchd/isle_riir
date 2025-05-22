use windows::Win32::Graphics::Direct3D::D3DVECTOR;
use crate::lego1::realtime::matrix::Matrix4;
use crate::lego1::realtime::vector::Vector3;
use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::{MaterialMode, TglResult};
use crate::lego1::tgl::mesh_builder::MeshBuilder;
use crate::lego1::tgl::texture::Texture;

pub trait Group: Object {
	fn set_transformation(&mut self, transformation: &Matrix4) -> TglResult;

	fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) -> TglResult;

	fn set_texture(&mut self, texture: &dyn Texture) -> TglResult;

	fn get_texture(&self, texture: &mut dyn Texture) -> TglResult;

	fn set_material_mode(&mut self, material_mode: MaterialMode) -> TglResult;

	fn add_group(&mut self, group: &dyn Group) -> TglResult;

	fn add_mesh_builder(&mut self, mesh_builder: &dyn MeshBuilder) -> TglResult;

	fn remove_group(&mut self, group: &dyn Group) -> TglResult;

	fn remove_mesh_builder(&mut self, mesh_builder: &dyn MeshBuilder) -> TglResult;

	fn remove_all(&mut self) -> TglResult;

	fn bounds(&self, a: &mut D3DVECTOR, b: &mut D3DVECTOR) -> TglResult;
}
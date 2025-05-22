pub mod object;
pub mod renderer;
pub mod device;
mod view;
mod camera;
mod light;
mod mesh;
mod group;
mod mesh_builder;
mod texture;
mod d3drm;

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::HDC;

pub enum ColorModel {
	Ramp,
	RGB
}

pub enum ShadingModel {
	Wirefame,
	UnlitFlat,
	Flat,
	Gouraud,
	Phong
}

pub enum LightType {
	Ambient,
	Point,
	Spot,
	Directional,
	ParallelPoint
}

pub enum ProjectionType {
	Perspective,
	Orthographic,
}

pub enum TextureMappingMode {
	Linear,
	PerspectiveCorrect
}

pub enum MaterialMode {
	FromParent,
	FromFrame,
	FromMesh
}

pub struct PaletteEntry {
	red: u8,
	green: u8,
	blue: u8
}

pub struct DeviceDirect3DCreateData {
	direct_3d: (),
	direct_3d_device: ()
}

pub struct DeviceDirectDrawCreateData {
	driver_guid: (),
	h_wnd: HWND,
	direct_draw: (),
	front_buffer: (),
	back_buffer: (),
}

pub type TglResult = std::result::Result<(), ()>;

pub fn succeeded(result: TglResult) -> bool {
	result.is_ok()
}
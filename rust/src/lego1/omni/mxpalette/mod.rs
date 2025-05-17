use windows::Win32::Graphics::Gdi::{PALETTEENTRY, RGBQUAD};
use crate::lego1::omni::mxtypes::MxResult;

#[derive(Clone)]
pub struct MxPalette {

}

impl MxPalette {
	pub fn new() -> Self {
		Self {
			
		}	
	}
	
	pub fn from_data(data: &[RGBQUAD; 256]) -> Self {
		todo!()
	}

	pub fn get_entries(&self, entries: &mut [PALETTEENTRY; 256]) -> MxResult {
		todo!()
	}
}
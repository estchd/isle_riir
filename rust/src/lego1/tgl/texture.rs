use crate::lego1::tgl::object::Object;
use crate::lego1::tgl::{PaletteEntry, TglResult};

pub trait Texture: Object {
	fn set_texels(&mut self, 
	              width: i32, 
	              height: i32, 
	              bits_per_texel: i32, 
	              texels: &[u8]
	) -> TglResult;
	
	fn fill_rows_of_texture(&mut self, 
	                        y: i32, 
	                        height: i32,
	                        buffer: &[u8]
	);
	
	fn changed(&mut self, texels_changed: i32, palette_changed: i32) -> TglResult;
	
	fn get_buffer_and_palette(&mut self, width: &mut i32, height: &mut i32, depth: &mut i32, buffer: &mut &[u8], palette_size: &mut i32, palette: &mut &[PaletteEntry]) -> TglResult;
	
	fn set_palette(&mut self, entry_count: i32, entries: &[PaletteEntry]) -> TglResult;
}
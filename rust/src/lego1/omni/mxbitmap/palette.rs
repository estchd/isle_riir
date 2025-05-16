use static_assertions::assert_eq_size;
use zerocopy::TryFromBytes;

pub trait ColorPaletteEntry {
	fn set_r(&mut self, r: u8);
	fn set_g(&mut self, g: u8);
	fn set_b(&mut self, b: u8);

	fn get_r(&self) -> u8;
	fn get_g(&self) -> u8;
	fn get_b(&self) -> u8;
}

pub struct ColorPalette<T: ColorPaletteEntry> {
	pub data: Box<[T; 255]>
}

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct LegoRGBPaletteEntry {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub reserved: u8
}

assert_eq_size!([u8; 4], LegoRGBPaletteEntry);

pub type LegoColorPalette = ColorPalette<LegoRGBPaletteEntry>;


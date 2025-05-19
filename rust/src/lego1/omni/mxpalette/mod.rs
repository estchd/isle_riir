mod default_palette_entries;
mod safer_hdc;

use windows::Win32::Graphics::DirectDraw::IDirectDrawPalette;
use windows::Win32::Graphics::Gdi::{GetDeviceCaps, GetSystemPaletteEntries, PALETTEENTRY, PC_NOCOLLAPSE, RASTERCAPS, RC_PALETTE, RGBQUAD, SIZEPALETTE};
use crate::lego1::omni::mxcore::{MxCore, MxCoreStruct};
use crate::lego1::omni::mxpalette::default_palette_entries::DEFAULT_PALETTE_ENTRIES;
use crate::lego1::omni::mxpalette::safer_hdc::SafeHDC;
use crate::lego1::omni::mxparam::MxParam;
use crate::lego1::omni::mxtypes::{MxResult, FAILURE, SUCCESS};

const D3DPAL_RESERVED: u32 = 0x80;
const D3DPAL_READONLY: u32 = 0x40;

#[derive(Clone)]
pub struct MxPalette {
	core: MxCoreStruct,
	palette: Option<Box<IDirectDrawPalette>>,
	entries: [PALETTEENTRY; 256],
	override_sky_color: bool,
	sky_color: PALETTEENTRY
}

impl MxPalette {
	pub fn new() -> Self {
		let mut palette = Self {
			core: MxCoreStruct::new(),
			palette: None,
			entries: [PALETTEENTRY {peRed: 0, peBlue: 0, peGreen: 0, peFlags: 0}; 256],
			override_sky_color: false,
			sky_color: DEFAULT_PALETTE_ENTRIES[141]
		};

		get_default_palette(&mut palette.entries);

		palette
	}

	pub fn from_data(data: &[RGBQUAD; 256]) -> Self {
		let mut palette = Self {
			core: MxCoreStruct::new(),
			palette: None,
			entries: [PALETTEENTRY {peRed: 0, peBlue: 0, peGreen: 0, peFlags: 0}; 256],
			override_sky_color: false,
			sky_color: DEFAULT_PALETTE_ENTRIES[141]
		};

		apply_system_entries_to_palette(&mut palette.entries);

		for i in 10..246 {
			palette.entries[i].peRed = data[i].rgbRed;
			palette.entries[i].peGreen = data[i].rgbGreen;
			palette.entries[i].peBlue = data[i].rgbBlue;
			palette.entries[i].peFlags = 0
		}

		palette.sky_color = palette.entries[141];

		palette
	}

	pub fn detach(&mut self) {
		todo!()
	}

	pub fn get_entries(&self, entries: &mut [PALETTEENTRY; 256]) {
		entries.copy_from_slice(&self.entries);
	}

	pub fn set_entries(&mut self, entries: &[PALETTEENTRY; 256]) -> MxResult {
		if self.palette.is_none() {
			return SUCCESS;
		}

		for i in 0..10 {
			self.entries[i].peFlags = D3DPAL_RESERVED as u8;
		}

		for i in 10..136 {
			self.entries[i].peFlags = D3DPAL_READONLY as u8 | PC_NOCOLLAPSE as u8;
			self.entries[i].peRed = entries[i].peRed;
			self.entries[i].peGreen = entries[i].peGreen;
			self.entries[i].peBlue = entries[i].peBlue;
		}

		for i in 136..140 {
			self.entries[i].peFlags = D3DPAL_RESERVED as u8 | PC_NOCOLLAPSE as u8;
			self.entries[i].peRed = entries[i].peRed;
			self.entries[i].peGreen = entries[i].peGreen;
			self.entries[i].peBlue = entries[i].peBlue;
		}

		if !self.override_sky_color {
			self.entries[140].peFlags = D3DPAL_READONLY as u8 | PC_NOCOLLAPSE as u8;
			self.entries[141].peFlags = D3DPAL_RESERVED as u8 | PC_NOCOLLAPSE as u8;

			for i in 140..142 {
				self.entries[i].peRed = entries[i].peRed;
				self.entries[i].peGreen = entries[i].peGreen;
				self.entries[i].peBlue = entries[i].peBlue;
			}
		}

		for i in 142..246 {
			self.entries[i].peFlags = D3DPAL_RESERVED as u8 | PC_NOCOLLAPSE as u8;
			self.entries[i].peRed = entries[i].peRed;
			self.entries[i].peGreen = entries[i].peGreen;
			self.entries[i].peBlue = entries[i].peBlue;
		}

		for i in 0..10 {
			self.entries[i].peFlags = D3DPAL_RESERVED as u8;
		}

		let result = unsafe {
			self.palette.as_ref().unwrap().SetEntries(0, 0, 256, &mut self.entries as *mut PALETTEENTRY)
		};

		if result.is_ok() {
			SUCCESS
		}
		else {
			FAILURE
		}
	}

	pub fn set_sky_color(&mut self, sky_color: &PALETTEENTRY) -> MxResult {
		if self.palette.is_none() {
			return SUCCESS;
		}

		self.entries[141].peRed = sky_color.peRed;
		self.entries[141].peGreen = sky_color.peGreen;
		self.entries[141].peBlue = sky_color.peBlue;

		self.sky_color = self.entries[141];

		let result = unsafe {
			self.palette.as_ref().unwrap().SetEntries(0, 141, 1, &mut self.sky_color as *mut PALETTEENTRY)
		};

		if result.is_ok() {
			SUCCESS
		}
		else {
			FAILURE
		}
	}

	pub fn reset(&mut self, ignore_sky_color: bool) {
		todo!()
	}

	pub fn create_native_palette(&self) -> Box<IDirectDrawPalette> {
		todo!()
	}

	pub fn set_palette(&mut self, palette: Box<IDirectDrawPalette>) {
		self.palette = Some(palette);
	}

	pub fn set_override_sky_color(&mut self, value: bool) {
		todo!()
	}
}

impl PartialEq<Self> for MxPalette {
	fn eq(&self, other: &Self) -> bool {
		for i in 0..256 {
			if self.entries[i].peRed != other.entries[i].peRed {
				return false;
			}
			if self.entries[i].peGreen != other.entries[i].peGreen {
				return false;
			}
			if self.entries[i].peBlue != other.entries[i].peBlue {
				return false;
			}
		}

		true
	}
}

impl Eq for MxPalette {}

impl MxCore for MxPalette {
	fn notify(&mut self, param: &mut dyn MxParam) -> MxResult {
		self.core.notify(param)
	}

	fn tickle(&mut self) -> MxResult {
		self.core.tickle()
	}

	fn class_name(&self) -> String {
		self.core.class_name()
	}

	fn is_a(&self, name: &str) -> bool {
		self.core.is_a(name)
	}

	fn get_id(&self) -> u32 {
		self.core.get_id()
	}
}

fn get_default_palette(entries: &mut [PALETTEENTRY; 256]) {
	let hdc = SafeHDC::new(None);

	let raster_caps = unsafe {
		GetDeviceCaps(Some(hdc.get()), RASTERCAPS)
	};

	let size_palette = unsafe {
		GetDeviceCaps(Some(hdc.get()), SIZEPALETTE)
	};

	if (raster_caps & RC_PALETTE as i32) != 0 && size_palette == 256 {
		unsafe {
			GetSystemPaletteEntries(hdc.get(), 0, Some(entries));
		}

		let dest = &mut entries[10..246];
		let source = &DEFAULT_PALETTE_ENTRIES[10..246];

		dest.copy_from_slice(source);
	}
	else {
		entries.copy_from_slice(&DEFAULT_PALETTE_ENTRIES);
	}
}

fn apply_system_entries_to_palette(entries: &mut [PALETTEENTRY; 256]) {
	todo!()
}
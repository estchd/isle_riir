use std::sync::{Mutex, Weak};
use crate::lego1::omni::mxgeometry::mxrect::MxRect32;
use crate::lego1::omni::mxpalette::MxPalette;
use crate::lego1::omni::mxvideoparamflags::MxVideoParamFlags;

pub type PaletteRef = Option<Weak<Mutex<MxPalette>>>;

pub struct MxVideoParam {
	rect: MxRect32,
	palette: PaletteRef,
	back_buffers: u32,
	flags: MxVideoParamFlags,
	unknown_0x1c: i32,
	device_id: Option<String>
}

impl MxVideoParam {
	pub fn new() -> Self {
		Self {
			rect: MxRect32::new(0, 0, 640, 480),
			palette: None,
			back_buffers: 0,
			flags: MxVideoParamFlags::new(),
			unknown_0x1c: 0,
			device_id: None,
		}
	}

	pub fn create(rect: &MxRect32, palette: PaletteRef, back_buffers: u32, flags: &MxVideoParamFlags) -> Self {
		Self {
			rect: *rect,
			palette,
			back_buffers,
			flags: *flags,
			unknown_0x1c: 0,
			device_id: None,
		}
	}

	pub fn copy(video_param: &MxVideoParam) -> Self {
		let mut new = Self {
			rect: video_param.rect,
			palette: video_param.palette.clone(),
			back_buffers: video_param.back_buffers,
			flags: video_param.flags,
			unknown_0x1c: video_param.unknown_0x1c,
			device_id: None,
		};

		new.set_device_name(video_param.device_id.as_deref());

		new
	}

	pub fn set_device_name(&mut self, device_id: Option<&str>) {
		self.device_id = device_id.map(|s| s.to_owned());
	}

	pub fn assign(&mut self, video_param: &MxVideoParam) -> &mut Self {
		self.rect = video_param.rect;
		self.palette = video_param.palette.clone();
		self.back_buffers = video_param.back_buffers;
		self.flags = video_param.flags;
		self.unknown_0x1c = video_param.unknown_0x1c;

		self.set_device_name(video_param.device_id.as_deref());

		self
	}

	pub fn flags(&self) -> &MxVideoParamFlags {
		&self.flags
	}

	pub fn flags_mut(&mut self) -> &mut MxVideoParamFlags {
		&mut self.flags
	}

	pub fn get_rect(&self) -> &MxRect32 {
		&self.rect
	}

	pub fn get_palette(&self) -> PaletteRef {
		self.palette.clone()
	}

	pub fn set_palette(&mut self, palette: PaletteRef) {
		self.palette = palette;
	}

	pub fn get_device_name(&self) -> Option<&str> {
		self.device_id.as_deref()
	}

	pub fn get_back_buffers(&self) -> u32 {
		self.back_buffers
	}

	pub fn set_back_buffers(&mut self, back_buffers: u32) {
		self.back_buffers = back_buffers;
	}
}
use std::fs::File;
use std::io::Read;
use std::os::windows::fs::OpenOptionsExt;
use std::path::Path;
use static_assertions::assert_eq_size;
use windows::Win32::Graphics::Gdi::{BITMAPFILEHEADER, BITMAPINFOHEADER, BI_RGB, PALETTEENTRY, RGBQUAD};
use crate::lego1::omni::mxcore::{MxCore, MxCoreStruct};
use crate::lego1::omni::mxpalette::MxPalette;
use crate::lego1::omni::mxparam::MxParam;
use crate::lego1::omni::mxtypes::{MxResult, FAILURE, SUCCESS};
use crate::lego1::omni::mxutilities::get_rect_intersection;

pub mod palette;

pub const BITMAP_SIGNATURE: u16 = 66 << 0 | 77 << 8;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct MxBitmapInfo {
	bmi_header: BITMAPINFOHEADER,
	bmi_colors: [RGBQUAD; 256]
}

assert_eq_size!([u8; 0x428], MxBitmapInfo);

impl MxBitmapInfo {
	pub fn new() -> Self {
		Self {
			bmi_header: Default::default(),
			bmi_colors: [RGBQUAD::default(); 256],
		}
	}
}


pub struct MxBitmap {
	core: MxCoreStruct,
	info: Box<MxBitmapInfo>,
	data: Vec<u8>,
	is_high_color: bool,
	palette: Option<Box<MxPalette>>
}

impl MxBitmap {
	pub fn import_bitmap(bitmap: &MxBitmap) -> Self {
		let info = bitmap.info.clone();
		let data = bitmap.data.clone();

		Self {
			core: MxCoreStruct::new(),
			info,
			data,
			is_high_color: false,
			palette: None,
		}
	}

	pub fn import_bitmap_info(info: &MxBitmapInfo) -> Self {
		let size = align_to_four_byte(info.bmi_header.biWidth) * info.bmi_header.biHeight;

		let info = Box::new(info.clone());

		let data = vec![0; size as usize];

		Self {
			core: MxCoreStruct::new(),
			info,
			data,
			is_high_color: false,
			palette: None,
		}
	}

	pub fn set_size(width: i32, height: i32, palette: &MxPalette, is_high_color: bool) -> Result<Self, MxResult> {
		let size = align_to_four_byte(width) * height;

		let mut info = Box::new(MxBitmapInfo::new());

		let data = vec![0u8; size as usize];

		info.bmi_header.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
		info.bmi_header.biWidth = width;
		info.bmi_header.biHeight = height;
		info.bmi_header.biPlanes = 1;
		info.bmi_header.biBitCount = 8;
		info.bmi_header.biCompression = 0;
		info.bmi_header.biSizeImage = size as u32;

		let mut new = Self {
			core: MxCoreStruct::new(),
			info,
			data,
			is_high_color: false,
			palette: None,
		};

		let res = Self::import_colors_to_palette(&mut new.info.bmi_colors, new.palette.as_ref().map(Box::as_ref));

		if res != SUCCESS {
			return Err(res);
		}

		let res = new.set_bit_depth(is_high_color);

		if res != SUCCESS {
			return Err(res);
		}

		Ok(new)
	}

	pub fn load_file(file: &mut File) -> Result<Self, MxResult> {
		let mut hdr: BITMAPFILEHEADER = BITMAPFILEHEADER::default();

		let hdr_ptr = unsafe {
			std::mem::transmute::<&mut BITMAPFILEHEADER, &mut [u8; std::mem::size_of::<BITMAPFILEHEADER>()]>(&mut hdr)
		};

		let res = file.read(hdr_ptr);

		if res.is_err() {
			return Err(FAILURE);
		}

		if hdr.bfType != BITMAP_SIGNATURE {
			return Err(FAILURE);
		}

		let mut info: Box<MxBitmapInfo> = Box::new(MxBitmapInfo::new());

		let info_ptr = unsafe {
			std::mem::transmute::<&mut MxBitmapInfo, &mut [u8; std::mem::size_of::<BITMAPINFOHEADER>()]>(info.as_mut())
		};

		let res = file.read(info_ptr);

		if res.is_err() {
			return Err(FAILURE);
		}

		if info.bmi_header.biBitCount != 8 {
			return Err(FAILURE);
		}

		let size = hdr.bfSize as i32 - std::mem::size_of::<BITMAPFILEHEADER>() as i32 - std::mem::size_of::<BITMAPINFOHEADER>() as i32;

		let mut data = vec![0u8; size as usize];

		let res = file.read(data.as_mut_slice());

		if res.is_err() {
			return Err(FAILURE);
		}

		let mut new = Self {
			core: MxCoreStruct::new(),
			info,
			data,
			is_high_color: false,
			palette: None,
		};

		let new_ref = &new;

		if new_ref.info.bmi_header.biSizeImage == 0 {
			(&mut new).info.bmi_header.biSizeImage = new.get_data_size() as u32;
		}

		Ok(new)
	}

	pub fn read<T: AsRef<Path>>(path: &T) -> Result<Self, MxResult> {
		let file = File::options()
			.read(true)
			.write(false)
			.share_mode(1)
			.create_new(false)
			.create(false)
			.open(path);

		if file.is_err() {
			return Err(FAILURE);
		}

		let mut file = file.unwrap();

		Self::load_file(&mut file)
	}

	pub fn v_table_0x28(&self, value: i32) -> i32 {
		-1
	}

	pub fn bit_blit(
		&mut self,
		src: &Self,
		src_left: i32,
		src_top: i32,
		dst_left: i32,
		dst_top: i32,
		width: i32,
		height: i32
	) {
		if !get_rect_intersection(
			src.get_bmi_width(),
			src.get_bmi_height_abs(),
			self.get_bmi_width(),
			self.get_bmi_height_abs(),
			src_left,
			src_top,
			dst_left,
			dst_top,
			width,
			height
		) {
			return;
		}

		let mut src_start = src.get_start(src_left, src_top);
		let mut dst_start = self.get_start(dst_top, dst_top);

		let src_stride = src.get_adjusted_stride() as i32;
		let dst_stride = self.get_adjusted_stride() as i32;

		let src_data = src.data.as_slice();
		let dst_data = self.data.as_mut_slice();

		for _ in 0..height {
			let src = &src_data[src_start as usize .. src_start as usize + width as usize];
			let dst = &mut dst_data[dst_start as usize .. dst_start as usize + width as usize];

			dst.copy_from_slice(src);

			src_start += src_stride;
			dst_start += dst_stride;
		}
	}

	pub fn bit_blit_transparent(
		&mut self,
		src: &Self,
		src_left: i32,
		src_top: i32,
		dst_left: i32,
		dst_top: i32,
		width: i32,
		height: i32
	) {
		if !get_rect_intersection(
			src.get_bmi_width(),
			src.get_bmi_height_abs(),
			self.get_bmi_width(),
			self.get_bmi_height_abs(),
			src_left,
			src_top,
			dst_left,
			dst_top,
			width,
			height
		) {
			return;
		}

		let mut src_start = src.get_start(src_left, src_top);
		let mut dst_start = self.get_start(dst_top, dst_top);

		let src_stride = -width + src.get_adjusted_stride() as i32;
		let dst_stride = - width + self.get_adjusted_stride() as i32;

		let src_data = src.data.as_slice();
		let dst_data = self.data.as_mut_slice();

		for h in 0..height {
			for w in 0..width {
				if src_data[src_start as usize + w as usize] != 0 {
					dst_data[dst_start as usize + w as usize] = src_data[src_start as usize + w as usize];
				}
			}

			src_start += src_stride;
			dst_start += dst_stride;
		}
	}

	pub fn create_palette(&self) -> Box<MxPalette> {
		if self.is_high_color {
			self.palette.as_ref().unwrap().clone()
		}
		else {
			Box::new(MxPalette::from_data(&self.info.bmi_colors))
		}
	}

	pub fn import_palette(&mut self, palette: &MxPalette) {
		if self.is_high_color {
			self.palette = Some(Box::new(palette.clone()))
		}
		else {
			Self::import_colors_to_palette(&mut self.info.bmi_colors, self.palette.as_ref().map(Box::as_ref));
		}
	}

	pub fn set_bit_depth(&mut self, is_high_color: bool) -> MxResult {
		if self.is_high_color && is_high_color {
			return SUCCESS;
		}

		if is_high_color {
			let palette = MxPalette::from_data(&self.info.bmi_colors);

			self.palette = Some(Box::new(palette));

			let palette_ptr = unsafe {
				std::mem::transmute::<&mut [RGBQUAD; 256], &mut [u16; (256 * std::mem::size_of::<RGBQUAD>()) / 2]>(&mut self.info.bmi_colors)
			};

			for i in 0..256u16 {
				palette_ptr[i as usize] = i;
			}
		}
		else {
			Self::import_colors_to_palette(&mut self.info.bmi_colors, self.palette.as_ref().map(Box::as_ref));
			self.palette = None;
		}

		self.is_high_color = is_high_color;

		SUCCESS
	}

	pub fn stretch_bits(
		&mut self,
		hdc: (),
		x_src: i32,
		y_src: i32,
		x_dest: i32,
		y_dest: i32,
		dest_width: i32,
		dest_height: i32,
	) -> MxResult {
		todo!()
	}

	pub fn get_bmi_header(&self) -> &BITMAPINFOHEADER {
		&self.info.bmi_header
	}

	pub fn get_bmi_header_mut(&mut self) -> &mut BITMAPINFOHEADER {
		&mut self.info.bmi_header
	}

	pub fn get_bmi_width(&self) -> i32 {
		self.get_bmi_header().biWidth
	}

	pub fn get_bmi_stride(&self) -> i32 {
		align_to_four_byte(self.get_bmi_width())
	}

	pub fn get_bmi_height(&self) -> i32 {
		self.get_bmi_header().biHeight
	}

	pub fn get_bmi_height_abs(&self) -> i32 {
		height_abs(self.get_bmi_height())
	}

	pub fn get_image(&self) -> &[u8] {
		self.data.as_slice()
	}

	pub fn get_image_mut(&mut self) -> &mut [u8] {
		self.data.as_mut_slice()
	}

	pub fn get_bitmap_info(&self) -> &MxBitmapInfo {
		&self.info
	}

	pub fn get_bitmap_info_mut(&mut self) -> &mut MxBitmapInfo {
		&mut self.info
	}

	pub fn get_data_size(&self) -> i32 {
		self.get_bmi_stride() * self.get_bmi_height_abs()
	}

	pub fn is_top_down(&self) -> bool {
		if self.info.bmi_header.biCompression == 0x10 {
			return true;
		}

		self.info.bmi_header.biHeight < 0
	}

	pub fn get_start(&self, left: i32, top: i32) -> i32 {
		if self.get_bmi_header().biCompression == BI_RGB.0 {
			let start_top = if self.is_top_down() {
				top
			} else {
				(self.get_bmi_height_abs() - 1) - top
			};

			left + self.get_bmi_stride() * start_top
		}
		else if self.get_bmi_header().biCompression == 0x10 {
			0
		}
		else {
			if self.is_top_down() {
				0
			} else {
				self.get_bmi_height_abs() - 1
			}
		}
	}

	fn is_bottom_up(&self) -> bool {
		if self.get_bmi_header().biCompression == 0x10 {
			return false;
		}

		self.get_bmi_header().biHeight > 0
	}

	fn import_colors_to_palette(colors: &mut [RGBQUAD; 256], palette: Option<&MxPalette>) -> MxResult {
		let mut entries: [PALETTEENTRY; 256] = [PALETTEENTRY::default(); 256];

		if let Some(palette) = palette {
			if palette.get_entries(&mut entries) != SUCCESS {
				return FAILURE;
			}
		}
		else {
			let palette = MxPalette::new();
			if palette.get_entries(&mut entries) != SUCCESS {
				return FAILURE;
			}
		}

		for i in 0..256 {
			colors[i].rgbRed = entries[i].peRed;
			colors[i].rgbGreen = entries[i].peGreen;
			colors[i].rgbBlue = entries[i].peBlue;
			colors[i].rgbReserved = 0;
		}

		SUCCESS
	}

	fn get_adjusted_stride(&self) -> u32 {
		if self.is_top_down() {
			self.get_bmi_stride() as u32
		}
		else {
			-self.get_bmi_stride() as u32
		}
	}
}

impl MxCore for MxBitmap {
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

pub struct IndexedBitmapData {
	pub data: Vec<u8>
}

pub fn align_to_four_byte(value: i32) -> i32 {
	(value + 3) & -4
}

pub fn height_abs(value: i32) -> i32 {
	value.abs()
}
mod ffi_structs;

use windows::Win32::Graphics::Gdi::BITMAPINFOHEADER;

#[repr(u16)]
pub enum FlicChunkType {
	FliChunkColor256 = 4,
	FliChunkSS2 = 7,
	FliChunkColor64 = 11,
	FliChunkLC = 12,
	FliChunkBlack = 13,
	FliChunkBrun = 15,
	FliChunkCopy = 16,
	FliChunkPStamp = 18,
	FliChunkFrame = 0xF1FA
}

#[repr(C, packed(1))]
pub struct FlicChunk {
	pub size: i32,
	pub chunk_type: FlicChunkType,
}

#[repr(C, packed(1))]
pub struct FlicHeader {
	pub chunk: FlicChunk,
	pub frames: u16,
	pub width: u16,
	pub height: u16,
	pub depth: u16,
	pub flags: u16,
	pub speed: u32
}

#[repr(C, packed(1))]
pub struct FlicFrame {
	pub chunk: FlicChunk,
	pub chunks: u16,
	pub delay: u16,
	pub reserved: u16,
	pub width: u16,
	pub height: u16,
}

pub fn decode_flc_frame(
	bitmap_header: &mut BITMAPINFOHEADER,
	flic_header: &mut FlicHeader,
	flic_frame: &mut FlicFrame,
	pixel_data: &mut [u8],
	decoded_color_map: &mut [u8]
) {
	todo!()
}

fn write_pixel(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	column: i16,
	row: i16,
	pixel: u8
) {
	if column < 0 || row < 0 || column as i32 > bitmap_header.biWidth || row as i32 > bitmap_header.biHeight {
		return;
	}

	let row_size = (bitmap_header.biWidth + 3) & -4;

	let index = row_size * row as i32 + column as i32;

	pixel_data[index as usize] = pixel;
}

fn write_pixels(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	mut column: i16,
	mut row: i16,
	data: &mut [u8],
	mut count: i16
) {
	let zcol = column;

	if clamp_line(bitmap_header, &mut column, &mut row, &mut count) == 0{
		return;
	}

	let data_offset = column - zcol;

	let row_size = (bitmap_header.biWidth + 3) & -4;

	let pixel_data_offset = row_size * row as i32 + column as i32;

	let source = &data[data_offset as usize..data_offset as usize + count as usize];

	let destination = &mut pixel_data[pixel_data_offset as usize..pixel_data_offset as usize + source.len()];

	destination.copy_from_slice(source);
}

fn clamp_line(
	bitmap_header: &mut BITMAPINFOHEADER,
	p_column: &mut i16,
	p_row: &mut i16,
	p_count: &mut i16
) -> i32 {
	let column = *p_column;
	let a_row = *p_row;
	let mut f_count = *p_count;
	let end = column + f_count;

	if a_row < 0 || bitmap_header.biHeight <= a_row as i32 || end < 0 ||bitmap_header.biWidth <= column as i32 {
		return 0;
	}

	if column < 0 {
		f_count += column;
		*p_count = f_count;
		*p_column = 0;
	}

	if bitmap_header.biWidth < end as i32 {
		f_count -= end - bitmap_header.biWidth as i16;
		*p_count = f_count;
	}

	if f_count < 0 {
		return 0;
	}

	1
}

fn write_pixel_run(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	mut column: i16,
	mut row: i16,
	pixel: u8,
	mut count: i16
) {
	if clamp_line(bitmap_header, &mut column, &mut row, &mut count) == 0 {
		return;
	}

	let row_size = (bitmap_header.biWidth + 3) & -4;

	let index = row_size * row as i32 + column as i32;

	let destination = &mut pixel_data[index as usize..index as usize + count as usize];

	for index in 0..count as usize {
		destination[index] = pixel;
	}
}

fn write_pixel_pairs(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	mut column: i16,
	mut row: i16,
	pixel: u16,
	mut count: i16
) {
	count <<= 1;

	if clamp_line(bitmap_header, &mut column, &mut row, &mut count) == 0 {
		return;
	}

	let is_odd = count % 2 == 0;

	count >>= 1;

	let row_size = (bitmap_header.biWidth + 3) & -4;

	let destination_index = row_size * row as i32 + column as i32;

	let destination = &mut pixel_data[destination_index as usize..destination_index as usize + count as usize];

	return todo!();

	for index in 0..count as usize {
	}

	if is_odd {

	}

	todo!()
}

fn decode_chunks(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	flic_header: &mut FlicHeader,
	flic_frame: &mut FlicFrame,
	flic_subchunks: &mut [FlicChunk],
	decoded_color_map: &mut bool
) -> u16 {
	*decoded_color_map = false;

	for subchunk in 0..flic_frame.chunks as i16 {
		let flic_chunk = &mut flic_subchunks[subchunk as usize];

		match flic_chunk.chunk_type {
			FlicChunkType::FliChunkColor256 => {
				decode_colors_256(bitmap_header, flic_chunk);
				*decoded_color_map = true;
			}
			FlicChunkType::FliChunkSS2 => {
				decode_ss2(bitmap_header, pixel_data, flic_chunk, flic_header);
			}
			FlicChunkType::FliChunkColor64 => {
				decode_colors_64(bitmap_header, flic_chunk);
				*decoded_color_map = true;
			}
			FlicChunkType::FliChunkLC => {
				decode_lc(bitmap_header, pixel_data, flic_chunk, flic_header);
			}
			FlicChunkType::FliChunkBlack => {
				decode_black(bitmap_header, pixel_data, flic_chunk, flic_header);
			}
			FlicChunkType::FliChunkBrun => {
				decode_brun(bitmap_header, pixel_data, flic_chunk, flic_header);
			}
			FlicChunkType::FliChunkCopy => {
				decode_copy(bitmap_header, pixel_data, flic_chunk, flic_header);
			}
			_ => {}
		}
	}

	0
}

fn decode_colors_256(
	bitmap_header: &mut BITMAPINFOHEADER,
	data: &mut FlicChunk,
) {
	decode_color_packets(bitmap_header, data);
}

fn decode_color_packets(
	bitmap_header: &mut BITMAPINFOHEADER,
	data: &mut FlicChunk,
) {
	let color_index = 0;

	let colors = data;

}

fn decode_color_packet(
	bitmap_header: &mut BITMAPINFOHEADER,
	data: &mut FlicChunk,
	index: u16,
	count: u16
) {
	todo!()
}

fn decode_colors_64(
	bitmap_header: &mut BITMAPINFOHEADER,
	data: &mut FlicChunk,
) {
	todo!();
}

fn decode_brun(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	data: &mut FlicChunk,
	flic_header: &mut FlicHeader,
) {
	todo!()
}

fn decode_lc(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	data: &mut FlicChunk,
	flic_header: &mut FlicHeader,
) {
	todo!()
}

fn decode_ss2(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	data: &mut FlicChunk,
	flic_header: &mut FlicHeader,
) {
	todo!()
}

fn decode_black(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	data: &mut FlicChunk,
	flic_header: &mut FlicHeader,
) {
	todo!()
}

fn decode_copy(
	bitmap_header: &mut BITMAPINFOHEADER,
	pixel_data: &mut [u8],
	data: &mut FlicChunk,
	flic_header: &mut FlicHeader,
) {
	todo!()
}
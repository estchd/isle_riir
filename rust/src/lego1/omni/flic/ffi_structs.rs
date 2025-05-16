use static_assertions::assert_eq_size;
use zerocopy::TryFromBytes;

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(u16)]
pub enum FlicFileType {
	Fli = 0xAF11,
	Flc = 0xAF12,
}

assert_eq_size!([u8; 2], FlicFileType);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct FlicFileHeaderMagic {
	pub size: u32,
	pub file_type: FlicFileType,
}

assert_eq_size!([u8; 6], FlicFileHeaderMagic);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct BaseFlicFileHeader {
	pub magic: FlicFileHeaderMagic,
	pub frames: u16,
	pub width: u16,
	pub height: u16,
	pub depth: u16,
	pub flags: u16
}

assert_eq_size!([u8; 16], BaseFlicFileHeader);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct FliFileHeader {
	pub base_header: BaseFlicFileHeader,
	pub speed: u16,
	pub reserved: [u8; 110]
}

assert_eq_size!([u8; 128], FliFileHeader);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct FlcFileHeader {
	pub base_header: BaseFlicFileHeader,
	pub speed: u32,
	pub reserved1: [u8; 2],
	pub created: u32,
	pub creator: [u8; 4],
	pub updated: u32,
	pub updater: [u8; 4],
	pub aspect_x: u16,
	pub aspect_y: u16,
	pub reserved2: [u8; 38],
	pub o_frame1: u32,
	pub o_frame_2: u32,
	pub reserved3: [u8; 40]
}

assert_eq_size!([u8; 128], FlcFileHeader);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(u16)]
pub enum FlcChunkType {
	Prefix = 0xF100,
	Frame = 0xF1FA
}

assert_eq_size!([u8; 2], FlcChunkType);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct FlcChunkHeader {
	pub size: u32,
	pub chunk_type: FlcChunkType,
	pub chunks: u16,
	pub reserved: [u8; 8]
}

assert_eq_size!([u8; 16], FlcChunkHeader);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(u16)]
pub enum FlcDataChunkType {
	Color256 = 4,
	SS2 = 7,
	Color = 11,
	LC = 12,
	Black = 13,
	Brun = 15,
	Copy = 16,
	PStamp = 18
}

assert_eq_size!([u8; 2], FlcDataChunkType);

#[derive(Copy, Clone, Debug)]
#[derive(TryFromBytes)]
#[repr(C, packed(1))]
pub struct FlcDataChunkHeader {
	pub size: u32,
	pub data_chunk_type: FlcDataChunkType,
}

assert_eq_size!([u8; 6], FlcDataChunkHeader);
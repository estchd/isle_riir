use bitfield::*;

pub type MxU8 = u8;
pub type MxS8 = i8;
pub type MxU16 = u16;
pub type MxS16 = i16;
pub type MxU32 = u32;
pub type MxS32 = i32;
pub type MxU64 = u64;
pub type MxS64 = i64;
pub type MxFloat = f32;
pub type MxDouble = f64;
pub type MxLong = i32;
pub type MxULong = u32;

pub type MxTime = MxS32;
pub type MxResult = MxLong;

pub const SUCCESS: MxResult = 0;
pub const FAILURE: MxResult = -1;

pub type MxBool = bool;

bitfield! {
	#[derive(Copy, Clone)]
	pub struct FlagBitfield(u8);
	impl Debug;
	impl BitAnd;
	impl BitOr;
	impl BitXor;
	impl new;
	#[inline]
	pub bit0, get_bit_0, set_bit_0: 0;
	pub bit1, get_bit_1, set_bit_1: 0;
	pub bit2, get_bit_2, set_bit_2: 0;
	pub bit3, get_bit_3, set_bit_3: 0;
	pub bit4, get_bit_4, set_bit_4: 0;
	pub bit5, get_bit_5, set_bit_5: 0;
	pub bit6, get_bit_6, set_bit_6: 0;
	pub bit7, get_bit_7, set_bit_7: 0;
}
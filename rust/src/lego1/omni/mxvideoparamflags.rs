use crate::lego1::omni::mxtypes::FlagBitfield;

pub struct MxVideoParamFlags {
	flags1: FlagBitfield,
	flags2: FlagBitfield,
}

impl MxVideoParamFlags {
	pub fn new() -> MxVideoParamFlags {
		let mut value = Self {
			flags1: FlagBitfield(0),
			flags2: FlagBitfield(0),
		};

		value.flags1.set_bit_6(true);
		value.flags1.set_bit_7(true);

		value.flags2.set_bit_1(true);

		value
	}

	pub fn set_fullscreen(&mut self, value: bool) {
		self.flags1.set_bit_0(value);
	}

	pub fn set_flip_surfaces(&mut self, value: bool) {
		self.flags1.set_bit_1(value);
	}

	pub fn set_back_buffers(&mut self, value: bool) {
		self.flags1.set_bit_2(value);
	}

	pub fn set_f1_bit3(&mut self, value: bool) {
		self.flags1.set_bit_3(value);
	}

	pub fn set_16_bit(&mut self, value: bool) {
		self.flags1.set_bit_5(value);
	}

	pub fn set_wide_view_angle(&mut self, value: bool) {
		self.flags1.set_bit_6(value);
	}

	pub fn set_f1_bit_7(&mut self, value: bool) {
		self.flags1.set_bit_7(value);
	}

	pub fn set_f2_bit_0(&mut self, value: bool) {
		self.flags2.set_bit_0(value);
	}

	pub fn set_f2_bit_1(&mut self, value: bool) {
		self.flags2.set_bit_1(value);
	}

	pub fn get_fullscreen(&self) -> bool {
		self.flags1.get_bit_0()
	}

	pub fn get_flip_surfaces(&self) -> bool {
		self.flags1.get_bit_1()
	}

	pub fn get_back_buffers(&self) -> bool {
		self.flags1.get_bit_2()
	}

	pub fn get_f1_bit_3(&self) -> bool {
		self.flags1.get_bit_3()
	}

	pub fn get_16_bit(&self) -> bool {
		self.flags1.get_bit_5()
	}

	pub fn get_wide_view_angle(&self) -> bool {
		self.flags1.get_bit_6()
	}

	pub fn get_f2_bit_0(&self) -> bool {
		self.flags2.get_bit_0()
	}

	pub fn get_f2_bit_1(&self) -> bool {
		self.flags2.get_bit_1()
	}
}
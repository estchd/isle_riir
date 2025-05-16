use crate::lego1::omni::mxcore::{MxCore, MxCoreStruct};
use crate::lego1::omni::mxparam::MxParam;
use crate::lego1::omni::mxtypes::MxResult;

pub mod palette;

pub struct MxBitmap {
	core: MxCoreStruct,
}

impl MxBitmap {
	pub fn new() -> Self {
		Self {
			core: MxCoreStruct::new(),
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
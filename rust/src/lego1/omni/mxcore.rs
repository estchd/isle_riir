use std::sync::atomic::{AtomicU32, Ordering};
use crate::lego1::omni::mxparam::MxParam;
use crate::lego1::omni::mxtypes::{MxResult, SUCCESS};

const NEXT_CORE_ID: AtomicU32 = AtomicU32::new(0);

pub trait MxCore {
	fn notify(&mut self, param: &mut dyn MxParam) -> MxResult;
	fn tickle(&mut self) -> MxResult;
	fn class_name(&self) -> String;
	fn is_a(&self, name: &str) -> bool;
	fn get_id(&self) -> u32;
}

pub struct MxCoreStruct {
	id: u32
}

impl MxCoreStruct {
	pub fn new() -> Self {
		let id = NEXT_CORE_ID.fetch_add(1, Ordering::Relaxed);

		Self {
			id
		}
	}
}

impl MxCore for MxCoreStruct {
	fn notify(&mut self, _: &mut dyn MxParam) -> MxResult {
		0
	}

	fn tickle(&mut self) -> MxResult {
		SUCCESS
	}

	fn class_name(&self) -> String {
		"MxCore".to_string()
	}

	fn is_a(&self, name: &str) -> bool {
		name == self.class_name()
	}

	fn get_id(&self) -> u32 {
		self.id
	}
}
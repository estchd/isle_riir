use std::marker::PhantomData;
use crate::lego1::omni::mxcore::{MxCore, MxCoreStruct};
use crate::lego1::omni::mxparam::MxParam;
use crate::lego1::omni::mxtypes::MxResult;

fn destroy<T>(_: T) {}

pub trait MxCollection<T>: MxCore {
	fn compare(&self, a: T, b: T) -> i8;
	fn set_destroy(&mut self, custom_destructor: Box<dyn Fn(T)>);
}

pub struct MxCollectionStruct<T> {
	pub count: u32,

	pub custom_destructor: Box<dyn Fn(T)>,

	pub core: MxCoreStruct,
	_phantom: PhantomData<T>,
}

impl<T> MxCollectionStruct<T> {
	pub fn new() -> Self {
		Self {
			count: 0,
			custom_destructor: Box::new(|t| {destroy::<T>(t)}),
			core: MxCoreStruct::new(),
			_phantom: PhantomData,
		}
	}
}

impl<T> MxCore for MxCollectionStruct<T> {

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

impl<T> MxCollection<T> for MxCollectionStruct<T> {
	fn compare(&self, _: T, _: T) -> i8 {
		0
	}

	fn set_destroy(&mut self, custom_destructor: Box<dyn Fn(T)>) {
		self.custom_destructor = custom_destructor;
	}
}
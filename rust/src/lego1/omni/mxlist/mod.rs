use crate::lego1::omni::mxcollection::{MxCollection, MxCollectionStruct};
use crate::lego1::omni::mxcore::MxCore;
use crate::lego1::omni::mxlist::mxlistentry::MxListEntry;
use crate::lego1::omni::mxparam::MxParam;
use crate::lego1::omni::mxtypes::MxResult;

pub mod mxlistentry;
pub mod mxlistcursor;

pub trait MxList<T>: MxCollection<T> {
	fn append(&mut self, object: T);
	fn prepend(&mut self, object: T);
	fn delete_all(&mut self);
	fn empty(&mut self);
	fn get(&self, index: usize) -> &Option<T>;
	fn get_mut(&mut self, index: usize) -> &mut Option<T>;

	fn first(&self) -> Option<usize>;
	fn last(&self) -> Option<usize>;
	
	fn num_elements(&self) -> usize;

	fn delete_entry(&mut self, index: usize);
	fn insert_entry(&mut self, object: T, prev: Option<usize>, next: Option<usize>);
}

pub struct MxListStruct<T> {
	first: Option<usize>,
	last: Option<usize>,
	data: Vec<Option<MxListEntry<T>>>,
	collection: MxCollectionStruct<T>
}

impl<T> MxListStruct<T> {
	pub fn new() -> Self {
		Self {
			first: None,
			last: None,
			data: Vec::new(),
			collection: MxCollectionStruct::new(),
		}
	}
}

impl<T> MxCore for MxListStruct<T> {
	fn notify(&mut self, param: &mut dyn MxParam) -> MxResult {
		self.collection.notify(param)
	}

	fn tickle(&mut self) -> MxResult {
		self.collection.tickle()
	}

	fn class_name(&self) -> String {
		self.collection.class_name()
	}

	fn is_a(&self, name: &str) -> bool {
		self.collection.is_a(name)
	}

	fn get_id(&self) -> u32 {
		self.collection.get_id()
	}
}

impl<T> MxCollection<T> for MxListStruct<T> {
	fn compare(&self, a: T, b: T) -> i8 {
		self.collection.compare(a, b)
	}

	fn set_destroy(&mut self, custom_destructor: Box<dyn Fn(T)>) {
		self.collection.set_destroy(custom_destructor);
	}
}

impl<T> MxList<T> for MxListStruct<T> {
	fn append(&mut self, object: T) {
		self.insert_entry(object, self.last, None);
	}

	fn prepend(&mut self, object: T) {
		self.insert_entry(object, None, self.first);
	}

	fn delete_all(&mut self) {
		for entry in self.data.drain(..) {
			if entry.is_none() {
				continue;
			}

			let entry = entry.unwrap();

			(self.collection.custom_destructor)(entry.object);
			self.first = None;
			self.last = None;
		}
	}

	fn empty(&mut self) {
		todo!()
	}

	fn get(&self, index: usize) -> &Option<T> {
		todo!()
	}

	fn get_mut(&mut self, index: usize) -> &mut Option<T> {
		todo!()
	}

	fn first(&self) -> Option<usize> {
		todo!()
	}

	fn last(&self) -> Option<usize> {
		todo!()
	}

	fn num_elements(&self) -> usize {
		todo!()
	}

	fn delete_entry(&mut self, index: usize) {
		todo!()
	}

	fn insert_entry(&mut self, object: T, prev: Option<usize>, next: Option<usize>) {
		todo!()
	}
}
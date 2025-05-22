pub trait Object {
	fn implementation_data_ptr(&self) -> ();

	fn implementation_data_ptr_mut(&mut self) -> ();
}

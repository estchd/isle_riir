pub trait LODObject {
	fn average_poly_area(&self) -> f64;

	fn n_verts(&self) -> i32;

	fn num_polys(&self) -> i32;

	fn v_table_0x10(&self) -> f32;
}
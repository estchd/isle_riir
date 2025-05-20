use nalgebra::{U1, U2, U3, U4};

pub type Matrix4 = nalgebra::Matrix4<f32>;

pub type Matrix4Vec2Ref<'a> = nalgebra::MatrixView<'a, f32, U2, U1, U1, U4>;

pub type Matrix4Vec3Ref<'a> = nalgebra::MatrixView<'a, f32, U3, U1, U1, U4>;

pub type Matrix4Vec4Ref<'a> = nalgebra::MatrixView<'a, f32, U4, U1, U1, U4>;

pub type Matrix4Vec2RefMut<'a> = nalgebra::MatrixViewMut<'a, f32, U2, U1, U1, U4>;

pub type Matrix4Vec3RefMut<'a> = nalgebra::MatrixViewMut<'a, f32, U3, U1, U1, U4>;

pub type Matrix4Vec4RefMut<'a> = nalgebra::MatrixViewMut<'a, f32, U4, U1, U1, U4>;
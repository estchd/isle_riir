use std::sync::atomic::Ordering;
use atomic_float::AtomicF32;

const USER_MAX_LOD_POWER: AtomicF32 = AtomicF32::new(0.0);
const USER_MAX_BASE: AtomicF32 = AtomicF32::new(4.0);
const USER_MAX_LOD: AtomicF32 = AtomicF32::new(3.6);
const PARTS_THRESHOLD: AtomicF32 = AtomicF32::new(1000.0);

pub struct RealtimeView {

}

impl RealtimeView {
	pub fn new() -> Self {
		Self::update_max_lod();

		Self {

		}
	}

	pub fn get_parts_threshold() -> f32 {
		PARTS_THRESHOLD.load(Ordering::Relaxed)
	}

	pub fn get_user_max_lod() -> f32 {
		USER_MAX_LOD.load(Ordering::Relaxed)
	}

	pub fn set_parts_threshold(value: f32) {
		PARTS_THRESHOLD.store(value, Ordering::Relaxed);
	}

	pub fn update_max_lod() {
		let user_max_base = USER_MAX_BASE.load(Ordering::Relaxed);
		let user_max_lod = USER_MAX_LOD.load(Ordering::Relaxed);

		let user_max_lod_power = user_max_base.powf(-user_max_lod);

		USER_MAX_LOD_POWER.store(user_max_lod, Ordering::Relaxed);
	}

	pub fn set_user_max_lod(value: f32) {
		USER_MAX_LOD.store(value, Ordering::Relaxed);
		Self::update_max_lod();
	}

	pub fn get_user_max_lod_power() -> f32 {
		USER_MAX_LOD_POWER.load(Ordering::Relaxed)
	}
}
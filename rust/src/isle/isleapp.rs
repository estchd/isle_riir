use std::sync::atomic::{AtomicBool, AtomicU8};
use windows::core::w;
use windows::Win32::Foundation::{HINSTANCE, HWND, RECT, WPARAM};
use windows::Win32::System::WindowsProgramming::GetProfileStringW;
use windows::Win32::UI::WindowsAndMessaging::HCURSOR;
use crate::lego1::omni::mxgeometry::mxrect::MxRect32;
use crate::lego1::omni::mxtypes::MxResult;
use crate::lego1::omni::mxvideoparam::MxVideoParam;
use crate::lego1::omni::mxvideoparamflags::MxVideoParamFlags;

const MOUSE_DOWN: AtomicU8 = AtomicU8::new(0);
const MOUSE_MOVED: AtomicU8 = AtomicU8::new(0);
const CLOSED: AtomicBool = AtomicBool::new(false);
const WINDOW_RECT: RECT = RECT {
	left: 0,
	top: 0,
	right: 640,
	bottom: 480,
};

const RM_DISABLED: AtomicBool = AtomicBool::new(false);

const WAITING_FOR_TARGET_DEPTH: AtomicBool = AtomicBool::new(false);

const TARGET_WIDTH: i32 = 640;

const TARGET_HEIGHT: i32 = 480;

const TARGET_DEPTH: i32 = 16;

const REQ_ENABLE_RM_DEVICE: AtomicBool = AtomicBool::new(false);

const WNDCLASS_NAME: &'static str = "Lego Island MainNoM App";

const WINDOW_TITLE: &'static str = "LEGO\u{AE}";

pub struct IsleApp {
	hd_path: Option<String>,
	cd_path: Option<String>,
	device_id: Option<String>,
	save_path: Option<String>,
	full_screen: bool,
	flip_surfaces: bool,
	back_buffers_in_vram: bool,
	using_8_bit: bool,
	using_16_bit: bool,
	unknown_0x24: i32,
	use_3d_sound: bool,
	use_music: bool,
	use_joystick: bool,
	joystick_index: i32,
	wide_view_angle: bool,
	island_quality: i32,
	island_texture: i32,
	game_started: bool,
	frame_delta: i32,
	video_param: MxVideoParam,
	window_active: bool,
	window_handle: Option<HWND>,
	draw_cursor: bool,
	cursor_arrow: Option<HCURSOR>,
	cursor_busy: Option<HCURSOR>,
	cursor_no: Option<HCURSOR>,
	cursor_current: Option<HCURSOR>
}

impl IsleApp {
	pub fn new() -> Self {
		let r = MxRect32::new(0, 0, 639, 479);
		let mut flags = MxVideoParamFlags::new();

		flags.set_16_bit(todo!()/*MxDirectDraw::GetPrimaryBitDepth() == 16*/);

		let video_param = MxVideoParam::create(&r, None, 1, &flags);

		Self {
			hd_path: None,
			cd_path: None,
			device_id: None,
			save_path: None,
			full_screen: true,
			flip_surfaces: false,
			back_buffers_in_vram: true,
			using_8_bit: false,
			using_16_bit: true,
			unknown_0x24: 0,
			use_3d_sound: false,
			use_music: true,
			use_joystick: false,
			joystick_index: 0,
			wide_view_angle: true,
			island_quality: 1,
			island_texture: 1,
			game_started: false,
			frame_delta: 10,
			video_param,
			window_active: true,
			window_handle: None,
			draw_cursor: false,
			cursor_arrow: None,
			cursor_busy: None,
			cursor_no: None,
			cursor_current: None,
		};
		todo!() /* LegoOmni::CreateInstance(); */
	}

	pub fn close(&mut self) {
		// let mut ds = MxDsAction::new();

		// ds.set_unknown_24(-2);

		if todo!() /* !lego() */ {
			return;
		}

		// game_state().save(0);

		if todo!() /* input_manager() */ {
			// input_manager().queue_event(NotificationId::NotificationKeyPress, 00, 0, 0, VK_SPACE);
		}

		// video_manager().get_3d_manager().get_lego_3d_view().get_view_manager().remove_all(None);

		// lego().remove_world(ds.get_atom_id(), ds.get_object_id());

		// lego().delete_object(ds);

		// transition_manager().set_wait_indicator(None);

		// lego().resume();

		loop {
			/*
			if streamer().close(None) != SUCCESS {
				break;
			}
			 */
		}

		while todo!() /* lego() && !lego().does_entity_exist(ds) */ {
			// timer().get_real_time();
			// tickle_manager().tickle();
		}

		todo!()
	}

	pub fn setup_lego_omni(&mut self) -> bool {
		let mut media_path: [u16; 128] = [0; 128];

		unsafe {
			GetProfileStringW(w!("LEGO Island"), w!("MediaPath"), w!(""), Some(&mut media_path));
		}

		// let flags = MxOnmiCreateFlags::new()
		// let param = MxOmniCreateParam::create(media_path, self.window_handle, self.video_param, flags);

		let failure: bool = todo!(); // lego().create(param) == FAILURE;

		if failure {
			false
		}
		else {
			// variable_table().set_variable("ACTOR_01", "");
			// tickle_manager().set_client_tickle_interval(video_manager(), 10);
			todo!();
			true
		}
	}

	pub fn setup_video_flags(&mut self,
	                         full_screen: bool,
	                         flip_surfaces: bool,
	                         back_buffers: bool,
	                         using_8_bit: bool,
	                         using_16_bit: bool,
	                         light_support: bool,
	                         param_7: bool,
	                         wide_view_angle: bool,
	                         device_id: &str
	) -> bool {
		self.video_param.flags_mut().set_fullscreen(full_screen);
		self.video_param.flags_mut().set_flip_surfaces(flip_surfaces);
		self.video_param.flags_mut().set_back_buffers(!back_buffers);
		self.video_param.flags_mut().set_lacks_light_support(!param_6);
		self.video_param.flags_mut().set_f1_bit_7(param_7);
		self.video_param.
		todo!()
	}

	pub fn setup_window(&mut self, instance: HINSTANCE, cmd_line: &str) -> MxResult {
		todo!()
	}

	pub fn read_reg(&mut self, name: &str, out_value: &mut str, out_size: u32) -> bool {
		todo!()
	}

	pub fn read_reg_bool(&mut self, name: &str, out: &mut bool) -> bool {
		todo!()
	}

	pub fn read_reg_int(&mut self, name: &str, out: &mut i32) -> bool {
		todo!()
	}

	pub fn load_config(&mut self) {
		todo!()
	}

	pub fn tick(&mut self, sleep_if_not_next_frame: bool) {
		todo!()
	}

	pub fn setup_cursor(&mut self, param: WPARAM) {
		todo!()
	}

	pub fn get_window_handle(&self) -> Option<HWND> {
		self.window_handle
	}

	pub fn get_frame_delta(&self) -> i32 {
		self.frame_delta
	}

	pub fn get_full_screen(&self) -> bool {
		self.full_screen
	}

	pub fn get_cursor_current(&self) -> Option<HCURSOR> {
		self.cursor_current
	}

	pub fn get_cursor_busy(&self) -> Option<HCURSOR> {
		self.cursor_busy
	}

	pub fn get_cursor_no(&self) -> Option<HCURSOR> {
		self.cursor_no
	}

	pub fn get_draw_cursor(&self) -> bool {
		self.draw_cursor
	}

	pub fn set_window_active(&mut self, window_active: bool) {
		self.window_active = window_active;
	}
}

impl Drop for IsleApp {
	fn drop(&mut self) {
		if todo!() /* LegoOmni::GetInstance() */ {
			self.close();
			todo!() /* MxOmni::DestroyInstance() */
		}
	}
}

fn find_existing_instance() -> bool {
	todo!()
}

fn start_direct_sound() -> bool {
	todo!()
}
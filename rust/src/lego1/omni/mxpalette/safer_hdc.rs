use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::{GetDC, ReleaseDC, HDC};

pub struct SafeHDC(HDC, Option<HWND>);

impl SafeHDC {
	pub fn new(h_wnd: Option<HWND>) -> Self {
		Self {
			0: unsafe { GetDC(h_wnd) },
			1: h_wnd
		}
	}

	pub fn get(&self) -> HDC {
		self.0
	}
}

impl Drop for SafeHDC {
	fn drop(&mut self) {
		unsafe {
			ReleaseDC(self.1, self.0);
		}
	}
}
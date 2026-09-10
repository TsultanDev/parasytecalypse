use std::ptr;

#[cfg(target_os = "windows")]
use luxarust_system::windows::{Handle, Window};

#[cfg(target_os = "windows")]
#[derive(Default)]
pub struct CreateInfo<'a> {
    pub handle: Option<&'a Handle>,
    pub window: Option<&'a Window>,
}

#[cfg(target_os = "windows")]
impl<'a> CreateInfo<'a> {
    pub fn set_handle(mut self, handle: &'a Handle) -> Self {
        self.handle = Some(handle);
        self
    }
    pub fn set_window(mut self, window: &'a Window) -> Self {
        self.window = Some(window);
        self
    }
}
pub struct Surface(pub(super) ash::vk::SurfaceKHR);

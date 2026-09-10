mod handle;
mod message;
mod window;

pub use handle::{
    Exception as HandleException, Handle, LoadFlags as HandleLoadFlags, LoadInfo as HandleLoadInfo,
};

pub use message::Message;
pub use window::{
    Class as WindowClass, ClassRegisterFlags as WindowClassRegisterFlags,
    ClassRegisterInfo as WindowClassRegisterInfo, CreateFlags as WindowCreateFlags,
    CreateInfo as WindowCreateInfo, Exception as WindowException, Process as WindowProcess, Window,
    wnd_proc,
};
use windows::Win32::Foundation::{HINSTANCE, HWND};

pub fn bind_to_graphics_surface(handle: &Handle, window: &Window) -> (HINSTANCE, HWND) {
    (handle.0, window.0)
}

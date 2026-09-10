use luxarust_window::windows::{
    Window, WindowClass, WindowClassRegisterInfo, WindowCreateInfo, WindowException, wnd_proc,
};
use windows::{
    Win32::{
        Foundation::{GetLastError, HINSTANCE, HMODULE},
        Graphics::Gdi::{COLOR_WINDOW, HBRUSH},
        System::LibraryLoader::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN,
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleHandleExW,
        },
        UI::WindowsAndMessaging::{
            CW_USEDEFAULT, CreateWindowExW, IDC_ARROW, LoadCursorW, RegisterClassExW,
            WINDOW_EX_STYLE, WNDCLASS_STYLES, WNDCLASSEXW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
    core::PCWSTR,
};

#[derive(Debug)]
pub enum Exception {
    FailedToGetModule,
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
    pub struct LoadFlags: u32 {
        const NONE                        = 0x00000000;
        const GET_FROM_ADDRESS            = GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS;
        const GET_PIN                     = GET_MODULE_HANDLE_EX_FLAG_PIN;
        const UNCHANGED_REFERENCE_COUNT   = GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
    }
}

#[derive(Default)]
pub struct LoadInfo {
    pub module_name: &'static str,
    pub flags: LoadFlags,
}

impl LoadInfo {
    pub fn set_module_name(mut self, module_name: &'static str) -> Self {
        self.module_name = module_name;
        self
    }
    pub fn set_flags(mut self, flags: LoadFlags) -> Self {
        self.flags = flags;
        self
    }
}

pub struct Handle(HINSTANCE);

impl Handle {
    pub fn load(load_info: &LoadInfo) -> Result<Handle, Exception> {
        unsafe {
            let mut module = HMODULE::default();

            let pstr: Vec<u16> = load_info
                .module_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let module_pcwstr = if load_info.flags.contains(LoadFlags::GET_FROM_ADDRESS) {
                PCWSTR(load_info.module_name.as_ptr() as *const u16)
            } else if load_info.module_name.is_empty() {
                PCWSTR::null()
            } else {
                PCWSTR(pstr.as_ptr())
            };

            let result = GetModuleHandleExW(load_info.flags.bits(), module_pcwstr, &mut module);

            if result.is_err() || module.is_invalid() {
                return Err(Exception::FailedToGetModule);
            }

            Ok(Handle(HINSTANCE(module.0)))
        }
    }
    pub fn register_window_class(
        &self,
        window_class_info: &WindowClassRegisterInfo,
    ) -> Result<WindowClass, WindowException> {
        unsafe {
            let wstr: Vec<u16> = window_class_info
                .name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let w_name = wstr.as_ptr();

            let cursor = LoadCursorW(None, IDC_ARROW);
            let cursor = match cursor {
                Ok(c) => c,
                Err(_) => return Err(WindowException::LoadCursorFailed),
            };

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: WNDCLASS_STYLES(window_class_info.flags.bits()),
                lpfnWndProc: Some(wnd_proc),
                hInstance: self.0,
                hCursor: cursor,
                hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as _),
                lpszClassName: PCWSTR(w_name),
                ..Default::default()
            };

            let atom = RegisterClassExW(&wc);
            if atom == 0 {
                let error = GetLastError();
                eprintln!("Win32 Error : {:?}", error);
                return Err(WindowException::RegisterClassFailed);
            }

            Ok(WindowClass(wc, w_name))
        }
    }
    pub fn create_window(&self, create_info: &WindowCreateInfo) -> Result<Window, WindowException> {
        unsafe {
            let wstr: Vec<u16> = create_info
                .title
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            let w_name = wstr.as_ptr();

            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(create_info.flags.bits()),
                PCWSTR(create_info.class.unwrap().1), // lpClassName: Nama class yang didaftarkan
                PCWSTR(w_name),                       // lpWindowName: Judul pada title bar
                WS_OVERLAPPEDWINDOW | WS_VISIBLE, // dwStyle: Window style standar (Titlebar, Min, Max, Close, Border)
                CW_USEDEFAULT,                    // X: Posisi horizontal awal
                CW_USEDEFAULT,                    // Y: Posisi vertikal awal
                create_info.width as i32,         // nWidth: Lebar jendela (pixel)
                create_info.height as i32,        // nHeight: Tinggi jendela (pixel)
                None,         // hWndParent: HWND jendela induk (None jika top-level)
                None,         // hMenu: Handle ke menu (None jika tidak ada)
                Some(self.0), // hInstance: Handle modul biner .exe
                None,         // lpParam: Pointer data kustom ke WM_CREATE
            );
            let hwnd = match hwnd {
                Ok(h) => h,
                Err(_) => return Err(WindowException::CreateWindowFailed),
            };
            Ok(Window(hwnd))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Windows Only"]
    fn handle_load_success_default() {
        let handle_info = LoadInfo::default();
        let handle = Handle::load(&handle_info);
        assert!(handle.is_ok());
    }

    #[test]
    #[ignore = "Windows Only"]
    fn handle_load_kernel_from_address_default() {
        let handle_info = LoadInfo::default()
            .set_module_name("kernel32.dll")
            .set_flags(LoadFlags::GET_FROM_ADDRESS);
        let handle = Handle::load(&handle_info);
        assert!(handle.is_ok());
    }
}

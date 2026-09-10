use windows::{
    Win32::{
        Foundation::{HINSTANCE, HMODULE},
        System::LibraryLoader::{
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, GET_MODULE_HANDLE_EX_FLAG_PIN,
            GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, GetModuleHandleExW,
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
    pub struct HandleLoadFlags: u32 {
        const NONE                        = 0x00000000;
        const GET_FROM_ADDRESS            = GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS;
        const GET_PIN                     = GET_MODULE_HANDLE_EX_FLAG_PIN;
        const UNCHANGED_REFERENCE_COUNT   = GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
    }
}

#[derive(Default)]
pub struct HandleLoadInfo {
    pub module_name: &'static str,
    pub flags: HandleLoadFlags,
}

impl HandleLoadInfo {
    pub fn set_module_name(mut self, module_name: &'static str) -> Self {
        self.module_name = module_name;
        self
    }
    pub fn set_flags(mut self, flags: HandleLoadFlags) -> Self {
        self.flags = flags;
        self
    }
}

pub struct Handle(pub HINSTANCE);

impl Handle {
    pub fn load(load_info: &HandleLoadInfo) -> Result<Handle, Exception> {
        unsafe {
            let mut module = HMODULE::default();

            let pstr: Vec<u16> = load_info
                .module_name
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();

            let module_pcwstr = if load_info.flags.contains(HandleLoadFlags::GET_FROM_ADDRESS) {
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
}

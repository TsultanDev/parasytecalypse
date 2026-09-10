use std::ffi::{CStr, CString};

use ash::{
    Entry,
    vk::{ApplicationInfo, InstanceCreateInfo},
};

#[derive(Debug)]
pub enum Exception {
    LoadFailed,
    CreateInstanceFailed,
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LoadFlags: u32{
        const NONE = 0x00000000;
    }
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MessengerTypeFlags: u32{
        const NONE = 0x00000000;
        const DEVICE_ADRESS_BINDING = ash::vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING.as_raw();
        const GENERAL = ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL.as_raw();
        const PERFORMANCE = ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE.as_raw();
        const VALIDATION = ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION.as_raw();
    }
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MessengerSeverityFlags: u32{
        const NONE = 0x00000000;
        const DEVICE_ADRESS_BINDING = ash::vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING.as_raw();
        const GENERAL = ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL.as_raw();
        const PERFORMANCE = ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE.as_raw();
        const VALIDATION = ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION.as_raw();
    }
}

pub enum Extension {
    MessengerCreateInfo { types: MessengerTypeFlags },
}

pub enum Version {
    Version(u32, u32, u32),
    VariantVersion(u32, u32, u32, u32),
}
impl Default for Version {
    fn default() -> Self {
        Version::Version(0, 0, 0)
    }
}

#[derive(Default)]
pub struct LoadInfo {
    pub application_name: &'static str,
    pub version: Version,
}
impl LoadInfo {
    pub fn set_application_name(mut self, appplication_name: &'static str) -> Self {
        self.application_name = appplication_name;
        self
    }
    pub fn set_version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }
}

pub struct Loader(Entry, ash::Instance);
impl Loader {
    pub fn load(load_info: &LoadInfo) -> Result<Self, Exception> {
        unsafe {
            let entry = Entry::load();
            let entry = match entry {
                Ok(e) => e,
                Err(_) => return Err(Exception::LoadFailed),
            };
            let pstr = load_info.application_name.as_ptr();
            let app_name = CStr::from_ptr(pstr as *const i8);

            let engine_name = CString::new("Luxarust").unwrap();

            let app_version = match load_info.version {
                Version::Version(a, b, c) => ash::vk::make_api_version(0, a, b, c),
                Version::VariantVersion(a, b, c, d) => ash::vk::make_api_version(a, b, c, d),
            };

            let application_info = ApplicationInfo::default()
                .engine_name(&engine_name)
                .engine_version(ash::vk::make_api_version(0, 0, 1, 0))
                .application_name(&app_name)
                .application_version(app_version)
                .api_version(ash::vk::API_VERSION_1_1);

            let instance_info = InstanceCreateInfo::default().application_info(&application_info);

            let instance = entry.create_instance(&instance_info, None);
            let instance = match instance {
                Ok(i) => i,
                Err(_) => return Err(Exception::CreateInstanceFailed),
            };

            Ok(Loader(entry, instance))
        }
    }
    pub fn terminate(self) {
        unsafe {
            self.1.destroy_instance(None);
        }
    }
}
struct Messenger(
    Option<ash::ext::debug_utils::Instance>,
    ash::vk::DebugUtilsMessengerEXT,
);

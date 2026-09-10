use std::ffi::{CStr, CString};

use ash::{
    Entry,
    vk::{ApplicationInfo, InstanceCreateInfo},
};

use crate::vulkan::{GraphicsSurface, GraphicsSurfaceCreateInfo};

#[derive(Debug)]
pub enum Exception {
    LoadFailed,
    CreateInstanceFailed,
    CreateMessengerFailed,
    CreateGraphicsSurfaceFailed,
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct LoadFlags: u32{
        const NONE = 0x00000000;
        const ACTIVATE_MESSENGER = 0x00000001;
    }
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
    pub extensions: Vec<&'static str>,
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
    pub fn set_extensions(mut self, extensions: Vec<&'static str>) -> Self {
        self.extensions = extensions;
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

            let default_extensions = [
                ash::vk::KHR_SURFACE_NAME.as_ptr(),
                ash::vk::KHR_WIN32_SURFACE_NAME.as_ptr(),
            ];

            let instance_info = InstanceCreateInfo::default()
                .application_info(&application_info)
                .enabled_extension_names(&default_extensions);

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

    #[cfg(target_os = "windows")]
    pub fn create_window_surface(
        &self,
        create_info: &GraphicsSurfaceCreateInfo,
    ) -> Result<GraphicsSurface, Exception> {
        unsafe {
            let handle = match create_info.handle {
                Some(h) => h,
                None => return Err(Exception::CreateGraphicsSurfaceFailed),
            };
            let window = match create_info.window {
                Some(w) => w,
                None => return Err(Exception::CreateGraphicsSurfaceFailed),
            };
            use luxarust_system::windows::bind_to_graphics_surface;

            let surface_loader = ash::khr::win32_surface::Instance::new(&self.0, &self.1);
            let (hinstance, hwnd) = bind_to_graphics_surface(handle, window);
            let surface_info = ash::vk::Win32SurfaceCreateInfoKHR::default()
                .hinstance(hinstance.0 as isize)
                .hwnd(hwnd.0 as isize);
            let surface = surface_loader.create_win32_surface(&surface_info, None);
            let surface = match surface {
                Ok(s) => s,
                Err(_) => return Err(Exception::CreateGraphicsSurfaceFailed),
            };
            Ok(GraphicsSurface(surface))
        }
    }
    pub fn destroy_surface(&self, surface: GraphicsSurface) {
        unsafe {
            let surface_loader = ash::khr::surface::Instance::new(&self.0, &self.1);
            surface_loader.destroy_surface(surface.0, None);
        }
    }
}

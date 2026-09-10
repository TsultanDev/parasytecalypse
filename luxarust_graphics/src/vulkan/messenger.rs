bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TypeFlags: u32{
        const NONE = 0x00000000;
        const DEVICE_ADRESS_BINDING = ash::vk::DebugUtilsMessageTypeFlagsEXT::DEVICE_ADDRESS_BINDING.as_raw();
        const GENERAL = ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL.as_raw();
        const PERFORMANCE = ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE.as_raw();
        const VALIDATION = ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION.as_raw();
    }
}

bitflags::bitflags! {
    #[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct SeverityFlags: u32{
        const NONE = 0x00000000;
        const ERROR = ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR.as_raw();
        const VERBOSE = ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE.as_raw();
        const WARNING = ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING.as_raw();
        const INFORMATION = ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO.as_raw();
    }
}

pub struct CreateInfo {
    types: TypeFlags,
    severities: SeverityFlags,
}

pub struct Messenger(
    Option<ash::ext::debug_utils::Instance>,
    ash::vk::DebugUtilsMessengerEXT,
);

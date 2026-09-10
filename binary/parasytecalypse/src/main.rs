use luxarust::{
    window::windows::{WindowClassRegisterInfo, WindowCreateFlags, WindowCreateInfo},
    windows::{Handle, HandleLoadFlags, HandleLoadInfo},
};

fn main() {
    let handle_info = HandleLoadInfo::default()
        .set_module_name("")
        .set_flags(HandleLoadFlags::NONE);
    let handle = Handle::load(&handle_info).expect("Failed to load");

    let window_class_info = WindowClassRegisterInfo::default().set_name("Luxarust");
    let window_class = handle
        .register_window_class(&window_class_info)
        .expect("Failed to create");

    let window_info = WindowCreateInfo::default()
        .set_width(800)
        .set_heigth(600)
        .set_title("Luxarust")
        .set_flags(WindowCreateFlags::OVERLAPPEDWINDOW)
        .set_class(&window_class);
    let window = handle.create_window(&window_info).expect("Failed window");

    println!("Hello, world!");
}

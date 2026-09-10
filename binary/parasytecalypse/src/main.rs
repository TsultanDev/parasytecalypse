use luxarust::windows::{Handle, HandleLoadFlags, HandleLoadInfo};

fn main() {
    let handle_info = HandleLoadInfo::default()
        .set_module_name("")
        .set_flags(HandleLoadFlags::NONE);
    let _handle = Handle::load(&handle_info).expect("Failed to load");

    println!("Hello, world!");
}

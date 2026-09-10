use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, MSG, PM_REMOVE, PeekMessageW, TranslateMessage, WM_QUIT,
};

pub struct Message(pub MSG);

impl Message {
    pub fn new() -> Self {
        Self(MSG::default())
    }

    pub fn message_blocking(&mut self) {
        unsafe {
            while GetMessageW(&mut self.0, None, 0, 0).into() {
                _ = TranslateMessage(&self.0);
                DispatchMessageW(&self.0);
            }
        }
    }

    pub fn message_non_blocking<F>(&mut self, mut on_idle: F)
    where
        F: FnMut(),
    {
        unsafe {
            loop {
                while PeekMessageW(&mut self.0, None, 0, 0, PM_REMOVE).into() {
                    if self.0.message == WM_QUIT {
                        return;
                    }

                    _ = TranslateMessage(&self.0);
                    DispatchMessageW(&self.0);
                }

                on_idle();
            }
        }
    }
}

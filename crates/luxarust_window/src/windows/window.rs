use windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    UI::WindowsAndMessaging::{
        CS_DBLCLKS, CS_DROPSHADOW, CS_HREDRAW, CS_NOCLOSE, CS_OWNDC, CS_VREDRAW, DefWindowProcW,
        PostQuitMessage, WM_CREATE, WM_DESTROY, WNDCLASSEXW, WNDPROC, WS_EX_ACCEPTFILES,
        WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_COMPOSITED, WS_EX_CONTEXTHELP,
        WS_EX_CONTROLPARENT, WS_EX_DLGMODALFRAME, WS_EX_LAYERED, WS_EX_LAYOUTRTL, WS_EX_LEFT,
        WS_EX_LEFTSCROLLBAR, WS_EX_LTRREADING, WS_EX_MDICHILD, WS_EX_NOACTIVATE,
        WS_EX_NOINHERITLAYOUT, WS_EX_NOPARENTNOTIFY, WS_EX_NOREDIRECTIONBITMAP,
        WS_EX_OVERLAPPEDWINDOW, WS_EX_PALETTEWINDOW, WS_EX_RIGHT, WS_EX_RIGHTSCROLLBAR,
        WS_EX_RTLREADING, WS_EX_STATICEDGE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_EX_TRANSPARENT,
        WS_EX_WINDOWEDGE,
    },
};

#[derive(Debug)]
pub enum Exception {
    LoadCursorFailed,
    RegisterClassFailed,
    CreateWindowFailed,
}

pub type Process = WNDPROC;

bitflags::bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ClassRegisterFlags: u32 {
        const NONE              = 0;
        const HORIZONTAL_REDRAW = CS_HREDRAW.0;
        const VERTICAL_REDRAW   = CS_VREDRAW.0;
        const DOUBLE_CLICKS     = CS_DBLCLKS.0;
        const DROP_SHADOW       = CS_DROPSHADOW.0;
        const OWN_DC            = CS_OWNDC.0;
        const NO_CLOSE          = CS_NOCLOSE.0;
    }
}

#[derive(Default)]
pub struct ClassRegisterInfo {
    pub name: &'static str,
    pub flags: ClassRegisterFlags,
    pub process: Process,
}
impl ClassRegisterInfo {
    pub fn set_name(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
    pub fn set_flags(mut self, flags: ClassRegisterFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn set_process(mut self, process: Process) -> Self {
        self.process = process;
        self
    }
}

pub struct Class(pub WNDCLASSEXW, pub *const u16);

bitflags::bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CreateFlags: u32 {
        const NONE                  = 0;
        const DLGMODALFRAME         = WS_EX_DLGMODALFRAME.0;
        const NOPARENTNOTIFY        = WS_EX_NOPARENTNOTIFY.0;
        const TOPMOST               = WS_EX_TOPMOST.0;
        const ACCEPTFILES           = WS_EX_ACCEPTFILES.0;
        const TRANSPARENT           = WS_EX_TRANSPARENT.0;
        const MDICHILD              = WS_EX_MDICHILD.0;
        const TOOLWINDOW            = WS_EX_TOOLWINDOW.0;
        const WINDOWEDGE            = WS_EX_WINDOWEDGE.0;
        const CLIENTEDGE            = WS_EX_CLIENTEDGE.0;
        const CONTEXTHELP           = WS_EX_CONTEXTHELP.0;
        const RIGHT                 = WS_EX_RIGHT.0;
        const LEFT                  = WS_EX_LEFT.0;
        const RTLREADING            = WS_EX_RTLREADING.0;
        const LTRREADING            = WS_EX_LTRREADING.0;
        const LEFTSCROLLBAR         = WS_EX_LEFTSCROLLBAR.0;
        const RIGHTSCROLLBAR        = WS_EX_RIGHTSCROLLBAR.0;
        const CONTROLPARENT         = WS_EX_CONTROLPARENT.0;
        const STATICEDGE            = WS_EX_STATICEDGE.0;
        const APPWINDOW             = WS_EX_APPWINDOW.0;
        const LAYERED               = WS_EX_LAYERED.0;
        const NOINHERITLAYOUT       = WS_EX_NOINHERITLAYOUT.0;
        const NOREDIRECTIONBITMAP  = WS_EX_NOREDIRECTIONBITMAP.0;
        const LAYOUTRTL             = WS_EX_LAYOUTRTL.0;
        const COMPOSITED            = WS_EX_COMPOSITED.0;
        const NOACTIVATE            = WS_EX_NOACTIVATE.0;

        // Combined Presets
        const OVERLAPPEDWINDOW      = WS_EX_OVERLAPPEDWINDOW.0;
        const PALETTEWINDOW         = WS_EX_PALETTEWINDOW.0;
    }
}

#[derive(Default)]
pub struct CreateInfo<'a> {
    pub title: &'static str,
    pub width: u32,
    pub height: u32,
    pub flags: CreateFlags,
    pub class: Option<&'a Class>,
}
impl<'a> CreateInfo<'a> {
    pub fn set_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }
    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }
    pub fn set_heigth(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
    pub fn set_flags(mut self, flags: CreateFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn set_class(mut self, class: &'a Class) -> Self {
        self.class = Some(class);
        self
    }
}

pub unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                // Jika ini Return Err, .unwrap() memicu panic -> crash 0xc000041d

                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

pub struct Window(pub HWND);
impl Window {}

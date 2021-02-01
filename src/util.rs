use {
    crate::bindings::windows::win32::{
        display_devices::RECT,
        dwm::DwmIsCompositionEnabled,
        system_services::{FALSE, TRUE, WS_CAPTION, WS_OVERLAPPEDWINDOW},
        windows_and_messaging::{AdjustWindowRectEx, WINDOWPOS_abi, HWND},
    },
    raw_window_handle::{HasRawWindowHandle, RawWindowHandle},
};

pub(crate) fn windows_window_handle<W: HasRawWindowHandle>(window: &W) -> HWND {
    // Get the window handle
    let window_handle = window.raw_window_handle();
    let window_handle = match window_handle {
        RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
        _ => panic!("Unsupported platform!"),
    };
    HWND(window_handle as isize)
}

pub(crate) unsafe fn is_dwm_enabled() -> bool {
    let mut f_dwm_enabled = FALSE;
    let dwm_enabled_result = DwmIsCompositionEnabled(&mut f_dwm_enabled);

    f_dwm_enabled == TRUE && dwm_enabled_result.is_ok()
}

#[repr(C)]
pub(crate) struct NCCALCSIZE_PARAMS {
    pub rgrc: [RECT; 3],
    pub lppos: *mut WINDOWPOS_abi,
}

pub(crate) unsafe fn window_frame_borders(with_caption: bool) -> RECT {
    let style_flags = if with_caption {
        WS_OVERLAPPEDWINDOW
    } else {
        WS_OVERLAPPEDWINDOW & !WS_CAPTION
    };

    let mut rect = RECT::default();
    AdjustWindowRectEx(&mut rect, style_flags, false.into(), 0);
    rect
}

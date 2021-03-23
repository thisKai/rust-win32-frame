use {
    crate::bindings::windows::win32::{
        display_devices::RECT,
        dwm::DwmIsCompositionEnabled,
        system_services::{BOOL, FALSE},
        windows_and_messaging::{AdjustWindowRectEx, HWND, WINDOWS_STYLE},
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
    let mut f_dwm_enabled = BOOL(FALSE);
    let dwm_enabled_result = DwmIsCompositionEnabled(&mut f_dwm_enabled);

    f_dwm_enabled.as_bool() && dwm_enabled_result.is_ok()
}

pub(crate) unsafe fn window_frame_borders(with_caption: bool) -> RECT {
    let style_flags = if with_caption {
        WINDOWS_STYLE::WS_OVERLAPPEDWINDOW
    } else {
        WINDOWS_STYLE::WS_OVERLAPPEDWINDOW & !WINDOWS_STYLE::WS_CAPTION
    };

    let mut rect = RECT::default();
    AdjustWindowRectEx(&mut rect, style_flags.0, false, 0);
    rect
}

impl std::ops::Not for WINDOWS_STYLE {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

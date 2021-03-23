fn main() {
    windows::build!(
        windows::win32::system_services::{
            TRUE,
            FALSE,
            NTSTATUS,
        },
        windows::win32::windows_and_messaging::{
            GetWindowRect,
            AdjustWindowRectEx,
            SetWindowPos,
            NCCALCSIZE_PARAMS,
            WM_CREATE,
            WM_ACTIVATE,
            WM_NCCALCSIZE,
            WM_NCHITTEST,
            HTTOPLEFT, HTTOP, HTCAPTION, HTTOPRIGHT,
            HTLEFT, HTNOWHERE, HTRIGHT,
            HTBOTTOMLEFT, HTBOTTOM, HTBOTTOMRIGHT,
        },
        windows::win32::windows_and_messaging::WINDOWS_STYLE,
        windows::win32::shell::{
            SetWindowSubclass,
            RemoveWindowSubclass,
            DefSubclassProc,
        },
        windows::win32::dwm::{
            DwmExtendFrameIntoClientArea,
            DwmIsCompositionEnabled,
            DwmDefWindowProc,
        },
    );
}

fn main() {
    windows::build!(
        windows::win32::system_services::{
            TRUE,
            FALSE,
            WM_CREATE,
            WM_ACTIVATE,
            WM_NCCALCSIZE,
            WM_NCHITTEST,
            WS_CAPTION,
            WS_OVERLAPPEDWINDOW,
            SWP_FRAMECHANGED,
            HTTOPLEFT, HTTOP, HTCAPTION, HTTOPRIGHT,
            HTLEFT, HTNOWHERE, HTRIGHT,
            HTBOTTOMLEFT, HTBOTTOM, HTBOTTOMRIGHT,
            NTSTATUS,
        }
        windows::win32::windows_and_messaging::{
            GetWindowRect,
            AdjustWindowRectEx,
            SetWindowPos,
            NCCALCSIZE_PARAMS,
        }
        windows::win32::shell::{
            SetWindowSubclass,
            RemoveWindowSubclass,
            DefSubclassProc,
        }
        windows::win32::dwm::{
            DwmExtendFrameIntoClientArea,
            DwmIsCompositionEnabled,
            DwmDefWindowProc,
        }
    );
}

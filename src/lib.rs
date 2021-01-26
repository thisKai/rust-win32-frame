mod bindings {
    ::windows::include_bindings!();
}
mod hit_test;
mod options;

pub use options::*;
use {
    bindings::windows::win32::{
        display_devices::RECT,
        dwm::{DwmDefWindowProc, DwmExtendFrameIntoClientArea, DwmIsCompositionEnabled},
        shell::{DefSubclassProc, RemoveWindowSubclass, SetWindowSubclass},
        system_services::{
            FALSE, HTCAPTION, HTNOWHERE, LRESULT, TRUE, WM_ACTIVATE, WM_NCCALCSIZE, WM_NCHITTEST,
            WS_CAPTION, WS_OVERLAPPEDWINDOW,
        },
        windows_and_messaging::{AdjustWindowRectEx, WINDOWPOS_abi, HWND, LPARAM, WPARAM},
    },
    hit_test::hit_test_nca,
    raw_window_handle::{HasRawWindowHandle, RawWindowHandle},
    std::ops::{Deref, DerefMut},
};

pub struct SubClassedWindow<W: HasRawWindowHandle> {
    id: usize,
    window: W,
    _options: Box<Options>,
}
impl<W: HasRawWindowHandle> SubClassedWindow<W> {
    pub fn wrap(window: W, options: Options) -> windows::Result<Self> {
        Self::wrap_with_id(window, options, 1)
    }
    pub fn wrap_with_id(window: W, options: Options, subclass_id: usize) -> windows::Result<Self> {
        let h_wnd = windows_window_handle(&window);
        let options = Box::new(options);
        let options_ptr = &*options as *const Options;
        unsafe {
            SetWindowSubclass(
                h_wnd,
                Some(subclass_procedure),
                subclass_id,
                options_ptr as usize,
            )
            .ok()?;
        }
        Ok(Self {
            id: subclass_id,
            window,
            _options: options,
        })
    }
    pub fn unwrap(self) -> windows::Result<W> {
        let h_wnd = windows_window_handle(&self.window);
        unsafe {
            RemoveWindowSubclass(h_wnd, Some(subclass_procedure), self.id).ok()?;
        }
        Ok(self.window)
    }
}
fn windows_window_handle<W: HasRawWindowHandle>(window: &W) -> HWND {
    // Get the window handle
    let window_handle = window.raw_window_handle();
    let window_handle = match window_handle {
        RawWindowHandle::Windows(window_handle) => window_handle.hwnd,
        _ => panic!("Unsupported platform!"),
    };
    HWND(window_handle as isize)
}
impl<W: HasRawWindowHandle> Deref for SubClassedWindow<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
impl<W: HasRawWindowHandle> DerefMut for SubClassedWindow<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

pub trait WithSubclass: HasRawWindowHandle + Sized {
    fn with_subclass(self, options: Options) -> windows::Result<SubClassedWindow<Self>>;
}
impl<W: HasRawWindowHandle> WithSubclass for W {
    fn with_subclass(self, options: Options) -> windows::Result<SubClassedWindow<Self>> {
        SubClassedWindow::wrap(self, options)
    }
}

extern "system" fn subclass_procedure(
    h_wnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
    _u_id_subclass: usize,
    dw_ref_data: usize,
) -> LRESULT {
    unsafe {
        if is_dwm_enabled() {
            let options = &*(dw_ref_data as *const Options);

            let msg = u_msg as i32;

            let (dwm_result, dwm_handled) = {
                let mut result = LRESULT(0);
                let handled = DwmDefWindowProc(h_wnd, u_msg, w_param, l_param, &mut result).is_ok();
                (result, handled)
            };

            // if msg == WM_CREATE {
            //     let mut rect = RECT::default();
            //     GetWindowRect(h_wnd, &mut rect);

            //     // Inform application of the frame change.
            //     let width = rect.right - rect.left;
            //     let height = rect.bottom - rect.top;

            //     // Extend the frame into the client area.
            //     let p_mar_inset = MARGINS {
            //         cy_top_height: 1,
            //         ..Default::default()
            //     };
            //     DwmExtendFrameIntoClientArea(h_wnd, &p_mar_inset);
            //     SetWindowPos(
            //         h_wnd,
            //         HWND(0),
            //         rect.left,
            //         rect.top,
            //         width,
            //         height,
            //         SWP_FRAMECHANGED as _,
            //     );
            // }
            if msg == WM_ACTIVATE {
                // Extend the frame into the client area.
                let p_mar_inset = options.extend_frame.to_win32();
                DwmExtendFrameIntoClientArea(h_wnd, &p_mar_inset);
            }
            if msg == WM_NCCALCSIZE && w_param == WPARAM(TRUE as _) {
                let Options {
                    extend_client_area: adjust_client_area, ..
                } = options;

                // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
                let pncsp = &mut *(l_param.0 as *mut NCCALCSIZE_PARAMS);

                pncsp.rgrc[0].left -= adjust_client_area.left;
                pncsp.rgrc[0].top -= adjust_client_area.top;
                pncsp.rgrc[0].right += adjust_client_area.right;
                pncsp.rgrc[0].bottom += adjust_client_area.bottom;
            }
            if msg == WM_NCHITTEST && dwm_result == LRESULT(0) {
                let hit_test_result = hit_test_nca(h_wnd, l_param);

                if hit_test_result == LRESULT(HTNOWHERE) {
                    return LRESULT(HTCAPTION);
                }
                return hit_test_result;
            }

            if dwm_handled {
                return dwm_result;
            }
        }

        DefSubclassProc(h_wnd, u_msg, w_param, l_param)
    }
}

unsafe fn is_dwm_enabled() -> bool {
    let mut f_dwm_enabled = FALSE;
    let dwm_enabled_result = DwmIsCompositionEnabled(&mut f_dwm_enabled);

    f_dwm_enabled == TRUE && dwm_enabled_result.is_ok()
}

#[repr(C)]
struct NCCALCSIZE_PARAMS {
    pub rgrc: [RECT; 3],
    pub lppos: *mut WINDOWPOS_abi,
}

unsafe fn window_frame_borders(with_caption: bool) -> RECT {
    let style_flags = if with_caption {
        WS_OVERLAPPEDWINDOW
    } else {
        WS_OVERLAPPEDWINDOW & !WS_CAPTION
    };

    let mut rect = RECT::default();
    AdjustWindowRectEx(&mut rect, style_flags, false.into(), 0);
    rect
}

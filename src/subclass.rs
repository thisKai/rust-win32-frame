use crate::{
    bindings::windows::win32::{
        dwm::{DwmDefWindowProc, DwmExtendFrameIntoClientArea},
        shell::DefSubclassProc,
        system_services::{LRESULT, TRUE, WM_ACTIVATE, WM_NCCALCSIZE, WM_NCHITTEST},
        windows_and_messaging::{HWND, LPARAM, WPARAM},
    },
    hit_test::{non_client_hit_test, transform_hit_test, HitTestArea, Point, WindowMetrics},
    options::WindowFrame,
    util::{is_dwm_enabled, NCCALCSIZE_PARAMS},
};

pub(crate) extern "system" fn subclass_procedure(
    h_wnd: HWND,
    u_msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
    _u_id_subclass: usize,
    dw_ref_data: usize,
) -> LRESULT {
    unsafe {
        if is_dwm_enabled() {
            let options = &*(dw_ref_data as *const WindowFrame);

            let msg = u_msg as i32;

            let (dwm_result, dwm_handled) = {
                if options.hit_test_caption_buttons {
                    let mut result = LRESULT(0);
                    let handled =
                        DwmDefWindowProc(h_wnd, u_msg, w_param, l_param, &mut result).is_ok();
                    (result, handled)
                } else {
                    (LRESULT(0), false)
                }
            };

            if msg == WM_ACTIVATE {
                // Extend the frame into the client area.
                let p_mar_inset = options.extend_frame.to_win32();
                DwmExtendFrameIntoClientArea(h_wnd, &p_mar_inset);
            }
            if msg == WM_NCCALCSIZE && w_param == WPARAM(TRUE as _) {
                let WindowFrame {
                    extend_client_area: adjust_client_area,
                    ..
                } = options;

                // Calculate new NCCALCSIZE_PARAMS based on custom NCA inset.
                let pncsp = &mut *(l_param.0 as *mut NCCALCSIZE_PARAMS);

                pncsp.rgrc[0].left -= adjust_client_area.left;
                pncsp.rgrc[0].top -= adjust_client_area.top;
                pncsp.rgrc[0].right += adjust_client_area.right;
                pncsp.rgrc[0].bottom += adjust_client_area.bottom;
            }
            if msg == WM_NCHITTEST && !dwm_handled {
                let global_position = Point::from_l_param(l_param);

                let metrics = WindowMetrics::new(h_wnd);

                let def_hit_test = non_client_hit_test(global_position, &metrics, options);
                let hit_test = transform_hit_test(def_hit_test, options);
                if !matches!(hit_test, HitTestArea::Client) {
                    return hit_test.l_result();
                }
            }

            if dwm_handled {
                return dwm_result;
            }
        }

        DefSubclassProc(h_wnd, u_msg, w_param, l_param)
    }
}

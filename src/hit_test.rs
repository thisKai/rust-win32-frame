use crate::{
    bindings::windows::win32::{
        display_devices::RECT,
        system_services::{
            HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION, HTLEFT, HTNOWHERE, HTRIGHT, HTTOP,
            HTTOPLEFT, HTTOPRIGHT, LRESULT,
        },
        windows_and_messaging::{GetWindowRect, HWND, LPARAM},
    },
    window_frame_borders,
};

pub(crate) unsafe fn hit_test_nca(h_wnd: HWND, l_param: LPARAM) -> LRESULT {
    // Get the point coordinates for the hit test.
    let (x, y) = get_l_param_point(l_param);

    // Get the window rectangle.
    let mut window_rect = RECT::default();
    GetWindowRect(h_wnd, &mut window_rect);

    // Get the frame rectangle, adjusted for the style without a caption.
    let frame_rect = window_frame_borders(false);

    // Get the frame rectangle, adjusted for the style with a caption.
    let caption_frame_rect = window_frame_borders(true);

    // Determine if the hit test is for resizing. Default middle (1,1).
    let mut row = 1;
    let mut col = 1;
    let mut on_resize_border = false;

    // Determine if the point is at the top or bottom of the window.
    if y >= window_rect.top && y < window_rect.top - caption_frame_rect.top {
        on_resize_border = y < (window_rect.top - frame_rect.top);
        row = 0;
    } else if y < window_rect.bottom && y >= window_rect.bottom - caption_frame_rect.bottom {
        row = 2;
    }

    // Determine if the point is at the left or right of the window.
    if x >= window_rect.left && x < window_rect.left - caption_frame_rect.left {
        col = 0; // left side
    } else if x < window_rect.right && x >= window_rect.right - caption_frame_rect.right {
        col = 2; // right side
    }

    // Hit test (HTTOPLEFT, ... HTBOTTOMRIGHT)
    let hit_tests = [
        [
            HTTOPLEFT,
            if on_resize_border { HTTOP } else { HTCAPTION },
            HTTOPRIGHT,
        ],
        [HTLEFT, HTNOWHERE, HTRIGHT],
        [HTBOTTOMLEFT, HTBOTTOM, HTBOTTOMRIGHT],
    ];
    LRESULT(hit_tests[row][col])
}

fn get_l_param_point(lp: LPARAM) -> (i32, i32) {
    (
        lo_word(lp.0 as u32) as i16 as i32,
        hi_word(lp.0 as u32) as i16 as i32,
    )
}

const fn lo_word(l: u32) -> u16 {
    (l & 0xffff) as u16
}
const fn hi_word(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

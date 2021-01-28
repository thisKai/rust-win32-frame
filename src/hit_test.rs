use crate::{
    bindings::windows::win32::{
        display_devices::RECT,
        system_services::{
            HTBOTTOM, HTBOTTOMLEFT, HTBOTTOMRIGHT, HTCAPTION, HTLEFT, HTNOWHERE, HTRIGHT, HTTOP,
            HTTOPLEFT, HTTOPRIGHT, LRESULT,
        },
        windows_and_messaging::{GetWindowRect, HWND, LPARAM},
    },
    window_frame_borders, WindowFrame,
};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn from_l_param(l_param: LPARAM) -> Self {
        let (x, y) = get_l_param_point(l_param);
        Self { x, y }
    }
}

pub struct Size {
    pub width: i32,
    pub height: i32,
}

pub struct HitTest {
    pub area: HitTestArea,
    pub client_position: Point,
    pub client_size: Size,
}
#[derive(Debug, Clone, Copy)]
pub enum HitTestArea {
    Caption,
    Resize(Border),
    Client,
}
impl HitTestArea {
    pub fn l_result(&self) -> LRESULT {
        match self {
            Self::Caption => LRESULT(HTCAPTION),
            Self::Resize(border) => border.l_result(),
            Self::Client => LRESULT(HTNOWHERE),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExtentHitTest {
    Extent(Border),
    ClientArea(Point),
}

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum Border {
    TopLeft = HTTOPLEFT,
    Top = HTTOP,
    TopRight = HTTOPRIGHT,
    Left = HTLEFT,
    Right = HTRIGHT,
    BottomLeft = HTBOTTOMLEFT,
    Bottom = HTBOTTOM,
    BottomRight = HTBOTTOMRIGHT,
}
impl Border {
    pub fn l_result(&self) -> LRESULT {
        LRESULT(*self as i32)
    }
}

pub struct WindowMetrics {
    window: RECT,
    frame: WindowFrameMetrics,
}
impl WindowMetrics {
    pub unsafe fn new(h_wnd: HWND) -> Self {
        // Get the window rectangle.
        let mut rect = RECT::default();
        GetWindowRect(h_wnd, &mut rect);
        Self {
            window: rect,
            frame: WindowFrameMetrics::new(),
        }
    }
}
pub struct WindowFrameMetrics {
    adjust_resize_borders: RECT,
    adjust_caption: i32,
}
impl WindowFrameMetrics {
    pub unsafe fn new() -> Self {
        // Get the frame rectangle, adjusted for the style without a caption.
        let frame_rect = window_frame_borders(false);

        // Get the frame rectangle, adjusted for the style with a caption.
        let caption_frame_rect = window_frame_borders(true);

        Self {
            adjust_resize_borders: frame_rect,
            adjust_caption: caption_frame_rect.top,
        }
    }
}

pub(crate) unsafe fn non_client_hit_test(
    point: Point,
    metrics: &WindowMetrics,
    options: &WindowFrame,
) -> HitTest {
    // Get the point coordinates for the hit test.
    let Point { x, y } = point;

    let WindowMetrics { window, frame } = metrics;

    // Determine if the hit test is for resizing. Default middle (1,1).
    let mut row = 1;
    let mut col = 1;
    let top_resize_border = y < window.top - frame.adjust_resize_borders.top;

    let client_area_top = window.top - frame.adjust_caption - options.extend_client_area.top;
    let client_area_bottom = window.bottom - frame.adjust_resize_borders.bottom;
    // Determine if the point is at the top or bottom of the window.
    if top_resize_border || (y >= window.top && y < client_area_top) {
        row = 0;
    } else if y < window.bottom && y >= client_area_bottom {
        row = 2;
    }

    let client_area_left = window.left - frame.adjust_resize_borders.left;
    let client_area_right = window.right - frame.adjust_resize_borders.right;
    // Determine if the point is at the left or right of the window.
    if x >= window.left && x < client_area_left {
        col = 0; // left side
    } else if x < window.right && x >= client_area_right {
        col = 2; // right side
    }

    // Hit test (HTTOPLEFT, ... HTBOTTOMRIGHT)
    let hit_tests = [
        [
            HitTestArea::Resize(Border::TopLeft),
            if top_resize_border {
                HitTestArea::Resize(Border::Top)
            } else {
                HitTestArea::Caption
            },
            HitTestArea::Resize(Border::TopRight),
        ],
        [
            HitTestArea::Resize(Border::Left),
            HitTestArea::Client,
            HitTestArea::Resize(Border::Right),
        ],
        [
            HitTestArea::Resize(Border::BottomLeft),
            HitTestArea::Resize(Border::Bottom),
            HitTestArea::Resize(Border::BottomRight),
        ],
    ];
    HitTest {
        area: hit_tests[row][col],
        client_position: Point {
            x: x - client_area_left,
            y: y - client_area_top,
        },
        client_size: Size {
            width: client_area_right - client_area_left,
            height: client_area_bottom - client_area_top,
        },
    }
}

pub(crate) unsafe fn extent_hit_test(
    point: Point,
    client_size: Size,
    options: &WindowFrame,
) -> ExtentHitTest {
    // Get the point coordinates for the hit test.
    let Point { x, y } = point;

    // Determine if the hit test is for resizing. Default middle (1,1).
    let mut row = 1;
    let mut col = 1;

    let client_area_top = options.extend_frame.top;
    let client_area_bottom = client_size.height - options.extend_frame.bottom;
    // Determine if the point is at the top or bottom of the window.
    if y >= 0 && y < client_area_top {
        row = 0;
    } else if y < client_size.height && y >= client_area_bottom {
        row = 2;
    }

    let client_area_left = options.extend_frame.left;
    let client_area_right = client_size.width - options.extend_frame.right;
    // Determine if the point is at the left or right of the window.
    if x >= 0 && x < client_area_left {
        col = 0; // left side
    } else if x < client_size.width && x >= client_area_right {
        col = 2; // right side
    }

    // Hit test (HTTOPLEFT, ... HTBOTTOMRIGHT)
    let hit_tests = [
        [
            ExtentHitTest::Extent(Border::TopLeft),
            ExtentHitTest::Extent(Border::Top),
            ExtentHitTest::Extent(Border::TopRight),
        ],
        [
            ExtentHitTest::Extent(Border::Left),
            ExtentHitTest::ClientArea(Point {
                x: x - client_area_left,
                y: y - client_area_top,
            }),
            ExtentHitTest::Extent(Border::Right),
        ],
        [
            ExtentHitTest::Extent(Border::BottomLeft),
            ExtentHitTest::Extent(Border::Bottom),
            ExtentHitTest::Extent(Border::BottomRight),
        ],
    ];
    hit_tests[row][col]
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

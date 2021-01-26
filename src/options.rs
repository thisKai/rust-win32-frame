use crate::{bindings::windows::win32::controls::MARGINS, window_frame_borders};

pub struct Options {
    pub extend_frame: Margins,
    pub extend_client_area: Margins,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Margins {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Margins {
    pub fn sheet() -> Self {
        Self {
            left: -1,
            top: -1,
            right: -1,
            bottom: -1,
        }
    }
    pub fn caption(caption_height: i32) -> Self {
        Self {
            top: caption_height,
            ..Default::default()
        }
    }
    pub fn default_caption() -> Self {
        let frame_rect = unsafe { window_frame_borders(true) };
        Self {
            top: -frame_rect.top,
            ..Default::default()
        }
    }
    pub(crate) fn to_win32(&self) -> MARGINS {
        MARGINS {
            cx_left_width: self.left,
            cy_top_height: self.top,
            cx_right_width: self.right,
            cy_bottom_height: self.bottom,
        }
    }
}

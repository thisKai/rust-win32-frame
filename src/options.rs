use crate::{bindings::windows::win32::controls::MARGINS, window_frame_borders};

#[derive(Debug, Default)]
pub struct Options {
    pub extend_frame: Margins,
    pub extend_client_area: Margins,
    pub hit_test_extended_caption: bool,
    pub hit_test_extended_resize_borders: bool,
}
impl Options {
    pub fn extended_caption(extra_height: i32) -> Self {
        Self {
            extend_frame: Margins::caption(extra_height),
            extend_client_area: Margins::default(),
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
        }
    }
    pub fn custom_caption() -> Self {
        Self {
            extend_frame: Margins::default_caption(),
            extend_client_area: Margins::default_caption(),
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
        }
    }
    pub fn extended_custom_caption(extra_height: i32) -> Self {
        Self {
            extend_frame: Margins::extended_caption(extra_height),
            extend_client_area: Margins::default_caption(),
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
        }
    }
    pub fn custom_caption_height(caption_height: i32) -> Self {
        Self {
            extend_frame: Margins::caption(caption_height),
            extend_client_area: Margins::default_caption(),
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
        }
    }
    pub fn remove_caption() -> Self {
        Self {
            extend_frame: Margins::default(),
            extend_client_area: Margins::default_caption(),
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
        }
    }
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
    pub fn extended_caption(caption_height: i32) -> Self {
        let frame_rect = unsafe { window_frame_borders(true) };
        Self {
            top: caption_height - frame_rect.top,
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

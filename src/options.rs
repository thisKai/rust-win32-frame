use crate::{
    bindings::windows::win32::controls::MARGINS,
    hit_test::{HitTestArea, Point, Size},
    window_frame_borders,
};

#[derive(Default)]
pub struct WindowFrame {
    pub extend_frame: Margins,
    pub extend_client_area: Margins,
    pub hit_test_caption_buttons: bool,
    pub hit_test_extended_caption: bool,
    pub hit_test_extended_resize_borders: bool,
    pub intercept_client_area_hit_test: Option<HitTestIntercept>,
    pub intercept_top_resize_border_hit_test: Option<HitTestIntercept>,
}
pub type HitTestIntercept = Box<dyn Fn(&Point, &Size) -> Option<HitTestArea>>;
impl WindowFrame {
    pub fn extended_caption(extra_height: i32) -> Self {
        Self {
            extend_frame: Margins::caption(extra_height),
            extend_client_area: Margins::default(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
        }
    }
    pub fn sheet() -> Self {
        Self {
            extend_frame: Margins::sheet(),
            extend_client_area: Margins::default(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
        }
    }
    pub fn custom_caption() -> Self {
        Self {
            extend_frame: Margins::default_caption(),
            extend_client_area: Margins::default_caption(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
        }
    }
    pub fn extended_custom_caption(extra_height: i32) -> Self {
        Self {
            extend_frame: Margins::extended_caption(extra_height),
            extend_client_area: Margins::default_caption(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
        }
    }
    pub fn custom_sheet() -> Self {
        Self {
            extend_frame: Margins::sheet(),
            extend_client_area: Margins::default_caption(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
        }
    }
    pub fn custom_caption_height(caption_height: i32) -> Self {
        Self {
            extend_frame: Margins::caption(caption_height),
            extend_client_area: Margins::default_caption(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
        }
    }
    pub fn remove_caption() -> Self {
        Self {
            extend_frame: Margins::default(),
            extend_client_area: Margins::default_caption(),
            hit_test_caption_buttons: true,
            hit_test_extended_caption: true,
            hit_test_extended_resize_borders: false,
            intercept_client_area_hit_test: None,
            intercept_top_resize_border_hit_test: None,
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
        Self {
            top: system_caption_height(),
            ..Default::default()
        }
    }
    pub fn extended_caption(caption_height: i32) -> Self {
        Self {
            top: caption_height + system_caption_height(),
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
    pub(crate) fn zero_if_sheet(&self) -> Self {
        Self {
            left: if self.left < 0 { 0 } else { self.left },
            top: if self.top < 0 { 0 } else { self.top },
            right: if self.right < 0 { 0 } else { self.right },
            bottom: if self.bottom < 0 { 0 } else { self.bottom },
        }
    }
}

pub fn system_caption_height() -> i32 {
    let frame_rect = unsafe { window_frame_borders(true) };
    -frame_rect.top
}

mod bindings {
    ::windows::include_bindings!();
}
mod dark_mode;
mod hit_test;
mod options;
mod subclass;
mod util;

use {
    bindings::windows::win32::{
        display_devices::RECT,
        dwm::DwmExtendFrameIntoClientArea,
        shell::{RemoveWindowSubclass, SetWindowSubclass},
        windows_and_messaging::{GetWindowRect, SetWindowPos, SetWindowPos_uFlags, HWND},
    },
    dark_mode::dark_dwm_decorations,
    raw_window_handle::HasRawWindowHandle,
    std::{
        cell::Cell,
        ops::{Deref, DerefMut},
    },
    subclass::subclass_procedure,
    util::{window_frame_borders, windows_window_handle},
};
pub use {
    dark_mode::Theme,
    hit_test::{HitTestArea, Point, Size},
    options::*,
};

pub struct WindowCustomization {
    handle: HWND,
    subclass_id: usize,
    options: Box<WindowFrame>,
    is_set: Cell<bool>,
}
impl WindowCustomization {
    pub fn new<W: HasRawWindowHandle>(window: &W, options: WindowFrame) -> windows::Result<Self> {
        Self::with_id(window, options, 1)
    }
    pub fn with_id<W: HasRawWindowHandle>(
        window: &W,
        options: WindowFrame,
        subclass_id: usize,
    ) -> windows::Result<Self> {
        let handle = windows_window_handle(window);
        let customization = Self {
            handle,
            subclass_id,
            options: Box::new(options),
            is_set: Cell::new(false),
        };
        unsafe {
            customization.set()?;
        }
        Ok(customization)
    }
    pub unsafe fn set(&self) -> windows::Result<()> {
        let options_ptr = &*self.options as *const WindowFrame;
        SetWindowSubclass(
            self.handle,
            Some(subclass_procedure),
            self.subclass_id,
            options_ptr as usize,
        )
        .ok()?;
        self.is_set.set(true);
        self.update();
        Ok(())
    }
    pub fn edit(&mut self) -> WindowFrameMut {
        WindowFrameMut {
            customization: self,
        }
    }
    unsafe fn update(&self) {
        if let Some(theme) = &self.options.theme {
            dark_dwm_decorations(self.handle, matches!(theme, Theme::Dark));
        }

        let mut rect = RECT::default();
        GetWindowRect(self.handle, &mut rect);

        // Inform application of the frame change.
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let p_mar_inset = self.options.extend_frame.to_win32();

        SetWindowPos(
            self.handle,
            HWND(0),
            rect.left,
            rect.top,
            width,
            height,
            SetWindowPos_uFlags::SWP_FRAMECHANGED,
        );
        DwmExtendFrameIntoClientArea(self.handle, &p_mar_inset);
    }
    pub unsafe fn remove(&self) -> windows::Result<()> {
        RemoveWindowSubclass(self.handle, Some(subclass_procedure), self.subclass_id).ok()?;
        self.is_set.set(false);
        Ok(())
    }
}
impl Drop for WindowCustomization {
    fn drop(&mut self) {
        if self.is_set.get() {
            unsafe {
                let _ = self.remove();
            }
        }
    }
}

pub struct CustomizedWindow<W: HasRawWindowHandle> {
    window: W,
    customization: WindowCustomization,
}
impl<W: HasRawWindowHandle> CustomizedWindow<W> {
    pub fn wrap(window: W, options: WindowFrame) -> windows::Result<Self> {
        Self::wrap_with_id(window, options, 1)
    }
    pub fn wrap_with_id(
        window: W,
        options: WindowFrame,
        subclass_id: usize,
    ) -> windows::Result<Self> {
        let customization = WindowCustomization::with_id(&window, options, subclass_id)?;
        let subclassed = Self {
            window,
            customization,
        };
        Ok(subclassed)
    }
    pub fn unwrap(self) -> windows::Result<W> {
        Ok(self.window)
    }
    pub fn edit_custom_frame(&mut self) -> WindowFrameMut {
        self.customization.edit()
    }
}
impl<W: HasRawWindowHandle> Deref for CustomizedWindow<W> {
    type Target = W;

    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
impl<W: HasRawWindowHandle> DerefMut for CustomizedWindow<W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.window
    }
}

pub struct WindowFrameMut<'a> {
    customization: &'a mut WindowCustomization,
}
impl<'a> Deref for WindowFrameMut<'a> {
    type Target = WindowFrame;

    fn deref(&self) -> &Self::Target {
        &*self.customization.options
    }
}
impl<'a> DerefMut for WindowFrameMut<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.customization.options
    }
}
impl<'a> Drop for WindowFrameMut<'a> {
    fn drop(&mut self) {
        unsafe {
            self.customization.update();
        }
    }
}

pub trait CustomWindowFrame: HasRawWindowHandle + Sized {
    fn customize_frame(self, options: WindowFrame) -> windows::Result<CustomizedWindow<Self>>;
}
impl<W: HasRawWindowHandle> CustomWindowFrame for W {
    fn customize_frame(self, options: WindowFrame) -> windows::Result<CustomizedWindow<Self>> {
        CustomizedWindow::wrap(self, options)
    }
}

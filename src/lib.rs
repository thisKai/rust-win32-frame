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
        system_services::SWP_FRAMECHANGED,
        windows_and_messaging::{GetWindowRect, SetWindowPos, HWND},
    },
    dark_mode::dark_dwm_decorations,
    raw_window_handle::HasRawWindowHandle,
    std::ops::{Deref, DerefMut},
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
        self.update();
        Ok(())
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
            SWP_FRAMECHANGED as _,
        );
        DwmExtendFrameIntoClientArea(self.handle, &p_mar_inset);
    }
    pub unsafe fn remove(&self) -> windows::Result<()> {
        RemoveWindowSubclass(self.handle, Some(subclass_procedure), self.subclass_id).ok()
    }
}
impl Drop for WindowCustomization {
    fn drop(&mut self) {
        unsafe {
            let _ = self.remove();
        }
    }
}

pub struct CustomizedWindow<W: HasRawWindowHandle> {
    subclass_id: usize,
    window: W,
    options: Box<WindowFrame>,
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
        let h_wnd = windows_window_handle(&window);
        let subclassed = Self {
            subclass_id,
            window,
            options: Box::new(options),
        };
        let options_ptr = &*subclassed.options as *const WindowFrame;
        unsafe {
            SetWindowSubclass(
                h_wnd,
                Some(subclass_procedure),
                subclass_id,
                options_ptr as usize,
            )
            .ok()?;
            subclassed.update();
        }
        Ok(subclassed)
    }
    pub fn unwrap(self) -> windows::Result<W> {
        let h_wnd = windows_window_handle(&self.window);
        unsafe {
            RemoveWindowSubclass(h_wnd, Some(subclass_procedure), self.subclass_id).ok()?;
        }
        Ok(self.window)
    }
    pub fn edit_custom_frame(&mut self) -> WindowFrameMut<W> {
        WindowFrameMut { window: self }
    }
    unsafe fn update(&self) {
        let h_wnd = windows_window_handle(&self.window);

        if let Some(theme) = &self.options.theme {
            dark_dwm_decorations(h_wnd, matches!(theme, Theme::Dark));
        }

        let mut rect = RECT::default();
        GetWindowRect(h_wnd, &mut rect);

        // Inform application of the frame change.
        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let p_mar_inset = self.options.extend_frame.to_win32();

        SetWindowPos(
            h_wnd,
            HWND(0),
            rect.left,
            rect.top,
            width,
            height,
            SWP_FRAMECHANGED as _,
        );
        DwmExtendFrameIntoClientArea(h_wnd, &p_mar_inset);
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

pub struct WindowFrameMut<'a, W: HasRawWindowHandle> {
    window: &'a mut CustomizedWindow<W>,
}
impl<'a, W: HasRawWindowHandle> Deref for WindowFrameMut<'a, W> {
    type Target = WindowFrame;

    fn deref(&self) -> &Self::Target {
        &*self.window.options
    }
}
impl<'a, W: HasRawWindowHandle> DerefMut for WindowFrameMut<'a, W> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.window.options
    }
}
impl<'a, W: HasRawWindowHandle> Drop for WindowFrameMut<'a, W> {
    fn drop(&mut self) {
        unsafe {
            self.window.update();
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

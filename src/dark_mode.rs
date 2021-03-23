use {
    crate::bindings::windows::win32::{
        system_services::{BOOL, NTSTATUS},
        windows_and_messaging::HWND,
    },
    once_cell::sync::Lazy,
    std::{ffi::c_void, mem},
    windows_dll::dll,
};

pub enum Theme {
    Light,
    Dark,
}

static WIN10_BUILD: Lazy<Option<u32>> = Lazy::new(|| {
    #[inline]
    const fn NT_SUCCESS(Status: NTSTATUS) -> bool {
        Status.0 >= 0
    }

    #[dll(ntdll)]
    extern "system" {
        #[allow(non_snake_case)]
        fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOW) -> NTSTATUS;
    }

    #[allow(non_snake_case)]
    #[repr(C)]
    struct OSVERSIONINFOW {
        dwOSVersionInfoSize: u32,
        dwMajorVersion: u32,
        dwMinorVersion: u32,
        dwBuildNumber: u32,
        dwPlatformId: u32,
        szCSDVersion: [u16; 128],
    }
    if !RtlGetVersion::exists() {
        return None;
    }
    unsafe {
        let mut version_info = OSVERSIONINFOW {
            dwOSVersionInfoSize: 0,
            dwMajorVersion: 0,
            dwMinorVersion: 0,
            dwBuildNumber: 0,
            dwPlatformId: 0,
            szCSDVersion: [0; 128],
        };
        let status = RtlGetVersion(&mut version_info);

        if NT_SUCCESS(status)
            && version_info.dwMajorVersion == 10
            && version_info.dwMinorVersion == 0
        {
            Some(version_info.dwBuildNumber)
        } else {
            None
        }
    }
});

static DARK_MODE_SUPPORTED: Lazy<bool> = Lazy::new(|| match *WIN10_BUILD {
    Some(build) => build >= 17763,
    None => false,
});

pub fn dark_dwm_decorations(hwnd: HWND, enable_dark_mode: bool) -> bool {
    #[allow(non_snake_case)]
    type WINDOWCOMPOSITIONATTRIB = u32;
    const WCA_USEDARKMODECOLORS: WINDOWCOMPOSITIONATTRIB = 26;

    #[allow(non_snake_case)]
    #[repr(C)]
    struct WINDOWCOMPOSITIONATTRIBDATA {
        Attrib: WINDOWCOMPOSITIONATTRIB,
        pvData: *mut c_void,
        cbData: usize,
    }

    #[dll(user32)]
    extern "system" {
        #[allow(non_snake_case)]
        fn SetWindowCompositionAttribute(
            h_wnd: HWND,
            data: *mut WINDOWCOMPOSITIONATTRIBDATA,
        ) -> BOOL;
    }

    if *DARK_MODE_SUPPORTED && SetWindowCompositionAttribute::exists() {
        unsafe {
            let mut is_dark_mode_bigbool = BOOL::from(enable_dark_mode);
            let mut data = WINDOWCOMPOSITIONATTRIBDATA {
                Attrib: WCA_USEDARKMODECOLORS,
                pvData: &mut is_dark_mode_bigbool as *mut _ as _,
                cbData: mem::size_of::<BOOL>(),
            };

            SetWindowCompositionAttribute(hwnd, &mut data).as_bool()
        }
    } else {
        false
    }
}

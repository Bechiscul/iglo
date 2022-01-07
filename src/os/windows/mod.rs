use windows::Win32::{
    Foundation::*, Graphics::Gdi::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*,
};

use super::WindowApi;

impl IntoPWSTR for &str {
    fn into_pwstr(&self) -> (PWSTR, Vec<u16>) {
        let mut v: Vec<u16> = self.encode_utf16().chain([0u16]).collect();
        (PWSTR(v.as_mut_ptr()), v)
    }
}

impl IntoPWSTR for String {
    fn into_pwstr(&self) -> (PWSTR, Vec<u16>) {
        self.as_str().into_pwstr()
    }
}

pub struct Window {
    hinstance: HINSTANCE,
    hwnd: HWND,
}

impl Window {
    pub fn new() -> Result<Self, ()> {
        let hinstance = unsafe { GetModuleHandleW(PWSTR::default()) };
        let class_name = Self::register_class(hinstance)?;

        let hwnd = unsafe {
            CreateWindowExW(
                0,
                class_name.into_pwstr().0,
                PWSTR::default(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                640,
                480,
                HWND::default(),
                HMENU::default(),
                hinstance,
                std::ptr::null(),
            )
        };

        if hwnd == 0 {
            return Err(());
        }

        Ok(Self { hinstance, hwnd })
    }

    fn is_class_registered(instance: HINSTANCE, class_name: &str) -> bool {
        let mut wcx = WNDCLASSEXW::default();
        unsafe {
            GetClassInfoExW(
                instance,
                class_name.into_pwstr().0,
                std::ptr::addr_of_mut!(wcx),
            )
        }
        .as_bool()
    }

    fn register_class(instance: HINSTANCE) -> Result<&'static str, ()> {
        const CLASS_NAME: &str = "iglo_window";

        if !Self::is_class_registered(instance, CLASS_NAME) {
            let wcx = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_OWNDC,
                lpfnWndProc: Some(Self::wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: instance,
                hIcon: HICON::default(),
                hCursor: HCURSOR::default(),
                hbrBackground: HBRUSH::default(),
                lpszMenuName: PWSTR::default(),
                lpszClassName: CLASS_NAME.into_pwstr().0,
                hIconSm: HICON::default(),
            };

            unsafe {
                if RegisterClassExW(&wcx) == 0 {
                    println!("Failed to register class!");
                    println!("{:?}", GetLastError());
                    return Err(());
                }
            }
        }

        Ok(CLASS_NAME)
    }

    extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }

    pub fn hinstance(&self) -> &HINSTANCE {
        &self.hinstance
    }

    pub fn hwnd(&self) -> &HWND {
        &self.hwnd
    }
}

impl WindowApi for Window {
    fn show(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_SHOW);
        }
    }

    fn poll_events(&self) {
        unsafe {
            let mut msg: MSG = MSG::default();
            while PeekMessageW(std::ptr::addr_of_mut!(msg), self.hwnd, 0, 0, PM_REMOVE).0 != 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}

trait IntoPWSTR {
    fn into_pwstr(&self) -> (PWSTR, Vec<u16>);
}

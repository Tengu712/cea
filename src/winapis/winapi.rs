use super::*;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::*,
        Graphics::Gdi::ValidateRect,
        System::LibraryLoader::GetModuleHandleW,
        UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
    },
};

/// Show a message box.
pub fn show_messagebox<'a, T>(message: T, title: T)
where
    T: windows::core::IntoParam<'a, windows::core::PCWSTR>,
{
    unsafe { MessageBoxW(None, message, title, MB_OK | MB_ICONERROR) };
}
/// Show a yes-no message box. The answer is yes, it returns true.
pub fn ask_yesno<'a, T>(message: T, title: T) -> bool
where
    T: windows::core::IntoParam<'a, windows::core::PCWSTR>,
{
    unsafe { MessageBoxW(None, message, title, MB_YESNO) == IDNO }
}
/// Get key state and return next value based on previous.
pub fn get_next_keystate(vkey: i32, state: i16) -> i16 {
    if (unsafe { GetAsyncKeyState(vkey) } as u16 & 0x8000) > 0 {
        std::cmp::max(state + 1, 1)
    } else if state > 0 {
        -1
    } else {
        0
    }
}
/// Private. Window procedure.
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message as u32 {
            WM_PAINT => {
                ValidateRect(window, std::ptr::null());
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}

/// Struct to reference basic objects
pub struct WindowsApplication {
    hwnd: HWND,
    path: String,
}
impl WindowsApplication {
    /// Create WindowsApplication struct that is only way to use api with window handle.
    pub fn new(
        path: &str,
        title: &str,
        width: i32,
        height: i32,
        windowed: bool,
    ) -> Result<Self, MyErr> {
        let (window_style, window_show) = if windowed {
            (WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX, SW_SHOW)
        } else {
            (WS_POPUP, SW_SHOWMAXIMIZED)
        };
        // Get instance handle
        let instance = unsafe { GetModuleHandleW(None) };
        if instance.0 == 0 {
            return Err(MyErr::WinApp(ErrKnd::Get, String::from("instance handle")));
        }
        // Register window class
        let wcex = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance,
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW) },
            lpszClassName: PCWSTR(
                "RustWindowClass\0"
                    .encode_utf16()
                    .collect::<Vec<u16>>()
                    .as_ptr(),
            ),
            ..Default::default()
        };
        if unsafe { RegisterClassExW(&wcex) == 0 } {
            return Err(MyErr::WinApp(
                ErrKnd::Common,
                String::from("Registration window class failed"),
            ));
        }
        // Adjust window size
        let mut window_rect = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        unsafe { AdjustWindowRectEx(&mut window_rect, window_style, false, Default::default()) };
        // Create window and get window handle
        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                "RustWindowClass",
                title,
                window_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                window_rect.right - window_rect.left,
                window_rect.bottom - window_rect.top,
                None,
                None,
                instance,
                std::ptr::null(),
            )
        };
        if hwnd.is_invalid() {
            return Err(MyErr::WinApp(ErrKnd::Creation, String::from("window")));
        }
        unsafe { ShowWindow(hwnd, window_show) };
        // Finish
        Ok(Self {
            path: path.to_string(),
            hwnd,
        })
    }
    /// Process all window events.
    /// If return value is true, window has closed. Otherwise, it is deadtime.
    pub fn do_event(&self) -> bool {
        let mut message = MSG::default();
        loop {
            if unsafe { PeekMessageW(&mut message, None, 0, 0, PM_REMOVE).into() } {
                if message.message == WM_QUIT {
                    return true;
                }
                unsafe {
                    TranslateMessage(&message);
                    DispatchMessageW(&message);
                }
                continue;
            }
            break;
        }
        false
    }
    /// Getter for window handle.
    pub fn get_window_handle(&self) -> &HWND {
        &self.hwnd
    }
    /// Getter for current path.
    pub fn get_current_path(&self) -> &String {
        &self.path
    }
}

#![cfg(windows)]

fn to_wstring(input_str: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(input_str)
        .encode_wide()
        .chain(std::iter::once(0)) // Adds a null terminator
        .collect()
}

pub use winapi::um::winuser::*;
pub use winapi::um::libloaderapi::GetModuleHandleW;
pub use winapi::shared::windef::*;
pub use winapi::shared::minwindef::*;

pub unsafe extern "system" fn handle_window_events(
    window_handle: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {
    DefWindowProcW(window_handle, message, wparam, lparam)
}

fn create_window() {
    let window_class_name = to_wstring("Window test!");
    let window_name = to_wstring("Window test!");
    
    unsafe {
        let instance_handle = GetModuleHandleW(std::ptr::null_mut());

        let window_class = WNDCLASSW{
            style: CS_DBLCLKS,
            lpfnWndProc: Some(handle_window_events),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: instance_handle,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: std::ptr::null_mut(),
            lpszMenuName: std::ptr::null_mut(),
            lpszClassName: window_name.as_ptr(),
        };
        RegisterClassW(&window_class);

        SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);

        let style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;

        CreateWindowExW(
            0,
            window_class_name.as_ptr(),
            window_name.as_ptr(),
            style,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance_handle,
            std::ptr::null_mut(),
        );

        // Initialize MSG struct with uninitialized memory.
        let mut message: MSG = std::mem::MaybeUninit::uninit().assume_init();
        while winapi::um::winuser::GetMessageW(&mut message, std::ptr::null_mut(), 0, 0) != 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn main() {
    create_window();
}
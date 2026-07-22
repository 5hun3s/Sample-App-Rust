#[derive(Debug, Clone)]
pub struct ActiveWindowInfo {
    pub window_title: String,
    pub process_id: u32,
    pub process_name: Option<String>,
    pub executable_path: Option<String>,
    pub window_class: Option<String>,

    pub window_x: Option<i32>,
    pub window_y: Option<i32>,
    pub window_width: Option<i32>,
    pub window_height: Option<i32>,

    pub is_maximized: bool,
    pub is_minimized: bool,
    pub idle_seconds: u64,
}

#[cfg(target_os = "windows")]
pub fn get_active_window_info() -> Result<Option<ActiveWindowInfo>, String> {
    use std::path::Path;

    use windows_sys::Win32::{
        Foundation::{CloseHandle, RECT},
        System::{
            SystemInformation::GetTickCount,
            Threading::{
                OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32,
                PROCESS_QUERY_LIMITED_INFORMATION,
            },
        },
        UI::WindowsAndMessaging::{
            GetClassNameW, GetForegroundWindow, GetLastInputInfo, GetWindowRect,
            GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsZoomed,
            LASTINPUTINFO,
        },
    };

    unsafe {
        let window_handle = GetForegroundWindow();

        if window_handle.is_null() {
            return Ok(None);
        }

        // ウィンドウタイトル取得
        let title_length = GetWindowTextLengthW(window_handle);

        let window_title = if title_length > 0 {
            let mut buffer = vec![0_u16; title_length as usize + 1];

            let copied_length =
                GetWindowTextW(window_handle, buffer.as_mut_ptr(), buffer.len() as i32);

            if copied_length > 0 {
                String::from_utf16_lossy(&buffer[..copied_length as usize])
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // プロセスID取得
        let mut process_id = 0_u32;

        GetWindowThreadProcessId(window_handle, &mut process_id);

        // ウィンドウクラス名取得
        let mut class_buffer = vec![0_u16; 256];

        let class_length = GetClassNameW(
            window_handle,
            class_buffer.as_mut_ptr(),
            class_buffer.len() as i32,
        );

        let window_class = if class_length > 0 {
            Some(String::from_utf16_lossy(
                &class_buffer[..class_length as usize],
            ))
        } else {
            None
        };

        // ウィンドウ位置・サイズ取得
        let mut rect = RECT::default();

        let rect_result = GetWindowRect(window_handle, &mut rect);

        let (window_x, window_y, window_width, window_height) = if rect_result != 0 {
            (
                Some(rect.left),
                Some(rect.top),
                Some(rect.right - rect.left),
                Some(rect.bottom - rect.top),
            )
        } else {
            (None, None, None, None)
        };

        let is_maximized = IsZoomed(window_handle) != 0;
        let is_minimized = IsIconic(window_handle) != 0;

        // 実行ファイルパス取得
        let executable_path = get_process_executable_path(process_id);

        // フルパスからファイル名だけ抽出
        let process_name = executable_path.as_ref().and_then(|path| {
            Path::new(path)
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
        });

        let idle_seconds = get_idle_seconds();

        Ok(Some(ActiveWindowInfo {
            window_title,
            process_id,
            process_name,
            executable_path,
            window_class,

            window_x,
            window_y,
            window_width,
            window_height,

            is_maximized,
            is_minimized,
            idle_seconds,
        }))
    }

    unsafe fn get_process_executable_path(process_id: u32) -> Option<String> {
        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, process_id);

        if process_handle.is_null() {
            return None;
        }

        let mut buffer = vec![0_u16; 32_768];
        let mut size = buffer.len() as u32;

        let result = QueryFullProcessImageNameW(
            process_handle,
            PROCESS_NAME_WIN32,
            buffer.as_mut_ptr(),
            &mut size,
        );

        let _ = CloseHandle(process_handle);

        if result == 0 {
            return None;
        }

        Some(String::from_utf16_lossy(&buffer[..size as usize]))
    }

    unsafe fn get_idle_seconds() -> u64 {
        let mut input_info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };

        let result = GetLastInputInfo(&mut input_info);

        if result == 0 {
            return 0;
        }

        let current_tick = GetTickCount();

        let idle_milliseconds = current_tick.wrapping_sub(input_info.dwTime);

        u64::from(idle_milliseconds) / 1_000
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_active_window_info() -> Result<Option<ActiveWindowInfo>, String> {
    // UbuntuのDev ContainerではWindows情報を取得できない
    Ok(None)
}

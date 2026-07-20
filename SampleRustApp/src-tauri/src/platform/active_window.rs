#[derive(Debug, Clone)]
pub struct ActiveWindowInfo {
    pub title: String,
    pub process_id: u32,
}

#[cfg(target_os = "windows")]
pub fn get_active_window() -> Result<Option<ActiveWindowInfo>, String> {
    use windows::Win32::UI::WindowsAndMessaging::{
        GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    };

    unsafe {
        // 現在最前面にあるウィンドウのハンドルを取得する
        let window_handle = GetForegroundWindow();

        // アクティブウィンドウが存在しないこともある
        if window_handle.0.is_null() {
            return Ok(None);
        }

        // ウィンドウを所有するプロセスのIDを取得する
        let mut process_id = 0_u32;

        GetWindowThreadProcessId(window_handle, Some(&mut process_id));

        // タイトルの文字数を取得する
        let title_length = GetWindowTextLengthW(window_handle);

        if title_length <= 0 {
            return Ok(Some(ActiveWindowInfo {
                title: String::new(),
                process_id,
            }));
        }

        /*
         * Windowsの文字列はUTF-16。
         * 終端文字分として +1 したバッファを用意する。
         */
        let mut buffer = vec![0_u16; title_length as usize + 1];

        let copied_length = GetWindowTextW(window_handle, &mut buffer);

        if copied_length == 0 {
            return Ok(Some(ActiveWindowInfo {
                title: String::new(),
                process_id,
            }));
        }

        let title = String::from_utf16_lossy(&buffer[..copied_length as usize]);

        Ok(Some(ActiveWindowInfo { title, process_id }))
    }
}

/*
 * UbuntuのDev Containerでもcargo checkできるようにする。
 * LinuxではWindowsのアクティブウィンドウ取得を行わない。
 */
#[cfg(not(target_os = "windows"))]
pub fn get_active_window() -> Result<Option<ActiveWindowInfo>, String> {
    Ok(None)
}

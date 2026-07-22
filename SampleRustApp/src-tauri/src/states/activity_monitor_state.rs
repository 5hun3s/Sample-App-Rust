use std::sync::Mutex;

use tokio::sync::watch;

#[derive(Debug, Clone)]
pub enum MonitorCommand {
    ChangeInterval(u64),
    Stop,
}

pub struct ActivityMonitorState {
    sender: Mutex<Option<watch::Sender<MonitorCommand>>>,
}

impl ActivityMonitorState {
    pub fn new() -> Self {
        Self {
            sender: Mutex::new(None),
        }
    }

    pub fn start(&self, sender: watch::Sender<MonitorCommand>) -> Result<(), String> {
        let mut guard = self
            .sender
            .lock()
            .map_err(|_| "監視状態の取得に失敗しました。".to_string())?;

        if guard.is_some() {
            return Err("すでに記録を開始しています。".to_string());
        }

        *guard = Some(sender);

        Ok(())
    }

    pub fn send(&self, command: MonitorCommand) -> Result<(), String> {
        let guard = self
            .sender
            .lock()
            .map_err(|_| "監視状態の取得に失敗しました。".to_string())?;

        let sender = guard
            .as_ref()
            .ok_or_else(|| "記録は開始されていません。".to_string())?;

        sender
            .send(command)
            .map_err(|_| "監視処理への通知に失敗しました。".to_string())
    }

    pub fn clear(&self) -> Result<(), String> {
        let mut guard = self
            .sender
            .lock()
            .map_err(|_| "監視状態の取得に失敗しました。".to_string())?;

        *guard = None;

        Ok(())
    }

    pub fn is_running(&self) -> Result<bool, String> {
        let guard = self
            .sender
            .lock()
            .map_err(|_| "監視状態の取得に失敗しました。".to_string())?;

        Ok(guard.is_some())
    }
}

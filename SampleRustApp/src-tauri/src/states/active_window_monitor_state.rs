use std::sync::Mutex;

use tokio::sync::watch;

pub struct ActiveWindowMonitorState {
    control_sender: Mutex<Option<watch::Sender<MonitorCommand>>>,
}

#[derive(Debug, Clone)]
pub enum MonitorCommand {
    UpdateInterval(u64),
    Stop,
}

impl ActiveWindowMonitorState {
    pub fn new() -> Self {
        Self {
            control_sender: Mutex::new(None),
        }
    }

    pub fn replace_sender(&self, sender: watch::Sender<MonitorCommand>) -> Result<(), String> {
        let mut guard = self
            .control_sender
            .lock()
            .map_err(|_| "監視状態のロックに失敗しました。")?;

        // すでに監視中なら、古いタスクへ停止を通知する
        if let Some(old_sender) = guard.take() {
            let _ = old_sender.send(MonitorCommand::Stop);
        }

        *guard = Some(sender);

        Ok(())
    }

    pub fn send_command(&self, command: MonitorCommand) -> Result<(), String> {
        let guard = self
            .control_sender
            .lock()
            .map_err(|_| "監視状態のロックに失敗しました。")?;

        let sender = guard
            .as_ref()
            .ok_or("アクティブウィンドウの記録は開始されていません。")?;

        sender
            .send(command)
            .map_err(|_| "監視タスクへの通知に失敗しました。".to_string())
    }
}

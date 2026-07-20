import { invoke } from "@tauri-apps/api/core";

export type ActiveWindowLog = {
  id: number;
  windowTitle: string;
  processId: number;
  recordedAt: string;
};

export async function startActiveWindowMonitor(
  intervalSeconds: number,
): Promise<void> {
  await invoke("start_active_window_monitor", {
    intervalSeconds,
  });
}

export async function updateActiveWindowInterval(
  intervalSeconds: number,
): Promise<void> {
  await invoke("update_active_window_interval", {
    intervalSeconds,
  });
}

export async function stopActiveWindowMonitor(): Promise<void> {
  await invoke("stop_active_window_monitor");
}

export async function getActiveWindowLogs(): Promise<ActiveWindowLog[]> {
  return await invoke<ActiveWindowLog[]>("get_active_window_logs");
}

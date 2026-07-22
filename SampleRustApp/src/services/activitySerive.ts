import { invoke } from "@tauri-apps/api/core";

export type ActivityLog = {
  id: number;

  windowTitle: string;
  processId: number;
  processName: string | null;
  executablePath: string | null;
  windowClass: string | null;

  windowX: number | null;
  windowY: number | null;
  windowWidth: number | null;
  windowHeight: number | null;

  isMaximized: boolean;
  isMinimized: boolean;

  idleSeconds: number;
  recordedAt: string;
};

export async function startActivityMonitor(
  intervalSeconds: number,
): Promise<void> {
  await invoke("start_activity_monitor", {
    intervalSeconds,
  });
}

export async function changeActivityInterval(
  intervalSeconds: number,
): Promise<void> {
  await invoke("change_activity_interval", {
    intervalSeconds,
  });
}

export async function stopActivityMonitor(): Promise<void> {
  await invoke("stop_activity_monitor");
}

export async function isActivityMonitorRunning(): Promise<boolean> {
  return await invoke<boolean>("is_activity_monitor_running");
}

export async function getActivityLogs(limit = 100): Promise<ActivityLog[]> {
  return await invoke<ActivityLog[]>("get_activity_logs", { limit });
}

export async function deleteAllActivityLogs(): Promise<number> {
  return await invoke<number>("delete_all_activity_logs");
}

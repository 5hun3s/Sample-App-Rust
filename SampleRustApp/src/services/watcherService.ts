import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type FileCreatedEvent = {
  filePath: string;
  fileName: string;
  detectedAt: string;
};

export type BatExecutionResult = {
  filePath: string;
  batPath: string;

  success: boolean;
  exitCode: number | null;

  stdout: string;
  stderr: string;

  startedAt: string;
  finishedAt: string;
  durationMilliseconds: number;
};

export async function startFolderWatcher(
  watchDirectory: string,
  batPath: string,
): Promise<void> {
  await invoke("start_folder_watcher", {
    watchDirectory,
    batPath,
  });
}

export async function stopFolderWatcher(): Promise<boolean> {
  return await invoke<boolean>("stop_folder_watcher");
}

export async function getWatcherStatus(): Promise<boolean> {
  return await invoke<boolean>("get_watcher_status");
}

export async function listenFileCreated(
  handler: (event: FileCreatedEvent) => void,
): Promise<UnlistenFn> {
  return await listen<FileCreatedEvent>("file-created", (event) => {
    handler(event.payload);
  });
}

export async function listenBatStarted(
  handler: (filePath: string) => void,
): Promise<UnlistenFn> {
  return await listen<string>("bat-started", (event) => {
    handler(event.payload);
  });
}

export async function listenBatFinished(
  handler: (result: BatExecutionResult) => void,
): Promise<UnlistenFn> {
  return await listen<BatExecutionResult>("bat-finished", (event) => {
    handler(event.payload);
  });
}

export async function listenWatcherError(
  handler: (message: string) => void,
): Promise<UnlistenFn> {
  return await listen<string>("watcher-error", (event) => {
    handler(event.payload);
  });
}

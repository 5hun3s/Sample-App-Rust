import { useEffect, useState } from "react";

import {
  getActivityLogs,
  startActivityMonitor,
  stopActivityMonitor,
  type ActivityLog,
} from "../services/activitySerive";

export function ActivityPage() {
  const [intervalSeconds, setIntervalSeconds] = useState(10);

  const [isRunning, setIsRunning] = useState(false);

  const [logs, setLogs] = useState<ActivityLog[]>([]);

  const [errorMessage, setErrorMessage] = useState("");

  async function loadLogs(): Promise<void> {
    try {
      setErrorMessage("");
      setLogs(await getActivityLogs(200));
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  async function handleStart(): Promise<void> {
    try {
      setErrorMessage("");

      await startActivityMonitor(intervalSeconds);
      setIsRunning(true);
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  async function handleStop(): Promise<void> {
    try {
      setErrorMessage("");

      await stopActivityMonitor();
      setIsRunning(false);
      await loadLogs();
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  useEffect(() => {
    void loadLogs();
  }, []);

  return (
    <main>
      <h1>ウィンドウ操作履歴</h1>

      <label>
        記録間隔（秒）
        <input
          type="number"
          min={1}
          max={3600}
          value={intervalSeconds}
          disabled={isRunning}
          onChange={(event) => {
            setIntervalSeconds(Number(event.target.value));
          }}
        />
      </label>

      <div>
        <button
          type="button"
          disabled={isRunning}
          onClick={() => void handleStart()}
        >
          記録開始
        </button>

        <button
          type="button"
          disabled={!isRunning}
          onClick={() => void handleStop()}
        >
          記録停止
        </button>

        <button type="button" onClick={() => void loadLogs()}>
          履歴更新
        </button>
      </div>

      {errorMessage && <p role="alert">{errorMessage}</p>}

      <table>
        <thead>
          <tr>
            <th>記録日時</th>
            <th>アプリ</th>
            <th>ウィンドウタイトル</th>
            <th>プロセスID</th>
            <th>アイドル秒数</th>
          </tr>
        </thead>

        <tbody>
          {logs.map((log) => (
            <tr key={log.id}>
              <td>{log.recordedAt}</td>
              <td>{log.processName ?? "不明"}</td>
              <td>{log.windowTitle}</td>
              <td>{log.processId}</td>
              <td>{log.idleSeconds}秒</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}

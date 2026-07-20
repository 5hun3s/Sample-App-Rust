import { useEffect, useState } from "react";

import {
  getActiveWindowLogs,
  startActiveWindowMonitor,
  stopActiveWindowMonitor,
  updateActiveWindowInterval,
  type ActiveWindowLog,
} from "../services/activeWindowService";

export function ActiveWindowMonitor() {
  const [intervalSeconds, setIntervalSeconds] = useState(10);
  const [isMonitoring, setIsMonitoring] = useState(false);
  const [logs, setLogs] = useState<ActiveWindowLog[]>([]);
  const [errorMessage, setErrorMessage] = useState("");

  async function loadLogs(): Promise<void> {
    try {
      const result = await getActiveWindowLogs();
      setLogs(result);
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  async function handleStart(): Promise<void> {
    try {
      setErrorMessage("");

      await startActiveWindowMonitor(intervalSeconds);
      setIsMonitoring(true);
    } catch (error) {
      setErrorMessage(String(error));
    }
  }

  async function handleIntervalChange(newValue: number): Promise<void> {
    setIntervalSeconds(newValue);

    if (isMonitoring) {
      try {
        await updateActiveWindowInterval(newValue);
      } catch (error) {
        setErrorMessage(String(error));
      }
    }
  }

  async function handleStop(): Promise<void> {
    try {
      setErrorMessage("");

      await stopActiveWindowMonitor();
      setIsMonitoring(false);
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
      <h1>アクティブウィンドウ記録</h1>

      <label>
        記録間隔（秒）
        <input
          type="number"
          min={1}
          max={3600}
          value={intervalSeconds}
          onChange={(event) => {
            void handleIntervalChange(Number(event.target.value));
          }}
        />
      </label>

      <div>
        <button
          type="button"
          disabled={isMonitoring}
          onClick={() => {
            void handleStart();
          }}
        >
          記録開始
        </button>

        <button
          type="button"
          disabled={!isMonitoring}
          onClick={() => {
            void handleStop();
          }}
        >
          記録停止
        </button>

        <button
          type="button"
          onClick={() => {
            void loadLogs();
          }}
        >
          履歴更新
        </button>
      </div>

      {errorMessage && <p>{errorMessage}</p>}

      <table>
        <thead>
          <tr>
            <th>日時</th>
            <th>タイトル</th>
            <th>プロセスID</th>
          </tr>
        </thead>

        <tbody>
          {logs.map((log) => (
            <tr key={log.id}>
              <td>{log.recordedAt}</td>
              <td>{log.windowTitle}</td>
              <td>{log.processId}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}
export default ActiveWindowMonitor;

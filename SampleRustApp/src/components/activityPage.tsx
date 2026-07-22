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
  function formatDuration(totalSeconds: number): string {
  const hours = Math.floor(totalSeconds / 3600);

  const minutes = Math.floor(
    (totalSeconds % 3600) / 60,
  );

  const seconds = totalSeconds % 60;

  if (hours > 0) {
    return `${hours}時間${minutes}分${seconds}秒`;
  }

  if (minutes > 0) {
    return `${minutes}分${seconds}秒`;
  }

  return `${seconds}秒`;
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
      <th>開始・終了日時</th>
      <th>アプリ</th>
      <th>ウィンドウタイトル</th>
      <th>経過時間</th>
      <th>アイドル秒数</th>
          </tr>
        </thead>

        <tbody>
          {logs.map((log) => (
            <tr key={log.id}>
              <td>
                <div>{log.startedAt}</div>
                <div>～</div>
                <div>{log.endedAt}</div>
              </td>

              <td>{log.processName ?? "不明"}</td>

              <td>{log.windowTitle}</td>

              <td>
                {formatDuration(
                  log.durationSeconds,
                )}
              </td>

              <td>{log.idleSeconds}秒</td>
            </tr>
          ))}
        </tbody>
      </table>
    </main>
  );
}
export default ActivityPage;

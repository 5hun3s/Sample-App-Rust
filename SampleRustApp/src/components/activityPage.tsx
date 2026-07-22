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

    const minutes = Math.floor((totalSeconds % 3600) / 60);

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
    <main className="activity-page">
      <header className="app-header">
        <div className="app-mark" aria-hidden="true">JS</div>
        <h1>ウィンドウ操作履歴</h1>
      </header>

      <section className="control-panel">
        <label className="interval-field">
          <span>記録間隔（秒）</span>
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

        <div className="actions">
          <button
            className="button button-start"
            type="button"
            disabled={isRunning}
            onClick={() => void handleStart()}
          >
            記録開始
          </button>

          <button
            className="button button-stop"
            type="button"
            disabled={!isRunning}
            onClick={() => void handleStop()}
          >
            記録停止
          </button>

          <button
            className="button button-refresh"
            type="button"
            onClick={() => void loadLogs()}
          >
            履歴更新
          </button>
        </div>
      </section>

      {errorMessage && <p className="error-message" role="alert">{errorMessage}</p>}

      <div className="log-panel">
        <div className="window-bar" aria-hidden="true">
          <span />
          <span />
          <span />
        </div>
        <div className="table-scroll">
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
                  <td className="date-cell">
                    <div>{log.startedAt}</div>
                    <div>～</div>
                    <div>{log.endedAt}</div>
                  </td>

                  <td className="process-cell">{log.processName ?? "不明"}</td>

                  <td className="title-cell">{log.windowTitle}</td>

                  <td className="duration-cell">
                    {formatDuration(log.durationSeconds)}
                  </td>

                  <td className="idle-cell">{log.idleSeconds}秒</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </main>
  );
}
export default ActivityPage;

import { useEffect, useState } from "react";

import {
  getWatcherStatus,
  listenBatFinished,
  listenBatStarted,
  listenFileCreated,
  listenWatcherError,
  startFolderWatcher,
  stopFolderWatcher,
  type BatExecutionResult,
} from "./services/watcherService";

type ApplicationLog = {
  id: string;
  dateTime: string;
  type: "detected" | "running" | "success" | "error";
  message: string;
  detail?: string;
};

function App() {
  const [watchDirectory, setWatchDirectory] = useState("");

  const [batPath, setBatPath] = useState("");

  const [isWatching, setIsWatching] = useState(false);

  const [logs, setLogs] = useState<ApplicationLog[]>([]);

  function addLog(log: Omit<ApplicationLog, "id">): void {
    setLogs((currentLogs) => [
      {
        ...log,
        id: crypto.randomUUID(),
      },
      ...currentLogs,
    ]);
  }

  async function handleStart(): Promise<void> {
    try {
      await startFolderWatcher(watchDirectory, batPath);

      setIsWatching(true);

      addLog({
        dateTime: new Date().toISOString(),
        type: "running",
        message: "フォルダ監視を開始しました。",
        detail: watchDirectory,
      });
    } catch (error) {
      addLog({
        dateTime: new Date().toISOString(),
        type: "error",
        message: "監視開始に失敗しました。",
        detail: String(error),
      });
    }
  }

  async function handleStop(): Promise<void> {
    try {
      await stopFolderWatcher();

      setIsWatching(false);

      addLog({
        dateTime: new Date().toISOString(),
        type: "running",
        message: "フォルダ監視を停止しました。",
      });
    } catch (error) {
      addLog({
        dateTime: new Date().toISOString(),
        type: "error",
        message: "監視停止に失敗しました。",
        detail: String(error),
      });
    }
  }

  useEffect(() => {
    const unlistenFunctions: Array<() => void> = [];

    async function initialize(): Promise<void> {
      try {
        const running = await getWatcherStatus();

        setIsWatching(running);
      } catch (error) {
        console.error(error);
      }

      const unlistenCreated = await listenFileCreated((event) => {
        addLog({
          dateTime: event.detectedAt,
          type: "detected",
          message: `ファイルを検知しました: ${event.fileName}`,
          detail: event.filePath,
        });
      });

      unlistenFunctions.push(unlistenCreated);

      const unlistenStarted = await listenBatStarted((filePath) => {
        addLog({
          dateTime: new Date().toISOString(),
          type: "running",
          message: "BATを実行しています。",
          detail: filePath,
        });
      });

      unlistenFunctions.push(unlistenStarted);

      const unlistenFinished = await listenBatFinished(
        (result: BatExecutionResult) => {
          addLog({
            dateTime: result.finishedAt,
            type: result.success ? "success" : "error",

            message: result.success
              ? `BAT実行成功: ExitCode=${result.exitCode}`
              : `BAT実行失敗: ExitCode=${result.exitCode}`,

            detail: [
              `対象: ${result.filePath}`,
              `BAT: ${result.batPath}`,
              `処理時間: ${result.durationMilliseconds}ms`,
              "",
              "標準出力:",
              result.stdout || "(なし)",
              "",
              "標準エラー:",
              result.stderr || "(なし)",
            ].join("\n"),
          });
        },
      );

      unlistenFunctions.push(unlistenFinished);

      const unlistenError = await listenWatcherError((message) => {
        addLog({
          dateTime: new Date().toISOString(),
          type: "error",
          message: "エラーが発生しました。",
          detail: message,
        });
      });

      unlistenFunctions.push(unlistenError);
    }

    void initialize();

    return () => {
      for (const unlisten of unlistenFunctions) {
        unlisten();
      }
    };
  }, []);

  return (
    <main className="container">
      <h1>フォルダ監視・BAT実行</h1>

      <section>
        <label htmlFor="watch-directory">監視フォルダ</label>

        <input
          id="watch-directory"
          type="text"
          placeholder="C:\Import"
          value={watchDirectory}
          disabled={isWatching}
          onChange={(event) => {
            setWatchDirectory(event.target.value);
          }}
        />
      </section>

      <section>
        <label htmlFor="bat-path">実行するBAT</label>

        <input
          id="bat-path"
          type="text"
          placeholder="C:\CompanyTools\import-data.bat"
          value={batPath}
          disabled={isWatching}
          onChange={(event) => {
            setBatPath(event.target.value);
          }}
        />
      </section>

      <section className="actions">
        <button
          type="button"
          disabled={isWatching || !watchDirectory.trim() || !batPath.trim()}
          onClick={() => {
            void handleStart();
          }}
        >
          監視開始
        </button>

        <button
          type="button"
          disabled={!isWatching}
          onClick={() => {
            void handleStop();
          }}
        >
          監視停止
        </button>
      </section>

      <p>
        状態：
        <strong>{isWatching ? "監視中" : "停止中"}</strong>
      </p>

      <section>
        <h2>実行ログ</h2>

        {logs.length === 0 ? (
          <p>ログはありません。</p>
        ) : (
          <ul>
            {logs.map((log) => (
              <li key={log.id}>
                <p>
                  <strong>[{log.type}]</strong> {log.message}
                </p>

                <time>{new Date(log.dateTime).toLocaleString()}</time>

                {log.detail && <pre>{log.detail}</pre>}
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  );
}

export default App;

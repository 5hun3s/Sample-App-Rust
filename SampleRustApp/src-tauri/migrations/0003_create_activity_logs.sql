CREATE TABLE activity_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,

    window_title TEXT NOT NULL,
    process_id INTEGER NOT NULL,
    process_name TEXT,
    executable_path TEXT,
    window_class TEXT,

    window_x INTEGER,
    window_y INTEGER,
    window_width INTEGER,
    window_height INTEGER,

    is_maximized INTEGER NOT NULL DEFAULT 0,
    is_minimized INTEGER NOT NULL DEFAULT 0,

    idle_seconds INTEGER NOT NULL DEFAULT 0,
    recorded_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_activity_logs_recorded_at
ON activity_logs(recorded_at);

CREATE INDEX idx_activity_logs_process_name
ON activity_logs(process_name);
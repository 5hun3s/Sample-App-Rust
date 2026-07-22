ALTER TABLE activity_logs
ADD COLUMN started_at TEXT;

ALTER TABLE activity_logs
ADD COLUMN ended_at TEXT;

ALTER TABLE activity_logs
ADD COLUMN duration_seconds INTEGER NOT NULL DEFAULT 0;

UPDATE activity_logs
SET
    started_at = recorded_at,
    ended_at = recorded_at
WHERE started_at IS NULL;
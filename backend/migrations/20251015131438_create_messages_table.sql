CREATE TABLE messages (
    id TEXT PRIMARY KEY,              -- UUID
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,               -- 'user' or 'assistant'
    content TEXT NOT NULL,
    timestamp TEXT NOT NULL,          -- ISO 8601 timestamp
    FOREIGN KEY (session_id) REFERENCES sessions(id) ON DELETE CASCADE
);

# Aazan
Aazan is a full-stack Rust web app that uses a "learning by teaching" model. Users teach topics to an AI student, Bodhi, via text, PDF, or voice chat. This monolithic application is built with Axum and Dioxus from a single codebase. The name Aazan is derived from the Tamil word for "teacher" and the Hindi word for "easy."


# Aazan Development Progress

## ✅ Session Management API

We built a complete and robust backend API for managing teaching sessions. This included:

1. **Creating Sessions**: An endpoint to create a new session from raw text.
2. **Retrieving Sessions**: Endpoints to fetch a single session by its ID or list all existing sessions.
3. **Deleting Sessions**: An added endpoint to remove a session.
4. **Foundation**: We established the project structure, database models, and a clean separation between API handlers and database logic.

---

## ✅ PDF File Processing

We enhanced the application to accept study materials in a more versatile format. This involved:

1. **File Uploads**: A new endpoint that handles multipart/form-data for file uploads.
2. **PDF Text Extraction**: Integrated a library to parse uploaded PDF files and extract their text content directly in memory.
3. **Database Integration**: Connected the file upload logic to our existing session creation system, allowing users to start a session from a PDF.

---

### API Endpoints Implemented

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/api/session` | Create session from text or PDF |
| `GET` | `/api/sessions` | List all sessions |
| `GET` | `/api/session/{id}` | Retrieve specific session |
| `DELETE` | `/api/session/{id}` | Delete a session |

### Technical Stack

- **Backend Framework**: Axum (async Rust)
- **Database**: SQLite with sqlx
- **PDF Processing**: pdf-extract crate
- **File Handling**: Multipart form data parsing
- **Error Handling**: Custom error types with proper HTTP status codes

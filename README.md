# zkkodb – A Lightweight SQL-Inspired Database Engine in Rust

## Project Description

**zkkodb** is a lightweight, file-based, SQL-inspired relational database system implemented in **Rust**. It is built from scratch to handle structured data with features like schema validation, referential integrity, user management, transaction logic, and persistent storage – all without relying on external database engines. The goal is to implement the core mechanisms manually, including indexing, command parsing, and file-backed storage, using modern Rust practices like type safety, module-based architecture, and Serde-based JSON handling.

---

## Features & Design Decisions

### Storage
- Each **table** is stored in two separate files:
  - `table_name.schema.json` → contains column definitions and constraints
  - `table_name.data` → contains actual row data
- Each **user** is stored in its own file:
  - `users.schema.json` (fixed schema)
  - `users.data` (user rows, includes hashed passwords)
- All files are stored in a dedicated directory, e.g. `./data/zkkodb/`
- On startup, the engine scans the directory and loads all schemas and data files into memory

---

###  User Management

- Users are stored in a special table called `users`
- Fields: `id`, `username`, `password_hash`, `role`, `created_at`
- Supports authentication via hashed passwords
- Users can have roles (e.g. `admin`, `reader`, `writer`) with access control (planned)
- Example user creation command:
  ```json
  {
    "command": "create_user",
    "username": "alice",
    "password": "secret123",
    "role": "admin"
  }
  ```

---

### Schema & Validation

- When creating a table, its schema is defined and stored as JSON
- On `insert`, the engine validates:
  - Keys must match the schema
  - Data types must match (`INT`, `FLOAT`, `STRING`, `CHAR`)
  - `not_null` fields must be present
  - `default` values are inserted if data is missing
- Planned constraint support includes:
  - `not_null`
  - `default`
  - `unique`

---

### CRUD Operations

- All operations are parsed from JSON commands
- Supported commands:
  - `create_table`
  - `insert`
  - `select`
  - `update`
  - `delete`
  - `create_user` / `authenticate`
- Each command is parsed and validated against the corresponding schema

---

### Referential Integrity

- Foreign key references can be defined in the schema:
  ```json
  {
    "user_id": {
      "type": "INT",
      "references": "users.id",
      "not_null": true
    }
  }
  ```
- On deletion:
  - The system checks whether other tables reference the row
  - If so, it warns or blocks the operation depending on the chosen mode

---

### Indexing & Search

- Tables are sorted by primary key to allow efficient **binary search**
- Binary search is used for fast row lookup (`O(log n)`)
- Planned: A custom **hash map** structure for key-based indexing
  - Keys: e.g. `id`
  - Value: in-memory row pointer or file offset
  - Includes hash function and collision resolution

---

##  Implementation

- Entirely written in **Rust**
- Uses:
  - `serde` for JSON handling
  - `mmap` or buffered file IO for persistent storage
  - Strong typing for schema validation
  - Module-based separation of logic
- No SQL parser – all commands are defined as JSON objects

---

##  Example File Structure

```
data/
├── users.schema.json
├── users.data
├── posts.schema.json
├── posts.data
```

---

## Example Command

```json
{
  "command": "insert",
  "table": "users",
  "values": {
    "id": 1,
    "username": "admin",
    "password_hash": "abc123",
    "role": "admin"
  }
}
```

---

## Planned Modules

```rust
// src/
├── lib.rs            // Entry point
├── parser.rs         // Parses and dispatches JSON commands
├── schema.rs         // Schema definitions and validation
├── crud.rs           // Insert / Select / Update / Delete logic
├── index.rs          // Binary search and hash-based indexing
├── hashmap.rs        // Custom in-memory hashmap implementation
├── utils.rs          // File IO, JSON helpers, hashing etc.
├── user.rs           // Authentication and user-related logic
```
---

**zkkodb** is designed for full control, minimal dependencies, and educational value. It’s a great way to understand how databases actually work under the hood – with real files, real parsing, and real rules.


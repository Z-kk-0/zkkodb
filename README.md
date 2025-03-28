# zkkodb – A Lightweight SQL-Like Database Engine in C

## Project Description

**zkkodb** is a lightweight, file-based, SQL-inspired relational database system implemented in **C**. It is designed from scratch to handle structured data with features like schema validation, referential integrity, transaction logic, and persistent storage – all without relying on external database engines. The goal is to build a fully functioning database system, learning and implementing the core mechanisms manually, including indexing and parsing.

---

## Requirements & Design Decisions

### Storage
- Each table is stored in two separate files:
  - `table_name.schema.json` → contains column definitions and constraints
  - `table_name.data` → contains actual row data
- All files are stored in a dedicated directory, e.g. `/var/zkkodb`
- On startup, the engine scans the folder and loads all schemas and tables into memory

---

### Schema & Validation
- When creating a table, the column schema is defined and stored as JSON
- On `insert`, the engine validates:
  - That all keys match the schema
  - That the data types match (`INT`, `FLOAT`, `STRING`, `CHAR`)
  - That `not_null` fields are not null
  - That `default` values are applied if no data is provided
- Planned constraint support includes:
  - `not_null`
  - `default`
  - `unique`

---

### CRUD Operations
- All operations are parsed from JSON commands
- Supported operations:
  - `create_table`
  - `insert`
  - `select`
  - `update`
  - `delete`
- Each command is parsed and validated against the schema before being executed

---

### Referential Integrity
- Foreign key references are declared in the schema:
  ```json
  {
    "user_id": {
      "type": "INT",
      "references": "users.id",
      "not_null": true
    }
  }
  ```
- On delete:
  - The system checks which other tables reference the row
  - The user is warned before deletion and can confirm or cancel

---

### Searching & Indexing
- Tables are sorted by primary key (e.g. `id`) to allow efficient **binary search**
- Binary Search is used for lookups with logarithmic complexity (`O(log n)`)
- A custom **hash map** implementation is planned for faster access:
  - Keys: e.g. `id`
  - Value: pointer or index to the row in memory or file
  - Will include a hash function and collision resolution strategy

---

### Implementation Language
- Entire project is written in **C**
- No external SQL engine dependencies
- All logic (parsing, validation, search, storage, transactions) is implemented manually


## Functions 
the file structure would look something like this: 
src/
├── core/
│   ├── CMakeLists.txt
│   ├── crud.c
│   ├── hashmap.c
│   ├── index.c
│   ├── parser.c
│   ├── schema.c
│
├── headers/
│   ├── crud.h
│   ├── hashmap.h
│   ├── index.h
│   ├── parser.h
│   ├── schema.h
│
├── utils/
│   ├── CMakeLists.txt
│   ├── file_utils.c
│   ├── json_utils.c
│
├── main.c
CMakeLists.txt

So i will declare the Functions of the individual c files

### parser.c
```c

int parse_command(const char* json_str) {
    cJSON* root = cJSON_Parse(json_str);
    if (!root) {
        return 1; 
    }

    cJSON* command = cJSON_GetObjectItem(root, "command");
    if (!command || !cJSON_IsString(command)) {
        cJSON_Delete(root);
        return 2;
    }

    if (strcmp(command->valuestring, "insert") == 0) {
        // handle_insert(...)
    } else if (strcmp(command->valuestring, "create") == 0) {
        // handle_create(...)
    } else if (strcmp(command->valuestring, "delete") == 0) {
        // handle_delete(...)
    } else {
        cJSON_Delete(root);
        return 3;
    }

    cJSON_Delete(root);
    return 0;   
}


}

int validate_syntax(const char* json_str) {
    
}

```

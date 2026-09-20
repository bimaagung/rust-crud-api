# Rust CRUD API

A simple RESTful CRUD (Create, Read, Update, Delete) API for blog posts, built with **Rust**, **Actix Web**, **Diesel** and **PostgreSQL**.

The service exposes a `/api/posts` resource with full CRUD operations, JSON request/response bodies, connection pooling via `r2d2`, structured JSON error responses and request logging.

---

## Features

- Full CRUD over the `posts` table (`POST`, `GET`, `GET /{id}`, `PUT /{id}`, `DELETE /{id}`)
- Asynchronous HTTP server powered by Actix Web 4
- Type-safe SQL queries with Diesel ORM and compile-time schema checking
- Connection pooling with `diesel::r2d2` (shared through Actix `web::Data`)
- Diesel CLI managed SQL migrations
- Environment configuration through `.env` (`dotenvy`)
- Unified error handling with a custom `ApiError` type that implements `ResponseError`
- Structured logging with `env_logger` + Actix `Logger` middleware
- Partial updates (`PATCH`-like semantics) on `PUT /api/posts/{id}`

---

## Tech Stack

| Component    | Crate / Tool | Version  |
| ------------ | ------------ | -------- |
| Language     | Rust (edition 2024) | 1.85+ (developed with 1.98.1) |
| Web framework | `actix-web` | 4 |
| ORM / Query builder | `diesel` (features: `postgres`, `r2d2`) | 2.3.13 |
| Connection pool | `diesel::r2d2` (`r2d2` 0.8) | 0.8.10 |
| Serialization | `serde` (derive), `serde_json` | 1.0 |
| Env loading  | `dotenvy` | 0.15.7 |
| Logging      | `env_logger`, `log` | 0.11 / 0.4 |
| Database     | PostgreSQL | 12+ |
| Migration tool | `diesel_cli` (with `postgres` feature) | 2.3.13 |

---

## Project Structure

```
rust-crud/
├── .env                     # Local environment variables (DATABASE_URL)
├── Cargo.toml               # Package manifest and dependencies
├── diesel.toml              # Diesel CLI config (schema path, migrations dir)
├── migrations/
│   └── 2026-09-20-090825-0000_create_posts/
│       ├── up.sql           # CREATE TABLE posts (...)
│       └── down.sql         # DROP TABLE posts
└── src/
    ├── main.rs              # Entry point: dotenv, logger, pool, Actix server, routes
    ├── db.rs                # DbPool type alias and init_pool()
    ├── handlers.rs          # CRUD handler functions and route registration
    ├── models.rs            # Post, NewPost, UpdatePost structs
    ├── schema.rs            # Diesel generated schema (posts table)
    └── errors.rs            # ApiError enum + ResponseError implementation
```

---

## Requirements

- **Rust** 1.85 or newer (the crate uses the 2024 edition)
- **PostgreSQL** server running and reachable
- **Diesel CLI** with the PostgreSQL backend (only needed for migrations / schema regeneration)

```bash
rustc --version
cargo --version
psql --version          # PostgreSQL client
diesel --version        # Diesel CLI (optional, for migrations)
```

Install the Diesel CLI if it is missing:

```bash
cargo install diesel_cli --no-default-features --features postgres
```

---

## Getting Started

### 1. Clone the repository

```bash
git clone <your-repository-url> rust-crud
cd rust-crud
```

### 2. Configure the environment

Create (or edit) the `.env` file in the project root:

```dotenv
DATABASE_URL=postgres://<user>:<password>@<host>:<port>/<database>
```

Default value used by this project:

```dotenv
DATABASE_URL=postgres://postgres:123456@localhost/rust-crud
```

> **Note:** `.env` is not listed in `.gitignore`, so make sure you never commit real production credentials.

Additional (optional) environment variables:

| Variable     | Default   | Description |
| ------------ | --------- | ----------- |
| `DATABASE_URL` | –       | **Required.** PostgreSQL connection string. The app panics if it is not set. |
| `RUST_LOG`   | `info`    | Log level filter consumed by `env_logger`. Example: `RUST_LOG=debug` |

### 3. Create the database and run migrations

`diesel setup` creates the database (if it does not exist yet) and runs all pending migrations:

```bash
diesel setup
```

If the database already exists and you only need to apply migrations:

```bash
diesel migration run
```

To roll back the last migration:

```bash
diesel migration revert
```

To revert and re-apply the last migration:

```bash
diesel migration redo
```

To drop the database and re-run every migration from scratch (destructive):

```bash
diesel database reset
```

### 4. Run the server

```bash
cargo run
```

The server binds to **`127.0.0.1:8080`**:

```
INFO  rust_crud > Server running at http://127.0.0.1:8000
INFO  actix_server::builder > starting 1 worker(s)
INFO  actix_server::server > Tokio runtime found; starting in existing Tokio runtime
```

> **Heads-up:** the log message printed by `src/main.rs` mentions port `8000`, but the actual bind address is `127.0.0.1:8080`. Use port **8080** when calling the API.

For a production-style build:

```bash
cargo build --release
./target/release/rust-crud
```

---

## API Reference

Base URL: `http://127.0.0.1:8080`

| Method | Endpoint          | Description                     | Success |
| ------ | ----------------- | ------------------------------- | ------- |
| `POST` | `/api/posts`      | Create a new post               | `201 Created` |
| `GET`  | `/api/posts`      | List all posts                  | `200 OK` |
| `GET`  | `/api/posts/{id}` | Get a single post by id         | `200 OK` |
| `PUT`  | `/api/posts/{id}` | Update one or more post fields  | `200 OK` |
| `DELETE` | `/api/posts/{id}` | Delete a post by id           | `204 No Content` |

### Post resource

```json
{
  "id": 1,
  "title": "My first post",
  "body": "Hello from Actix Web + Diesel",
  "published": false
}
```

| Field       | Type    | Notes |
| ----------- | ------- | ----- |
| `id`        | integer | Generated by PostgreSQL (`SERIAL PRIMARY KEY`), read-only |
| `title`     | string  | Required (`VARCHAR NOT NULL`) |
| `body`      | string  | Required (`TEXT NOT NULL`) |
| `published` | boolean | Defaults to `false`, only changeable through `PUT` |

### Create a post

```bash
curl -X POST http://127.0.0.1:8080/api/posts \
  -H 'Content-Type: application/json' \
  -d '{"title":"My first post","body":"Hello from Actix Web + Diesel"}'
```

`201 Created`

```json
{
  "id": 1,
  "title": "My first post",
  "body": "Hello from Actix Web + Diesel",
  "published": false
}
```

### List all posts

```bash
curl http://127.0.0.1:8080/api/posts
```

`200 OK`

```json
[
  { "id": 1, "title": "My first post", "body": "Hello from Actix Web + Diesel", "published": false }
]
```

### Get a post by id

```bash
curl http://127.0.0.1:8080/api/posts/1
```

`200 OK` – returns the post object, or `404 Not Found` if the id does not exist:

```json
{ "error": "Not Found: Data not found" }
```

### Update a post

All fields are optional: only the fields present in the request body are written to the database.

```bash
curl -X PUT http://127.0.0.1:8080/api/posts/1 \
  -H 'Content-Type: application/json' \
  -d '{"title":"Updated title","published":true}'
```

`200 OK`

```json
{
  "id": 1,
  "title": "Updated title",
  "body": "Hello from Actix Web + Diesel",
  "published": true
}
```

### Delete a post

```bash
curl -i -X DELETE http://127.0.0.1:8080/api/posts/1
```

`204 No Content` on success, or `404 Not Found` when no row matched:

```json
{ "error": "Not Found: Post with id 1 not found" }
```

---

## Error Handling

Every handler returns `Result<HttpResponse, ApiError>`. `ApiError` lives in `src/errors.rs` and implements Actix's `ResponseError`, so failures are serialized as JSON:

```json
{ "error": "Not Found: Data not found" }
```

| `ApiError` variant  | HTTP status | When it happens |
| ------------------- | ----------- | --------------- |
| `NotFound(String)`  | `404` | Query returned no row (Diesel `NotFound`) or a delete matched zero rows |
| `DatabaseError(String)` | `500` | Any other Diesel error, or a connection pool failure |
| `InternalServerError` | `500` | The blocking task (`web::block`) itself failed |

Conversions are provided by `From<diesel::result::Error>` and `From<r2d2::PoolError>`.

---

## Database Migrations

Migrations live in `migrations/` and are managed by the Diesel CLI.

Create a new migration:

```bash
diesel migration generate <migration_name>
```

Then edit the generated `up.sql` / `down.sql`, apply it, and regenerate the Rust schema:

```bash
diesel migration run
diesel print-schema    # writes src/schema.rs based on diesel.toml
```

`diesel.toml` already points the generated schema to `src/schema.rs` and the migrations directory to `migrations`:

```toml
[print_schema]
file = "src/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId", "Clone"]

[migrations_directory]
dir = "migrations"
```

Existing migration:

```sql
-- migrations/2026-09-20-090825-0000_create_posts/up.sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false
);
```

---

## Development

```bash
cargo check        # fast type check
cargo build        # debug build
cargo test         # run tests (no tests are defined yet)
cargo clippy       # lints
cargo fmt          # formatting
RUST_LOG=debug cargo run   # verbose logging
```

### Known limitations

- `PUT /api/posts/{id}` with an empty body (`{}`) produces an empty changeset, which Diesel rejects; send at least one field.
- `POST /api/posts` does not accept `published`; new posts always start as `published: false`.
- Request bodies are not validated (`title` / `body` have no length constraints or validation layer).
- The listen address (`127.0.0.1:8080`) and the log message in `src/main.rs` (port `8000`) are inconsistent.
- There is no authentication/authorization, no pagination and no automated test suite.

---

## License

No license file is included in this repository. Add one (for example `MIT` or `Apache-2.0`) before distributing the project.

# Rust CRUD API

A clean, asynchronous RESTful CRUD (Create, Read, Update, Delete) API for blog posts, built with **Rust**, **Axum**, **SeaORM**, and **PostgreSQL**, following **Clean Architecture** best practices.

The service exposes a `/api/posts` resource with full CRUD operations, JSON request/response bodies, async connection pooling, unified error handling, and structured request tracing.

---

## Features

- **Clean Architecture**: Strict separation of concerns across Domain, Application, Infrastructure, and Presentation layers.
- **Asynchronous Web Framework**: Powered by **Axum 0.8** with Tokio runtime.
- **Async ORM**: Type-safe queries and ActiveModels with **SeaORM 2.0** over PostgreSQL (`sqlx`).
- **Full CRUD API**: Endpoints for creating, reading, updating, and deleting posts.
- **Partial Updates**: Patch-like semantics supported on `PUT /api/posts/{id}`.
- **Unified Error Handling**: Domain errors automatically mapped to consistent JSON HTTP responses (`{"error": "..."}`).
- **Structured Tracing**: Request tracing and diagnostic logging with `tracing`, `tracing-subscriber`, and `tower-http`.
- **Testability**: Decoupled domain repository interfaces allow thorough unit and integration testing without requiring a live database.

---

## Architecture Overview

Dependencies follow the Clean Architecture dependency rule (pointing inwards):

```
┌─────────────────────────────────────────────────────────┐
│                   Presentation Layer                    │
│   (Axum Handlers, Routes, HTTP DTOs, Error Mapping)     │
├─────────────────────────────────────────────────────────┤
│                   Application Layer                     │
│           (Use Cases / Services, App DTOs)              │
├─────────────────────────────────────────────────────────┤
│                      Domain Layer                       │
│    (Pure Entities, Repository Traits, Domain Errors)    │
└─────────────────────────────────────────────────────────┘
                            ▲
                            │ implements traits
┌───────────────────────────┴─────────────────────────────┐
│                  Infrastructure Layer                   │
│      (SeaORM Entities, Repositories, Connection Pool)   │
└─────────────────────────────────────────────────────────┘
```

- **Domain Layer (`src/domain/`)**: Innermost layer. Pure domain entities (`Post`), repository trait interfaces (`PostRepository`), and domain errors (`DomainError`). Zero dependencies on web or database frameworks.
- **Application Layer (`src/application/`)**: Application business workflows (`PostUseCase`), input validation, and use case DTOs (`CreatePostDto`, `UpdatePostDto`).
- **Infrastructure Layer (`src/infrastructure/`)**: External adapters. SeaORM entities (`entities::post`), database connection pool (`connection::init_pool`), and repository implementation (`SeaOrmPostRepository`).
- **Presentation Layer (`src/presentation/`)**: Web layer using Axum. Request handlers, router setup, HTTP request DTOs, and error mapping into HTTP status codes and JSON responses.
- **Composition Root (`src/main.rs`, `src/config.rs`)**: Application bootstrapping, environment configuration, database connection, dependency injection, and HTTP server binding.

---

## Tech Stack

| Component | Crate / Tool | Version |
| --------- | ------------ | ------- |
| Language | Rust (edition 2024) | 1.85+ |
| Web Framework | `axum` | 0.8.9 |
| Async Runtime | `tokio` | 1.x |
| Async ORM | `sea-orm` (features: `sqlx-postgres`, `runtime-tokio-rustls`, `macros`) | 2.0.3 |
| Serialization | `serde`, `serde_json` | 1.0 |
| Environment | `dotenvy` | 0.15.7 |
| Tracing & Logging | `tracing`, `tracing-subscriber`, `tower-http` | 0.1 / 0.3 / 0.6 |
| Database | PostgreSQL | 12+ |

---

## Project Structure

```
rust-crud/
├── .env.example                      # Example environment configuration
├── Cargo.toml                        # Project dependencies
├── migrations/                       # PostgreSQL migrations
│   └── 2026-09-20-090825-0000_create_posts/
│       ├── up.sql                    # CREATE TABLE posts (...)
│       └── down.sql                  # DROP TABLE posts
├── src/
│   ├── lib.rs                        # Library root exporting modules
│   ├── main.rs                       # Application entry point & Composition Root
│   ├── config.rs                     # Environment configuration loader
│   ├── domain/                       # Core business layer
│   │   ├── mod.rs
│   │   ├── entities.rs               # Pure Post entity
│   │   ├── repositories.rs           # PostRepository trait interface
│   │   └── errors.rs                 # DomainError enum
│   ├── application/                  # Use cases and orchestration
│   │   ├── mod.rs
│   │   ├── dtos.rs                   # CreatePostDto, UpdatePostDto
│   │   └── use_cases.rs              # PostUseCase implementation & unit tests
│   ├── infrastructure/               # External drivers and adapters
│   │   ├── mod.rs
│   │   └── database/
│   │       ├── mod.rs
│   │       ├── connection.rs         # SeaORM connection pool initialization
│   │       ├── entities/             # SeaORM ActiveModel / Model definitions
│   │       └── repositories/         # SeaOrmPostRepository implementation
│   └── presentation/                 # Presentation / HTTP interface
│       ├── mod.rs
│       └── http/
│           ├── mod.rs
│           ├── dtos.rs               # HTTP request payloads
│           ├── errors.rs             # Axum IntoResponse error mapper
│           ├── handlers.rs           # Axum request handlers
│           ├── routes.rs             # Axum Router definition
│           └── state.rs              # Axum AppState dependency injection
└── tests/
    └── api_tests.rs                  # End-to-end Axum API integration tests
```

---

## Getting Started

### 1. Configure the Environment

Copy `.env.example` to `.env` and adjust the variables for your setup:

```bash
cp .env.example .env
```

```dotenv
DATABASE_URL=postgres://<user>:<password>@<host>:<port>/<database>
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
RUST_LOG=rust_crud=info,tower_http=info
```

| Variable | Default | Description |
| -------- | ------- | ----------- |
| `DATABASE_URL` | – | **Required.** PostgreSQL connection string. |
| `SERVER_HOST` | `127.0.0.1` | IP address or host to bind to. |
| `SERVER_PORT` | `8080` | Port to listen on. |
| `RUST_LOG` | `rust_crud=info,tower_http=info` | Tracing / log level filter. |

### 2. Run Database Migrations

Ensure the `posts` table is created in your PostgreSQL database:

```sql
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT false
);
```

You can execute this via `psql`:

```bash
psql $DATABASE_URL -f migrations/2026-09-20-090825-0000_create_posts/up.sql
```

### 3. Run the Server

```bash
cargo run
```

The server binds to `http://127.0.0.1:8080`.

---

## API Reference

Base URL: `http://127.0.0.1:8080`

| Method | Endpoint | Description | Success Status |
| ------ | -------- | ----------- | -------------- |
| `POST` | `/api/posts` | Create a new post | `201 Created` |
| `GET` | `/api/posts` | List all posts | `200 OK` |
| `GET` | `/api/posts/{id}` | Get a single post by id | `200 OK` |
| `PUT` | `/api/posts/{id}` | Update one or more post fields | `200 OK` |
| `DELETE` | `/api/posts/{id}` | Delete a post by id | `204 No Content` |

### Create Post

```bash
curl -X POST http://127.0.0.1:8080/api/posts \
  -H 'Content-Type: application/json' \
  -d '{"title":"Clean Architecture in Rust","body":"Decoupling domain from infrastructure with Axum & SeaORM"}'
```

`201 Created`
```json
{
  "id": 1,
  "title": "Clean Architecture in Rust",
  "body": "Decoupling domain from infrastructure with Axum & SeaORM",
  "published": false
}
```

### List Posts

```bash
curl http://127.0.0.1:8080/api/posts
```

`200 OK`
```json
[
  {
    "id": 1,
    "title": "Clean Architecture in Rust",
    "body": "Decoupling domain from infrastructure with Axum & SeaORM",
    "published": false
  }
]
```

### Get Post by ID

```bash
curl http://127.0.0.1:8080/api/posts/1
```

`200 OK` (or `404 Not Found` if the id does not exist):
```json
{
  "error": "Not Found: Data not found"
}
```

### Update Post (Partial Update)

```bash
curl -X PUT http://127.0.0.1:8080/api/posts/1 \
  -H 'Content-Type: application/json' \
  -d '{"title":"Updated Title","published":true}'
```

`200 OK`
```json
{
  "id": 1,
  "title": "Updated Title",
  "body": "Decoupling domain from infrastructure with Axum & SeaORM",
  "published": true
}
```

### Delete Post

```bash
curl -i -X DELETE http://127.0.0.1:8080/api/posts/1
```

`204 No Content`

---

## Testing & Quality Checks

```bash
cargo test            # Run unit tests and API integration tests
cargo check           # Fast type checking
cargo clippy          # Linter
cargo fmt --check     # Code formatting check
```

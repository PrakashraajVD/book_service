# Book Service API

A RESTful book-management service built with Rust, Axum, PostgreSQL, and Toasty.

## Features

- Axum-based HTTP API
- PostgreSQL persistence through Toasty
- Database migrations managed by `toasty-cli`
- Book CRUD endpoints
- JSON request parsing and responses with Serde
- Request validation with Garde for book create and update payloads
- Pagination for book listings
- Structured JSON logging with `tracing`
- CORS, request IDs, request tracing, and body-size limits
- Environment-based configuration

## Project Structure

```text
rest_api_workspace/
├── crates/
│   └── book_service/
│       ├── src/
│       │   ├── app/
│       │   │   ├── book/       # Book routes, handlers, and payloads
│       │   │   └── shared/     # Shared utilities, including pagination
│       │   ├── bin/
│       │   │   ├── app.rs      # HTTP server entry point
│       │   │   └── migration.rs
│       │   ├── models/         # Toasty database models
│       │   ├── config.rs
│       │   ├── errors.rs
│       │   ├── routes.rs
│       │   └── state.rs
│       ├── toasty/             # Migration history, SQL, and schema snapshots
│       └── Toasty.toml
├── Cargo.toml
└── .justfile
```

## Prerequisites

- Rust toolchain
- PostgreSQL running locally, or Docker Desktop
- `just` command runner (optional)

## Configuration

The service is configured through environment variables. Store local values in
`crates/book_service/.env`.

Required variables:

| Variable                    | Purpose                            |
| --------------------------- | ---------------------------------- |
| `SERVER_PORT`               | HTTP server port                   |
| `SERVER_ALLOWED_ORIGINS`    | Allowed CORS origins               |
| `SERVER_ALLOWED_METHODS`    | Allowed HTTP methods               |
| `SERVER_ALLOWED_HEADERS`    | Allowed request headers            |
| `SERVER_DEFAULT_BODY_LIMIT` | Maximum request-body size in bytes |
| `DB_PROTOCOL`               | Database protocol                  |
| `DB_HOST`                   | Database host                      |
| `DB_PORT`                   | Database port                      |
| `DB_USER`                   | Database user                      |
| `DB_PASS`                   | Database password                  |
| `DB_NAME`                   | Database name                      |

> Never commit `.env` files, database passwords, connection URLs, API keys, or
> production hostnames.

## Local Database

Run PostgreSQL locally or use the project Compose configuration. Configure the
application with your untracked `.env` file before applying migrations or
starting the server.

## Database Migrations

Apply pending migrations:

```sh
just book migration apply
```

The `books` table contains:

| Column           | PostgreSQL type | Rust type           |
| ---------------- | --------------- | ------------------- |
| `id`             | `UUID`          | `uuid::Uuid`        |
| `created_at`     | `TIMESTAMPTZ`   | `jiff::Timestamp`   |
| `updated_at`     | `TIMESTAMPTZ`   | `jiff::Timestamp`   |
| `published_date` | `DATE`          | `jiff::civil::Date` |
| `status`         | `SMALLINT`      | `BookStatus`        |
| `title`          | `TEXT`          | `String`            |
| `description`    | `TEXT`          | `Option<String>`    |
| `image_url`      | `TEXT`          | `Option<String>`    |

## Running the Service

```sh
just book app
```

By default, the API listens on the port configured by `SERVER_PORT`.

## OpenAPI Docs

The API schema is generated from the Rust route annotations and can be regenerated with:

```sh
just book apidoc
```

The generated spec is stored in `crates/book_service/openapi.yaml` and the live service is published at:

- https://book-service-pxm7.onrender.com
- OpenAPI docs: https://book-service-pxm7.onrender.com/openapi.yaml

## API Endpoints

| Method   | Path             | Description           |
| -------- | ---------------- | --------------------- |
| `GET`    | `/livez`         | Health-check endpoint |
| `GET`    | `/v1/books`      | List books            |
| `POST`   | `/v1/books`      | Create a book         |
| `GET`    | `/v1/books/{id}` | Get a book by ID      |
| `PUT`    | `/v1/books/{id}` | Update a book         |
| `DELETE` | `/v1/books/{id}` | Delete a book         |

> Use `/v1/books` without a trailing slash.

### List Books

```http
GET /v1/books?page=1&per_page=10
```

- `page` defaults to `1`.
- `per_page` defaults to `10`.
- `per_page` is limited to `100`.

### Create a Book

```http
POST /v1/books
Content-Type: application/json
```

```json
{
  "title": "The Rust Programming Language",
  "description": "A guide to programming with Rust.",
  "image_url": "https://example.com/book.png",
  "published_date": "2026-09-02",
  "status": "pending"
}
```

Supported status values:

```text
pending
verified
```

#### Request Validation

`POST /v1/books` and `PUT /v1/books/{id}` validate their JSON body before a
handler writes to the database.

| Field            | Rule                                                              |
| ---------------- | ----------------------------------------------------------------- |
| `title`          | Required; 1 to 255 characters                                     |
| `image_url`      | Optional; when present, must be a valid URL                       |
| `description`    | Optional; no additional validation                                |
| `published_date` | Required and must be a valid date accepted by `jiff::civil::Date` |
| `status`         | Required and must be `pending` or `verified`                      |

Malformed JSON or values that cannot be deserialized return `400 Bad Request`.
Validation failures return `422 Unprocessable Entity` with one message per
invalid field:

```http
HTTP/1.1 422 Unprocessable Entity
Content-Type: application/json
```

```json
{
  "errors": {
    "title": "Must be at least 1 character long",
    "image_url": "Must be a valid URL"
  }
}
```

### Update a Book

```http
PUT /v1/books/{id}
Content-Type: application/json
```

```json
{
  "title": "The Rust Programming Language, Second Edition",
  "description": "An updated Rust programming guide.",
  "image_url": "https://example.com/book-v2.png",
  "published_date": "2026-09-03",
  "status": "verified"
}
```

### Delete a Book

```http
DELETE /v1/books/{id}
```

A successful deletion returns:

```http
204 No Content
```

## Development Commands

```sh
just check
just test
just lint
just build
```

## Observability

The service uses `tracing` and emits structured JSON logs.

Set `RUST_LOG` to control log verbosity:

```sh
RUST_LOG=trace just book app
```

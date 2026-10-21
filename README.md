# 🚀 Axum Postgres Diesel Boilerplate

[![Rust](https://img.shields.io/badge/Rust-%23DEA584.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Framework-Axum-blue?style=flat&logo=rust)](https://github.com/tokio-rs/axum)
[![Database-PostgreSQL](https://img.shields.io/badge/Database-PostgreSQL-336791?style=flat&logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Diesel](https://img.shields.io/badge/ORM-Diesel-4B8BBE?style=flat&logo=rust)](https://diesel.rs/)
[![Docker](https://img.shields.io/badge/Docker-ready-blue?style=flat&logo=docker&logoColor=white)](https://www.docker.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A clean and ready-to-use boilerplate for building high-performance asynchronous web services with **Axum** 🦀 and **PostgreSQL** 🐘. It gives you a solid starting point for Rust APIs with database integration and migrations powered by **Diesel**.

---

## ⚙️ Prerequisites

| Tool | Purpose | Install |
|------|---------|---------|
| 🧰 Rust & Cargo | Build and run the app | [rust-lang.org](https://www.rust-lang.org/tools/install) |
| 🐋 Docker | Run Postgres and containerized deployment | [docs.docker.com](https://docs.docker.com/get-docker/) |
| 🛠️ Diesel CLI | Manage database migrations | See below |

---

## 🧩 Install Diesel CLI

Install the Diesel command-line tool with PostgreSQL support:

```bash
# Install libpq-dev for Diesel Postgres
sudo apt-get update && sudo apt-get install -y libpq-dev
cargo install diesel_cli --no-default-features --features postgres
```

---

## 🐘 Run PostgreSQL Instance (via Docker)

Start a local PostgreSQL instance:

```bash
docker run -d \
  --name axum_postgres \
  -e POSTGRES_USER=darix \
  -e POSTGRES_PASSWORD=your_password \
  -e POSTGRES_DB=axum_postgres_diesel \
  -p 5432:5432 \
  postgres:latest
```

---

## 🔧 Configure Environment

Copy the example environment file:

```bash
cp .env.example .env
```

Then update `.env` to match your local setup:

```env
DATABASE_URL=postgres://darix:your_password@localhost:5432/axum_postgres_diesel
```

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://user:pass@localhost:5432/db` |

> 💡 Diesel CLI reads `DATABASE_URL` from your environment or `.env` file, so complete this step before running migrations.

---

## 🗃️ Run Database Migrations

Apply all Diesel migrations to set up your schema:

```bash
diesel migration run
```

Other useful commands:

| Command | Description |
|---------|-------------|
| `diesel migration generate <name>` | Create a new migration |
| `diesel migration run` | Apply pending migrations |
| `diesel migration revert` | Revert the last migration |
| `diesel migration redo` | Revert and re-apply the last migration |

---

## 🏎️ Run Application with Cargo

Once the database is up and migrated, start the server:

```bash
cargo run
```

Cargo will compile and launch your Axum server on `http://0.0.0.0:8080`, ready for local development and testing.

---

## 🐳 Run with Docker (without Docker Compose)

Build the image and pass the PostgreSQL connection URL as an environment variable:

> ⚠️ Inside a container, `localhost` refers to the container itself. Use `host.docker.internal` (Docker Desktop) or add `--add-host=host.docker.internal:host-gateway` (Linux) to reach Postgres running on your host.

```bash
docker build -t axum_postgres_app .

docker run -d -p 8080:8080 \
  --add-host=host.docker.internal:host-gateway \
  -e DATABASE_URL="postgres://darix:your_password@host.docker.internal:5432/axum_postgres_diesel" \
  --name axum_postgres_app \
  axum_postgres_app
```

---

## 🚢 Deploy with Docker Compose

Build and deploy the app together with its database 🧱🐳:

```bash
docker compose up -d --build
```

---

## 🧪 Run the Tests

Tests share one database, so run them sequentially:

```bash
cargo test -- --test-threads=1
```

<!-- ---

## 🌐 Access the API Docs

Open the interactive OpenAPI docs (Swagger UI) at:

👉 [http://0.0.0.0:8080/docs/](http://0.0.0.0:8080/docs/)

---

## 🧩 Example Preview Docs

![IMAGE](./img/image.png) -->
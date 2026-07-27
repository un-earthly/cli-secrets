# Project: cli-secrets

High-performance, zero-knowledge CLI and web dashboard to back up and sync local `.env` files and SSH configurations across machines.

- Binary Name: `cli-secrets`
- Stack: Rust (CLI + Axum Server) + Vue 3 / Svelte 5 + SQLx
## Objective
Build a high-performance environment variable and config sync tool using Rust for the CLI and Backend REST API, with a lightweight Vue 3 or Svelte 5 Web Dashboard.

## Repository Architecture (Cargo Workspace + SPA)
- `crates/cli`: Rust CLI tool (`clap`, `reqwest`, `aes-gcm`, `tokio`)
- `crates/server`: Rust Axum REST API server (`axum`, `sqlx`, `tokio`, `argon2`, `tower-http`)
- `crates/crypto`: Shared Rust crate for local zero-knowledge encryption algorithms
- `web/`: Vue 3 or Svelte 5 SPA frontend (Vite + Tailwind CSS)

## Core Tasks for Antigravity

### Task 1: Setup Workspace & Crypto Crate (`crates/crypto`)
- Implement PBKDF2/Argon2 key derivation from user master password.
- Implement AES-256-GCM encryption & decryption functions (`encrypt_bytes`, `decrypt_bytes`).
- Add comprehensive unit tests in Rust proving round-trip security.

### Task 2: Build Rust CLI (`crates/cli`)
- Implement recursive file scanner for `.env*` and `~/.ssh/config`.
- Build CLI commands:
  - `env-vault login`: Interactive token / Device Flow login.
  - `env-vault push`: Encrypts files locally and sends encrypted payloads to the backend.
  - `env-vault pull`: Downloads blobs from backend, prompts for password, decrypts, and writes to disk.

### Task 3: Build Axum Server (`crates/server`)
- Implement REST API endpoints:
  - `POST /api/auth/register` & `POST /api/auth/login`
  - `POST /api/vault/push` (stores encrypted payload + checksums)
  - `GET /api/vault/pull` (fetches encrypted payload)
  - `GET /api/vault/projects` (lists active project names and sync status)
- Setup SQLx with PostgreSQL / SQLite migrations.
- Enable CORS via `tower-http`.

### Task 4: Build Web Dashboard (`web/`)
- Setup Vite + Vue 3 / Svelte 5 + Tailwind CSS.
- Build clean dashboard showing project list, synced file paths, and last modified timestamps.
- Ensure 100/100 Lighthouse performance score.
<div align="center">
  <img src="https://rustacean.net/assets/cuddlyferris.svg" width="200" alt="ferris-logo" />
</div>

<br/>

<p align="center">
    A web server for handling RESTful API build with 
    <a href="https://github.com/tokio-rs/axum" target="_blank">axum</a> framework.
</p>

<div align="center">

[![Build status](https://github.com/tokio-rs/axum/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/tokio-rs/axum/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/axum)](https://crates.io/crates/axum)
[![Docs.rs](https://docs.rs/axum/badge.svg)](https://docs.rs/axum)

</div>

## Description

Axum Posts is a MPV web server that provides a RESTful API for managing posts. It allows users to create, read, update, and delete posts with authentication and authorization. The server is built using the Axum framework, which is designed for building web applications in Rust.

<br/>

## Installation

> Note: To run the server, you need to have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

Clone the repository and run the following command:

```bash
git clone https://github.com/AhmedEhab-SG/axum_posts.git
cd axum_posts
cargo run
```

Environment variables are required to run the server. You can create a `.env` file in the root directory of the project with the following content:

```env
IP=127.0.0.1
LOCAL=localhost
PORT=7878

DATABASE_URL=postgres://username:password@localhost:5432/axum_posts

JWT_ACCESS_TOKEN_SECRET=your_jwt_access_token_key
JWT_ACCESS_TOKEN_EXPIRES=300

JWT_REFRESH_TOKEN_SECRET=your_jwt_refresh_token_key
JWT_REFRESH_TOKEN_EXPIRES=25200
```

To migrate the database schema, you need to install the [sqlx-cli](https://crates.io/crates/sqlx-cli) tool:

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

<br/>

<!-- toc -->

## Token

There is some routes are required to provide a token in the `Authorization` header of your requests.

> The token is provided upon logging in.

```bash
curl -X POST http://localhost:7878/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{"email": "your_email", "password": "your_password"}'
```

> using the token in the `Authorization` header of your requests like an example for deleting a post:

```bash
curl -X DELETE http://localhost:7878/api/v1/posts/1 \
 -H "Authorization": "Bearer your_token"
```

<br/>

<!-- tocstop -->

## Documentation

> Note: some request body props: (\* means required), some routes require a token in the `Authorization` header and some require high role privileges like admin.

<details>
 <summary><b>Authentication</b></summary>

### POST /api/v1/auth/register

    - email\*: string, must be a valid email address
    - password\*: string, must be at least 8 characters long
    - name\*: string, must be at least 5 characters long

> response status: 201 Created

### POST /api/v1/auth/login

    - email\*: string, must be a valid email address
    - password\*: string, must be at least 8 characters long

- response headers:

```json
{
  "Authorization": "Bearer your_token",
  "Set-Cookie": "refresh_token=your_refresh_token; HttpOnly; Path=/; Max-Age=25200; SameSite=Lax"
}
```

- response body:

```json
{
  "message": "Login successful",
  "user": {
    "id": 1,
    "email": "your_email",
    "name": "your_name",
    "role": "user",
    "created_at": "2023-10-01T00:00:00Z",
    "updated_at": "2023-10-01T00:00:00Z"
  }
}
```

> response status: 200 OK

### DELETE /api/v1/auth/logout

![Token](https://img.shields.io/badge/Token-Required-blue.svg)

> response status: 200 OK

### Get /api/v1/auth/refresh

![Token](https://img.shields.io/badge/Token-Required-blue.svg)

- response headers:

```json
{
  "Authorization: "Bearer your_token",
}
```

> response status: 200 OK

</details>

---

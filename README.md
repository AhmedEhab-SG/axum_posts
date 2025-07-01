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

## Features and Endpoints

> Note: some request body props: (\* means required), some routes require a token in the `Authorization` header and some require high role privileges like `admin` or `mod`.

<details>
 <summary><b>Authentication:</b></summary>

### POST /api/v1/auth/register

- request body:

```
  - email\*: string, must be a valid email address
  - password\*: string, must be at least 8 characters long
  - name\*: string, must be at least 5 characters long
```

> response status: 201 Created

<br/>

### POST /api/v1/auth/login

- request body:

```
  - email\*: string, must be a valid email address
  - password\*: string, must be at least 8 characters long
```

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

<br/>

### DELETE /api/v1/auth/logout

![Token](https://img.shields.io/badge/Token-Required-blue.svg)

> response status: 200 OK

<br/>

### Get /api/v1/auth/refresh

![Token](https://img.shields.io/badge/Token-Required-blue.svg)

- response headers:

```json
{
  "Authorization": "Bearer your_token"
}
```

> response status: 200 OK

</details>

---

<details>
 <summary><b>Users:</b></summary>

### GET /api/v1/users?page=1&limit=10

![Token](https://img.shields.io/badge/Token-Required-blue.svg)
![Role](https://img.shields.io/badge/Role-Required-red.svg)

> query takes page and limit default is 1 and 10 respectively

- response body:

```json
{
  "limit": 10,
  "page": 1,
  "total": 1,
  "users": [
    {
      "created_at": "2025-06-23T23:50:09.085567Z",
      "email": "user@gmail.com",
      "id": "35b37ee5-78e9-473c-a89a-81724ae48b30",
      "name": "Name User",
      "role": "User",
      "updated_at": "2025-06-23T23:50:09.085567Z"
    },
    {
      "created_at": "2025-06-23T23:50:09.085567Z",
      "email": "admin@gmail.com",
      "id": "35b37ee5-78e9-473c-a89a-81724ae48b30",
      "name": "Name Admin",
      "role": "Admin",
      "updated_at": "2025-06-23T23:50:09.085567Z"
    }
  ]
}
```

> response status: 200 OK

<br/>

### GET /api/v1/users/:id

- reponse body:

```json
{
  "user": {
    "created_at": "2025-06-23T23:50:09.085567Z",
    "email": "goblo@gmail.com",
    "id": "35b37ee5-78e9-473c-a89a-81724ae48b30",
    "name": "Goblo",
    "role": "Admin",
    "updated_at": "2025-06-23T23:50:09.085567Z"
  }
}
```

> response status: 200 OK

<br/>

### PATCH /api/v1/users/:id

![Token](https://img.shields.io/badge/Token-Required-blue.svg)
![Roles](https://img.shields.io/badge/Roles-Access-purple.svg)

```
  - password: string, must be at least 8 characters long
  - name: string, must be at least 5 characters long
```

- request body:

```json
{
  "email": "your_email",
  "name": "your_name"
}
```

> response status: 204 No Content

<br/>

### PUT /api/v1/users/role/:id

![Token](https://img.shields.io/badge/Token-Required-blue.svg)
![Role](https://img.shields.io/badge/Role-Requried-red.svg)

```
  - role\*: string, must be one of the following: "user", "admin", "mod"
```

- request body:

```json
{
  "role": "admin"
}
```

> response status: 204 No Content

<br/>

### DELETE /api/v1/users/:id

![Token](https://img.shields.io/badge/Token-Required-blue.svg)
![Roles](https://img.shields.io/badge/Roles-Access-purple.svg)

> response status: 204 No Content

</details>

---

<details>
 <summary><b>Posts:</b></summary>

### GET /api/v1/posts?page=1&limit=10

> query takes page and limit default is 1 and 10 respectively

- response body:

```json
{
  "limit": 10,
  "page": 1,
  "total": 2,
  "posts": [
    {
      "id": 1,
      "title": "Post Title",
      "body": "Post content goes here.",
      "user_id": 1,
      "created_at": "2023-10-01T00:00:00Z",
      "updated_at": "2023-10-01T00:00:00Z"
    },
    {
      "id": 2,
      "title": "Another Post Title",
      "body": "More content goes here.",
      "user_id": 2,
      "created_at": "2023-10-01T00:00:00Z",
      "updated_at": "2023-10-01T00:00:00Z"
    }
  ]
}
```

> response status: 200 OK

<br/>

### GET /api/v1/posts/:id

- response body:

```json
{
  "post": {
    "id": 1,
    "title": "Post Title",
    "body": "Post content goes here.",
    "user_id": 1,
    "created_at": "2023-10-01T00:00:00Z",
    "updated_at": "2023-10-01T00:00:00Z"
  }
}
```

> response status: 200 OK

<br/>

### GET /api/v1/posts/user/:user_id

- response body:

```json
{
  "form_user_id": "35b37ee5-78e9-473c-a89a-81724ae48b30",
  "limit": 10,
  "page": 1,
  "posts": [
    {
      "body": "the massive expansion of object worlds in the social world and to the rise of work and leisure environments that promote and demand relations with objects",
      "created_at": "2025-07-01T12:30:54.137064Z",
      "id": "230b03b2-7bd1-44b1-b820-c41f4dc94507",
      "title": "Your Assume Post 2",
      "updated_at": "2025-07-01T12:30:54.137064Z",
      "user_id": "35b37ee5-78e9-473c-a89a-81724ae48b30"
    },
    {
      "body": "the massive expansion of object worlds in the social world and to the rise of work and leisure environments that promote and demand relations with objects",
      "created_at": "2025-07-01T12:27:45.373714Z",
      "id": "de3c7d1d-363f-4273-b4ed-3e212f5c3436",
      "title": "Your Assume Post",
      "updated_at": "2025-07-01T12:27:45.373714Z",
      "user_id": "35b37ee5-78e9-473c-a89a-81724ae48b30"
    }
  ],
  "total": 2
}
```

> response status: 200 OK

<br/>

### POST /api/v1/posts

![Token](https://img.shields.io/badge/Token-Required-blue.svg)

- request body:

```json
{
  "title": "Post Title",
  "body": "Post content goes here."
}
```

> response status: 201 Created

<br/>

### PATCH /api/v1/posts/:id

![Token](https://img.shields.io/badge/Token-Required-blue.svg)
![Roles](https://img.shields.io/badge/Roles-Access-purple.svg)

```
  - title: string, must be at least 5 characters long
  - body: string, must be at least 20 characters long
```

- request body:

```json
{
  "title": "Updated Post Title",
  "body": "Updated post content goes here."
}
```

> response status: 204 No Content

<br/>

### DELETE /api/v1/posts/:id

![Token](https://img.shields.io/badge/Token-Required-blue.svg)
![Roles](https://img.shields.io/badge/Roles-Access-purple.svg)

> response status: 204 No Content

</details>

---

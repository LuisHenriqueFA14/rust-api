<h1 align="center">rust-api</h1>
<p align="center">A simple API made with Rust!</p>

## Table of content

* [Overview](##overview)
* [Stack](##stack)
* [Usage](##usage)
* [API](##api)
  * [Create users](###create-users)
  * [List users](###list-users)
  * [List user info](###list-user-info)
  * [Login user](###login-user)
  * [Update user](###update-user)
  * [Delete user](###delete-user)


## Overview

Rust API is an API created for my studies on rust and REST APIs.

This api stores a user creation system, as well as a login system with authentication and validation of parameters, functions such as listing users, editing and deleting them.

## Stack

- [actix-web](https://crates.io/crates/actix-web)
- [diesel](https://crates.io/crates/diesel)
- [jsonwebtoken](https://crates.io/crates/jsonwebtoken)
- [bcrypt](https://crates.io/crates/bcrypt)
- [UUID](https://crates.io/crates/uuid)
- [regex](https://crates.io/crates/regex)

## Usage

After downloading the source code, run the following command to download the dependencies and build the project:
```
cargo build
```

Then set these environment variables in .env file:
```
HOST="127.0.0.1"
PORT=3000
DATABASE_URL=
JWT_SECRET=
```

Then run the following command to run the server:
```
cargo run
```

## API

### Create users

POST `/users`

Fields:
```json
{
    "username": "[USERNAME]",
    "email": "[EMAIL]",
    "password": "[PASSWORD]"
}
```

Response:
```json
{
    "id": "[ID]",
    "username": "[USERNAME]",
    "email": "[EMAIL]"
}
```

### List users

GET `/users`

Response:
```json
[
    {
        "id": "[ID]",
        "username": "[USERNAME]",
    },
    ...
]
```

### List user info

GET `/users/{ID}`

Response:
```json
{
    "id": "[ID]",
    "username": "[USERNAME]",
    "email": "[EMAIL]"
}
```

### Login user

POST `/login`

Fields:
```json
{
    "email": "[EMAIL]",
    "password": "[PASSWORD]"
}
```

Response:
```json
{
    "token": "[TOKEN]"
}
```

### Update user

PUT `/users`
**Requires authentication**

Fields:
```json
{
    "username": "[USERNAME]",
    "email": "[EMAIL]",
    "password": "[PASSWORD]"
}
```

Response:
```json
{
    "message": "User updated successfully!"
}
```

### Delete user

DELETE `/users`
**Requires authentication**

Fields:
```json
{
    "password": "[PASSWORD]"
}
```

Response:
```json
{
    "message": "User deleted successfully!"
}
```

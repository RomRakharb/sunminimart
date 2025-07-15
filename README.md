# Sunminimart / ซันมินิมาร์ท

Sunminimart is my first real project and my first attemp at learning [`iced`](https://iced.rs/), a rust GUI library.
The project's purpose is to be used in "Sunminimart", a shop owned by my sister (formerly by my mother).

## Features

- Manage inventory
- Sale frontend
- Ledger

## usage

1. set environment variables.

```
  MYSQL_ROOT_PASSWORD=root_password
  MYSQL_DATABASE=database
  MYSQL_USER=user
  MYSQL_PASSWORD=user_password

  DATABASE_URL=mysql://user:user_password@localhost:3306/database
  OLD_DATABASE_URL=mysql://old_user:old_user_password@old_ip:3306/database
```

2. get tools

- sqlx-cli
- bacon

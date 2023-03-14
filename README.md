# hands-on-backend-rs

actix-web + diesel で web backend を実装したやつです。

## Requirements

- [Rust](https://www.rust-lang.org/ja/tools/install)
- [Docker](https://www.docker.com/)

## Setup

### diesel_cli

diesel_cli は libpq に依存しているので適宜インストールを行なってください。

```shell
$ brew install libpq             # for mac users
$ sudo apt-get install -y libpq-dev  # for ubuntu users
```

diesel_cli のインストール

```shell
$ exec $SHELL -l
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel --version
```

### db initialization

```shell
$ make rund
$ make db/seed
```

## test

```shell
$ curl http://localhost:8080/health -i
HTTP/1.1 200 OK
content-length: 2
date: Mon, 13 Mar 2023 13:52:21 GMT

ok   
```

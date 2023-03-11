# hands-on-backend-rs

actix-web + diesel で web backend を実装したやつです。

## Requirements

- [Rust](https://www.rust-lang.org/ja/tools/install)
- [Docker](https://www.docker.com/)

## Setup

**direnv**

direnv があると環境変数のロードが非常に楽になるので使いましょう。

```shell
$ brew install direnv             # for mac users
$ sudo apt-get install -y direnv  # for ubuntu users
$ exec $SHELL -l
```

`.envrc.sample` から `.envrc` を生成し、ロードしてください。

```shell
$ cp .envrc.sample .envrc
$ direnv allow
direnv: loading ~/Workspace/hands-on-backend-rs/.envrc                         
direnv: export +DATABASE_URL
$ echo $DATABASE_URL
postgres://szpp:szpp3776@localhost:5432/szpp-mini-judge
```

**diesel_cli**

diesel_cli は libpq に依存しているので適宜インストールを行なってください。

```shell
$ brew install libpq             # for mac users
$ sudo apt-get install -y libpq  # for ubuntu users
```

diesel_cli のインストール

```shell
$ cargo install diesel_cli --no-default-features --features postgres
```

## build & run

```shell
$ docker compose up -d --build
```

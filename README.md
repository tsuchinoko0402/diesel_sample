# Diesel サンプル

- Rust の ORM である [Diesel](https://diesel.rs/) のサンプル

## 必要要件

- Docker
- docker-composed
- Rust
- Diesel

## 使い方

### DB のビルド

```term
docker-composed build
```

### DB の起動

```term
docker-composed up -d
```

### DB の終了

```term
docker-composed down
```

### サンプルアプリの起動

DB が起動している状態で。

```term
cd app
cargo run
```
